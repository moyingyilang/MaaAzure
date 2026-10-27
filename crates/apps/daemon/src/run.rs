//! 真机执行：连 ADB → 加载资源包 → 跑一条 Pipeline。
//!
//! 这是 P0 的最后一段路：**代码这一侧已经完整**，只差一台跑着游戏的模拟器来验收
//! （`docs/6-plan/6.1-roadmap.md` 的 P0 第 5 项）。

use std::path::PathBuf;

use maazure_core::TaskOutcome;
use maazure_runner::ffi::MaaEngine;

/// 一条命令行里能出现的参数。
struct Args {
    task: String,
    adb: String,
    address: String,
    server: String,
    resource_root: PathBuf,
    bundle: Option<String>,
    override_json: String,
}

fn parse(args: &[String]) -> Result<Args, String> {
    let mut task = None;
    let mut adb = "adb".to_string();
    let mut address = None;
    let mut server = "cn".to_string();
    let mut resource_root = PathBuf::from("resource");
    let mut bundle = None;
    let mut override_json = "{}".to_string();

    let mut it = args.iter();
    while let Some(a) = it.next() {
        let mut next = |what: &str| -> Result<String, String> {
            it.next().cloned().ok_or_else(|| format!("{what} 缺参数"))
        };
        match a.as_str() {
            "--adb" => adb = next("--adb")?,
            "--address" => address = Some(next("--address")?),
            "--server" => server = next("--server")?,
            "--resource" => resource_root = PathBuf::from(next("--resource")?),
            "--bundle" => bundle = Some(next("--bundle")?),
            "--override" => override_json = next("--override")?,
            other if other.starts_with("--") => return Err(format!("未知参数 {other}")),
            other => task = Some(other.to_string()),
        }
    }

    Ok(Args {
        task: task.ok_or("缺少任务名（如 CommissionStart）")?,
        adb,
        address: address.ok_or("缺少 --address（如 127.0.0.1:16384）")?,
        server,
        resource_root,
        bundle,
        override_json,
    })
}

/// 执行。
pub fn run_task(args: &[String]) -> std::process::ExitCode {
    let a = match parse(args) {
        Ok(a) => a,
        Err(e) => {
            eprintln!("--run 参数错误: {e}");
            return std::process::ExitCode::from(2);
        }
    };

    println!("── MaaAzure run ──");
    println!("任务      {}", a.task);
    println!("设备      {} @ {}", a.adb, a.address);
    println!("服务器    {}", a.server);

    let engine = match MaaEngine::connect_adb(&a.adb, &a.address, "{}") {
        Ok(e) => e,
        Err(e) => {
            eprintln!("连接失败: {e}");
            return std::process::ExitCode::FAILURE;
        }
    };
    let (w, h) = engine.resolution();
    println!("分辨率    {w}x{h}（MaaFW 归一化后）");

    // 运行时语义：先 common，再服务器包，后者覆盖前者
    let common = a.resource_root.join("common");
    let server = match &a.bundle {
        Some(b) => a.resource_root.join(b),
        None => a.resource_root.join(&a.server),
    };
    let mut bundles: Vec<&str> = Vec::new();
    if common.is_dir() {
        if let Some(s) = common.to_str() {
            bundles.push(s);
        }
    }
    match server.to_str() {
        Some(s) if server.is_dir() => bundles.push(s),
        _ => {
            eprintln!("资源包不存在: {}", server.display());
            return std::process::ExitCode::FAILURE;
        }
    }

    if let Err(e) = engine.load_bundles(&bundles) {
        eprintln!("加载资源失败: {e}");
        return std::process::ExitCode::FAILURE;
    }
    println!("资源包    {}", bundles.join(", "));

    // 业务自定义识别/动作：Pipeline 里的 "recognition": "Custom" 节点靠这里生效
    if let Err(e) =
        engine.register_recognition("CommissionPick", Box::new(maazure_runner::CommissionPick))
    {
        eprintln!("注册自定义识别失败: {e}");
        return std::process::ExitCode::FAILURE;
    }
    println!("自定义识别  CommissionPick");

    match engine.run_pipeline(&a.task, &a.override_json) {
        Ok(TaskOutcome::Ok) => {
            println!("结果      成功");
            std::process::ExitCode::SUCCESS
        }
        Ok(other) => {
            println!("结果      {:?}", other);
            std::process::ExitCode::FAILURE
        }
        Err(e) => {
            eprintln!("执行失败: {e}");
            std::process::ExitCode::FAILURE
        }
    }
}
