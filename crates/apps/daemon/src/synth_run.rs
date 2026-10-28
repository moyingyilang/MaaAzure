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
use maazure_runner::fixture::Script;
use maazure_runner::synth::{Canvas, SynthController};

struct Args {
    entry: Option<String>,
    show: Vec<String>,
    script: Option<PathBuf>,
    server: String,
    resource_root: PathBuf,
    frames: usize,
    max_wait: u64,
}

fn parse(args: &[String]) -> Result<Args, String> {
    let mut entry = None;
    let mut show = Vec::new();
    let mut script = None;
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
            "--script" => script = Some(PathBuf::from(next("--script")?)),
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
        entry,
        show,
        script,
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

    // 脚本模式：帧序列与期望值都来自文件
    let script = match &a.script {
        Some(p) => match Script::load(p) {
            Ok(s) => Some(s),
            Err(e) => {
                eprintln!("加载脚本失败: {e}");
                return std::process::ExitCode::FAILURE;
            }
        },
        None => None,
    };

    let Some(entry) = script.as_ref().map(|s| s.entry.clone()).or(a.entry.clone()) else {
        eprintln!("缺少 Pipeline 入口节点（或 --script）");
        return std::process::ExitCode::from(2);
    };
    let server = script
        .as_ref()
        .map(|s| s.server.clone())
        .unwrap_or(a.server.clone());

    let common = a.resource_root.join("common");
    let server_dir = a.resource_root.join(&server);
    let mut bundles: Vec<PathBuf> = Vec::new();
    if common.is_dir() {
        bundles.push(common);
    }
    if !server_dir.is_dir() {
        eprintln!("资源包不存在: {}", server_dir.display());
        return std::process::ExitCode::FAILURE;
    }
    bundles.push(server_dir);

    let name = script
        .as_ref()
        .map(|s| s.name.clone())
        .unwrap_or_else(|| "ad-hoc".to_string());
    println!("── MaaAzure synth ──");
    println!("用例      {name}");
    println!("入口      {entry}");
    println!("资源包    {server}");

    // 组装帧序列：脚本模式按 frames 逐帧贴图，临时模式只有一帧
    let show_sets: Vec<Vec<String>> = match &script {
        Some(s) => s.frames.iter().map(|f| f.show.clone()).collect(),
        None => vec![a.show.clone()],
    };
    let mut frames = Vec::with_capacity(show_sets.len());
    for (i, shows) in show_sets.iter().enumerate() {
        let mut canvas = Canvas::new();
        for node in shows {
            if let Err(e) = canvas.paste_node(&bundles, node) {
                eprintln!("第 {i} 帧贴图失败: {e}");
                return std::process::ExitCode::FAILURE;
            }
        }
        println!("  帧 {i}: {shows:?}");
        frames.push(canvas.to_frame());
    }
    // 至少两帧，点击才能推进画面
    while frames.len() < 2 {
        let last = frames
            .last()
            .cloned()
            .unwrap_or_else(|| Canvas::new().to_frame());
        frames.push(last);
    }
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
    let result = engine.run_pipeline(&entry, "{}");
    let elapsed = started.elapsed();
    let clicks = ctrl.clicks();

    let outcome = match &result {
        Ok(TaskOutcome::Ok) => "ok",
        Ok(TaskOutcome::Failed) => "failed",
        Ok(TaskOutcome::Recoverable) => "recoverable",
        Err(_) => "error",
    };

    println!("── 结果 ──");
    println!(
        "耗时      {:.1}s（上限 {}s）",
        elapsed.as_secs_f64(),
        a.max_wait
    );
    println!("点击      {clicks:?}");
    println!("滑动      {:?}", ctrl.swipes());
    println!("帧号      {}", ctrl.cursor());
    println!("状态      {outcome}");
    if let Err(e) = &result {
        eprintln!("执行失败: {e}");
    }

    // 脚本模式做断言
    if let Some(s) = &script {
        match s.check(outcome, clicks.len()) {
            Ok(()) => {
                println!("断言      通过");
                std::process::ExitCode::SUCCESS
            }
            Err(e) => {
                eprintln!("断言      失败：{e}");
                std::process::ExitCode::FAILURE
            }
        }
    } else if outcome == "ok" {
        std::process::ExitCode::SUCCESS
    } else {
        std::process::ExitCode::FAILURE
    }
}
