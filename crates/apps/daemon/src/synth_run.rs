//! `--synth`：用合成帧跑真实 Pipeline，不需要模拟器。
//!
//! ```bash
//! # 让 commission_goto_reward 命中，其余都不命中
//! maazure-daemon --synth CommissionStart --show commission_goto_reward
//!
//! # 让 global_home 命中（验证锚点跳转）
//! maazure-daemon --synth CommissionStart --show global_home
//! ```
//!
//! 原理见 `maazure_runner::synth`；这层只负责拼参数、装配、打印结果。

use std::path::PathBuf;

use maazure_core::TaskOutcome;
use maazure_runner::ffi::MaaEngine;
use maazure_runner::synth::{Canvas, SynthController};

struct Args {
    entry: String,
    show: Vec<String>,
    server: String,
    resource_root: PathBuf,
    frames: usize,
    max_wait: u64,
}

fn parse(args: &[String]) -> Result<Args, String> {
    let mut entry = None;
    let mut show = Vec::new();
    let mut server = "cn".to_string();
    let mut resource_root = PathBuf::from("resource");
    let mut frames = 2usize;
    let mut max_wait = 30u64;

    let mut it = args.iter();
    while let Some(a) = it.next() {
        let mut next = |what: &str| -> Result<String, String> {
            it.next().cloned().ok_or_else(|| format!("{what} 缺参数"))
        };
        match a.as_str() {
            "--show" => show.extend(
                next("--show")?
                    .split(',')
                    .filter(|s| !s.is_empty())
                    .map(str::to_string),
            ),
            "--server" => server = next("--server")?,
            "--resource" => resource_root = PathBuf::from(next("--resource")?),
            "--frames" => {
                frames = next("--frames")?
                    .parse()
                    .map_err(|_| "--frames 需要数字".to_string())?
            }
            "--wait" => {
                max_wait = next("--wait")?
                    .parse()
                    .map_err(|_| "--wait 需要秒数".to_string())?
            }
            other if other.starts_with("--") => return Err(format!("未知参数 {other}")),
            other => entry = Some(other.to_string()),
        }
    }

    Ok(Args {
        entry: entry.ok_or("缺少 Pipeline 入口节点（如 CommissionStart）")?,
        show,
        server,
        resource_root,
        frames: frames.max(1),
        max_wait,
    })
}

/// 执行。
pub fn run_synth(args: &[String]) -> std::process::ExitCode {
    let a = match parse(args) {
        Ok(a) => a,
        Err(e) => {
            eprintln!("--synth 参数错误: {e}");
            return std::process::ExitCode::from(2);
        }
    };

    let common = a.resource_root.join("common");
    let server = a.resource_root.join(&a.server);
    let mut bundles: Vec<PathBuf> = Vec::new();
    if common.is_dir() {
        bundles.push(common);
    }
    if !server.is_dir() {
        eprintln!("资源包不存在: {}", server.display());
        return std::process::ExitCode::FAILURE;
    }
    bundles.push(server);

    println!("── MaaAzure synth ──");
    println!("入口      {}", a.entry);
    println!("资源包    {:?}", bundles);
    println!("贴图      {:?}", a.show);

    // 合成帧：底板 + 逐个节点贴图；多帧时重复（让 click 能推进）
    let mut canvas = Canvas::new();
    for node in &a.show {
        if let Err(e) = canvas.paste_node(&bundles, node) {
            eprintln!("贴图失败: {e}");
            return std::process::ExitCode::FAILURE;
        }
        println!("  已贴 {node}");
    }
    let frame = canvas.to_frame();
    let frames = vec![frame; a.frames];
    let ctrl = SynthController::new(frames);

    let engine = match MaaEngine::from_synth(ctrl.clone()) {
        Ok(e) => e,
        Err(e) => {
            eprintln!("装配引擎失败: {e}");
            return std::process::ExitCode::FAILURE;
        }
    };
    let paths: Vec<&str> = bundles.iter().filter_map(|p| p.to_str()).collect();
    if let Err(e) = engine.load_bundles(&paths) {
        eprintln!("加载资源失败: {e}");
        return std::process::ExitCode::FAILURE;
    }
    if let Err(e) =
        engine.register_recognition("CommissionPick", Box::new(maazure_runner::CommissionPick))
    {
        eprintln!("注册自定义识别失败: {e}");
        return std::process::ExitCode::FAILURE;
    }

    let started = std::time::Instant::now();
    let result = engine.run_pipeline(&a.entry, "{}");
    let elapsed = started.elapsed();

    println!("── 结果 ──");
    println!(
        "耗时      {:.1}s（上限 {}s）",
        elapsed.as_secs_f64(),
        a.max_wait
    );
    println!("点击      {:?}", ctrl.clicks());
    println!("滑动      {:?}", ctrl.swipes());
    println!("帧号      {}", ctrl.cursor());

    match result {
        Ok(TaskOutcome::Ok) => {
            println!("Pipeline  成功");
            std::process::ExitCode::SUCCESS
        }
        Ok(other) => {
            println!("Pipeline  {:?}", other);
            std::process::ExitCode::FAILURE
        }
        Err(e) => {
            eprintln!("执行失败: {e}");
            std::process::ExitCode::FAILURE
        }
    }
}
