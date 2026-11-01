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

    if let Err(e) = maazure_runner::ffi::init_framework() {
        eprintln!("初始化 MaaFramework 失败: {e}");
        return std::process::ExitCode::FAILURE;
    }

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
        println!(
            "窗口      hwnd={hwnd}（截图 {} / 输入 {}）",
            a.screencap, a.input
        );
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
            eprintln!(
                "需要 --address（ADB）/ --win32（窗口句柄）/ --display（Android 虚拟屏）/ --game-window（浮窗包名）"
            );
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

    let outcome = engine.run_pipeline(&a.task, &a.override_json);
    report_next_run(&a.task, outcome.is_ok());
    match outcome {
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


/// B5 调度回报：跑完后把「下次该什么时候回来」写进配置。
///
/// 对应上游 `config.task_delay(target=future_finish)`：
/// - 扫描结果里有已知完成时刻 → 写最早的
/// - 没有（或任务失败）→ 删除 next_run，交给调度器按失败重试
///
/// 只对 Commission 生效；其它任务还未接扫描数据。
fn report_next_run(task: &str, ok: bool) {
    if task != "Commission" {
        return;
    }
    let path = std::env::var("MAAGUI_CONFIG").unwrap_or_else(|_| "config/alas.json".to_string());
    let mut cfg = match maazure_config::Config::load(&path) {
        Ok(c) => c,
        Err(e) => {
            println!("调度回报  跳过（{e}）");
            return;
        }
    };
    let wake = if ok {
        maazure_runner::commission_scan::next_wakeup_secs(
            &maazure_runner::maa_custom::commission_scan_entries(),
        )
    } else {
        None
    };
    match wake {
        Some(secs) => {
            let at = eta_string(secs);
            cfg.set_next_run("Commission", Some(at.clone()));
            match cfg.save(&path) {
                Ok(()) => println!("调度回报  Commission next_run = {at}（{secs}s 后）"),
                Err(e) => println!("调度回报  写入失败: {e}"),
            }
        }
        None => {
            cfg.set_next_run("Commission", None);
            let _ = cfg.save(&path);
            println!("调度回报  Commission 无已知完成时刻 → 按失败重试");
        }
    }
}

/// 把「多少秒后」格式化成 `YYYY-MM-DD HH:MM:SS`（本地时间用 UTC 近似，够调度比较）。
fn eta_string(secs: u64) -> String {
    let now = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|d| d.as_secs())
        .unwrap_or(0);
    let t = now + secs;
    let (days, rem) = (t / 86_400, t % 86_400);
    let (hh, mm, ss) = (rem / 3600, (rem % 3600) / 60, rem % 60);
    // 从 1970-01-01 起算的朴素日期（不考虑闰秒；用于调度足够）
    let mut y = 1970i64;
    let mut d = days as i64;
    loop {
        let leap = (y % 4 == 0 && y % 100 != 0) || y % 400 == 0;
        let len = if leap { 366 } else { 365 };
        if d < len {
            break;
        }
        d -= len;
        y += 1;
    }
    let leap = (y % 4 == 0 && y % 100 != 0) || y % 400 == 0;
    let months = [31, if leap { 29 } else { 28 }, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31];
    let mut m = 0usize;
    while m < 12 && d >= months[m] {
        d -= months[m];
        m += 1;
    }
    format!("{:04}-{:02}-{:02} {:02}:{:02}:{:02}", y, m + 1, d + 1, hh, mm, ss)
}
