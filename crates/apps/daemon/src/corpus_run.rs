//! `--corpus-run`：拿 golden corpus 的**真实截图**喂真实 Pipeline。
//!
//! 这是 ④「端到端 + 回放断言」的设备无关部分：语料一旦录好，本机就能跑
//! `Pipeline` 并把结果和 `case.json` 的 `outcome` 对齐。
//!
//! ```bash
//! maazure-daemon --corpus-run corpus/commission --entry CommissionStart
//! ```

use std::path::PathBuf;

use maazure_core::TaskOutcome;
use maazure_runner::Engine;
use maazure_runner::ffi::MaaEngine;
use maazure_runner::replay::Case;
use maazure_runner::synth::SynthController;

struct Args {
    case_dir: PathBuf,
    entry: String,
    server: String,
    resource_root: PathBuf,
}

fn parse(args: &[String]) -> Result<Args, String> {
    let mut case_dir = None;
    let mut entry = None;
    let mut server = "cn".to_string();
    let mut resource_root = PathBuf::from("resource");

    let mut it = args.iter();
    while let Some(a) = it.next() {
        let mut next = |what: &str| -> Result<String, String> {
            it.next().cloned().ok_or_else(|| format!("{what} 缺参数"))
        };
        match a.as_str() {
            "--entry" => entry = Some(next("--entry")?),
            "--server" => server = next("--server")?,
            "--resource" => resource_root = PathBuf::from(next("--resource")?),
            other if other.starts_with("--") => return Err(format!("未知参数 {other}")),
            other => case_dir = Some(PathBuf::from(other)),
        }
    }

    Ok(Args {
        case_dir: case_dir.ok_or("缺少语料目录（如 corpus/commission）")?,
        entry: entry.ok_or("缺少 --entry（Pipeline 入口节点）")?,
        server,
        resource_root,
    })
}

/// 执行。
pub fn run_corpus(args: &[String]) -> std::process::ExitCode {
    let a = match parse(args) {
        Ok(a) => a,
        Err(e) => {
            eprintln!("--corpus-run 参数错误: {e}");
            return std::process::ExitCode::from(2);
        }
    };

    let case = match Case::load(&a.case_dir) {
        Ok(c) => c,
        Err(e) => {
            eprintln!("加载语料失败: {e}");
            return std::process::ExitCode::FAILURE;
        }
    };

    let mut pngs: Vec<Vec<u8>> = Vec::new();
    for i in 0..case.len() {
        let Some(p) = case.frame_path(i) else {
            continue;
        };
        match std::fs::read(p) {
            Ok(b) => pngs.push(b),
            Err(e) => {
                eprintln!("读 {} 失败: {e}", p.display());
                return std::process::ExitCode::FAILURE;
            }
        }
    }
    if pngs.is_empty() {
        eprintln!("语料里没有帧");
        return std::process::ExitCode::FAILURE;
    }

    println!("── MaaAzure corpus-run ──");
    println!(
        "用例      {}（{} 帧，录制结果 {}）",
        case.info.name,
        pngs.len(),
        case.info.outcome
    );
    println!("入口      {}", a.entry);

    let ctrl = SynthController::from_png(pngs);

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

    if let Err(e) = maazure_runner::ffi::init_framework() {
        eprintln!("初始化 MaaFramework 失败: {e}");
        return std::process::ExitCode::FAILURE;
    }

    let started = std::time::Instant::now();
    let result = engine.run_pipeline_traced(&a.entry, "{}");
    let elapsed = started.elapsed();

    let (outcome, visited) = match result {
        Ok((o, v)) => (o, v),
        Err(e) => {
            eprintln!("执行失败: {e}");
            return std::process::ExitCode::FAILURE;
        }
    };
    let outcome_str = match outcome {
        TaskOutcome::Ok => "ok",
        TaskOutcome::Failed => "failed",
        TaskOutcome::Recoverable => "recoverable",
    };

    println!("── 结果 ──");
    println!("耗时      {:.1}s", elapsed.as_secs_f64());
    println!("点击      {:?}", ctrl.clicks());
    println!("执行节点  {visited:?}");
    println!("Pipeline  {outcome_str}");

    // 与录制时的结束状态对齐：这才是「回放断言」的含义
    let expect = case.info.outcome.to_ascii_lowercase();
    if expect.is_empty() || expect == "unknown" {
        println!("断言      跳过（录制时没记结束状态）");
        return std::process::ExitCode::SUCCESS;
    }
    if expect == outcome_str {
        println!("断言      通过（与录制一致：{expect}）");
        std::process::ExitCode::SUCCESS
    } else {
        eprintln!("断言      失败：录制是 {expect}，这次是 {outcome_str}");
        std::process::ExitCode::FAILURE
    }
}
