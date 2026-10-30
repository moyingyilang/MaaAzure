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
    address: Option<String>,
    server: String,
    resource_root: PathBuf,
    bundle: Option<String>,
    override_json: String,
    /// Windows 窗口句柄（`--win32 <hwnd>`）
    win32: Option<usize>,
    /// Android display id（`--display <id>`，虚拟屏/叠加视图）
    display: Option<String>,
    /// 只读这个包的浮窗（`--game-window <package>`）
    game_window: Option<String>,
    /// Win32 截图方式，默认 background（后台截窗口，不受其他窗口干扰）
    screencap: String,
    /// Win32 输入方式，默认 post（PostMessage，不抢鼠标）
    input: String,
}

fn parse(args: &[String]) -> Result<Args, String> {
    let mut task = None;
    let mut adb = "adb".to_string();
    let mut address = None;
    let mut win32 = None;
    let mut display = None;
    let mut game_window = None;
    let mut screencap = "background".to_string();
    let mut input = "post".to_string();
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
            "--win32" => {
                win32 = Some(
                    next("--win32")?
                        .parse()
                        .map_err(|_| "--win32 需要窗口句柄（十进制）".to_string())?,
                )
            }
            "--display" => display = Some(next("--display")?),
            "--game-window" => game_window = Some(next("--game-window")?),
            "--screencap" => screencap = next("--screencap")?,
            "--input" => input = next("--input")?,
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
        address,
        win32,
        display,
        game_window,
        screencap,
        input,
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
    println!("服务器    {}", a.server);

    let engine = if let Some(package) = a.game_window.clone() {
        println!("游戏窗口  {package}（整屏截图 → 裁窗口 → 1280x720）");
        match MaaEngine::from_game_window(&package, true) {
            Ok(e) => e,
            Err(e) => {
                eprintln!("连接失败: {e}");
                return std::process::ExitCode::FAILURE;
            }
        }
    } else if let Some(display) = a.display.clone() {
        println!("虚拟屏    display={display}（screencap -d / input -d）");
        let cmds = maazure_runner::android_display::DisplayCommands::new(display).with_su();
        match MaaEngine::from_display_commands(cmds) {
            Ok(e) => e,
            Err(e) => {
                eprintln!("连接失败: {e}");
                return std::process::ExitCode::FAILURE;
            }
        }
    } else if let Some(hwnd) = a.win32 {
        println!("窗口      hwnd={hwnd}（截图 {} / 输入 {}）", a.screencap, a.input);
        #[cfg(target_os = "windows")]
        {
            match MaaEngine::connect_win32(hwnd, &a.screencap, &a.input) {
                Ok(e) => e,
                Err(e) => {
                    eprintln!("连接失败: {e}");
                    return std::process::ExitCode::FAILURE;
                }
            }
        }
        #[cfg(not(target_os = "windows"))]
        {
            eprintln!("--win32 只能在 Windows 目标上使用（当前二进制不是 Windows 版）");
            return std::process::ExitCode::FAILURE;
        }
    } else {
        let Some(address) = a.address.clone() else {
            eprintln!("需要 --address（ADB）/ --win32（窗口句柄）/ --display（Android 虚拟屏）/ --game-window（浮窗包名）");
            return std::process::ExitCode::from(2);
        };
        println!("设备      {} @ {}", a.adb, address);
        match MaaEngine::connect_adb(&a.adb, &address, "{}") {
            Ok(e) => e,
            Err(e) => {
                eprintln!("连接失败: {e}");
                return std::process::ExitCode::FAILURE;
            }
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
