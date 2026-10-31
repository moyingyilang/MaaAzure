//! 应用 · 守护进程入口。
//!
//! **位置**：`4 应用` 下的可执行入口。依赖 `3 运行时` 与 `1 地基`。
//! 职责（P0 起逐步落地）：实例管理、调度循环、HTTP/WS/MCP、日志分发。
//! 当前为骨架：只提供 `--version` / `--help` / `--tree`，供产物冒烟与结构自检。

use std::path::PathBuf;

use maazure_core::{AzureError, ErrorBranch, TaskOutcome};
use maazure_runner::Engine;
use maazure_runner::replay::Case;

/// 真机执行入口（需要 `maa` feature）。
#[cfg(feature = "maa")]
mod run;

/// 合成帧执行入口（需要 `maa` feature）：不用模拟器跑真实 Pipeline。
#[cfg(feature = "maa")]
mod synth_run;

/// 用真实语料喂 Pipeline（需要 `maa` feature）。
#[cfg(feature = "maa")]
mod corpus_run;

/// 抓一帧「只有游戏窗口」的画面（需要 `maa` feature）。
#[cfg(feature = "maa")]
mod capture;

/// 连续录制游戏窗口画面，产出 golden corpus（需要 `maa` feature）。
#[cfg(feature = "maa")]
mod record;

/// 打印仓库的树状层次，方便确认「东西该放哪」。
fn print_tree() {
    println!("MaaAzure 树");
    println!();
    println!("crates/");
    println!("├── foundation/          1 地基：无业务概念、无 IO");
    println!("│   ├── core             错误树 / 几何 / 任务结果");
    println!("│   └── vision           识别后处理：数字纠错、时长");
    println!("├── domain/              2 领域：纯逻辑，可脱离模拟器测试");
    println!("│   ├── battle           战役 / 地图 / 战斗");
    println!("│   └── daily            日常任务的最小决策");
    println!("├── runtime/             3 运行时：唯一接触 MaaFramework");
    println!("│   └── runner           Engine trait / Tasker 驱动 / 调度器");
    println!("└── apps/                4 应用：入口与工具");
    println!("    ├── daemon           守护进程（本程序）");
    println!("    └── xtask            资源转换 / pipeline 校验");
    println!();
    println!("resource/");
    println!("├── default_pipeline.json  全局基线");
    println!("├── pipeline/              按领域分树：0-common 1-daily 2-battle 3-world");
    println!("└── image/<server>/        模板图（由 AzurPilot assets/ 转换）");
    println!();
    println!("data/");
    println!("├── campaign/              战役地图 JSON（由 campaign/*.py 提取）");
    println!("└── scheduler/             任务优先级与 NextRun 规则");
    println!();
    println!("docs/");
    println!("├── 1-foundation/   决策 · 边界 · 术语");
    println!("├── 2-architecture/ 分层 · 运行时 · 模块映射 · FFI");
    println!("├── 3-authoring/    Pipeline 规范 · 节点树 · 示例");
    println!("├── 4-migration/    图片 · 资源 · 战役数据 · 配置");
    println!("├── 5-engineering/  构建 · 测试 · 发布");
    println!("└── 6-plan/         路线图 · 任务树 · 风险");
}

/// 回放一个 golden corpus 用例：只验证引擎与语料格式，不跑游戏逻辑。
///
/// 领域状态机接进来之后，这里会换成「逐帧喂给状态机并断言决策一致」。
fn replay(dir: &PathBuf) -> std::process::ExitCode {
    let case = match Case::load(dir) {
        Ok(c) => c,
        Err(e) => {
            eprintln!("加载用例失败: {e}");
            return std::process::ExitCode::FAILURE;
        }
    };

    println!("── 回放 {} ──", case.info.name);
    println!("来源版本  {}", case.info.source_rev);
    println!("帧数      {}", case.len());
    println!("结束状态  {}", case.info.outcome);

    let mut engine = maazure_runner::replay::ReplayEngine::new(case);
    let mut echoed = 0usize;

    loop {
        let frame = match engine.screenshot() {
            Ok(f) => f,
            Err(e) => {
                eprintln!("第 {} 帧取图失败: {e}", engine.cursor());
                return std::process::ExitCode::FAILURE;
            }
        };

        if let Some(d) = engine.current_decision() {
            let hits: Vec<String> = d
                .hits
                .iter()
                .filter(|(_, v)| **v)
                .map(|(k, _)| k.clone())
                .collect();
            let action = match d.action.as_ref().and_then(|a| a.point()) {
                Some(p) => format!(" -> 点击({}, {})", p.x, p.y),
                None => String::new(),
            };
            println!(
                "frame {:>4}  {:>4}x{:<4} 命中[{}]{}",
                frame.seq,
                frame.width,
                frame.height,
                hits.join(", "),
                action
            );
            echoed += 1;

            // 语料里的点击要真的投递给引擎：这样断言层能比对「决策 vs 实际投递」
            if let Some(p) = d.action.as_ref().and_then(|a| a.point()) {
                let _ = engine.click(p);
            }
        }

        if !engine.advance() {
            break;
        }
    }

    println!("── 汇总 ──");
    println!("回放帧数    {}", engine.cursor() + 1);
    println!("带决策帧    {echoed}");
    println!("投递点击    {}", engine.clicks.len());

    std::process::ExitCode::SUCCESS
}

fn main() -> std::process::ExitCode {
    let version = env!("CARGO_PKG_VERSION");
    let args: Vec<String> = std::env::args().skip(1).collect();

    match args.first().map(String::as_str) {
        Some("--version") | Some("-V") => {
            println!("MaaAzure {version}");
            std::process::ExitCode::SUCCESS
        }
        Some("--help") | Some("-h") => {
            println!("MaaAzure {version} — 碧蓝航线自动化（MaaFramework + Rust + AGPL-3.0）");
            println!();
            println!("用法:");
            println!("  maazure-daemon --version   版本");
            println!("  maazure-daemon --help      帮助");
            println!("  maazure-daemon --tree      打印仓库树状结构");
            println!("  maazure-daemon --replay <dir>  回放一个 golden corpus 用例");
            println!(
                "  maazure-daemon --run <Task> --adb <path> --address <host:port> [--server cn]"
            );
            println!("                                在真实设备上跑一个任务（需 --features maa）");
            println!();
            println!("P0 骨架：任务执行尚未实现，见 docs/6-plan/6.1-roadmap.md");
            std::process::ExitCode::SUCCESS
        }
        Some("--tree") => {
            print_tree();
            std::process::ExitCode::SUCCESS
        }
        Some("--run") => {
            #[cfg(feature = "maa")]
            {
                run::run_task(&args[1..])
            }
            #[cfg(not(feature = "maa"))]
            {
                eprintln!(
                    "本次二进制没有编译 MaaFramework 支持。\n\
                     用 `cargo build -p maazure-daemon --features maa` 并设置 MAA_SDK_PATH。"
                );
                std::process::ExitCode::from(2)
            }
        }
        Some("--synth") => {
            #[cfg(feature = "maa")]
            {
                synth_run::run_synth(&args[1..])
            }
            #[cfg(not(feature = "maa"))]
            {
                eprintln!("本次二进制没有编译 MaaFramework 支持，--synth 不可用。");
                std::process::ExitCode::from(2)
            }
        }
        Some("--record-window") => {
            #[cfg(feature = "maa")]
            {
                record::run_record(&args[1..])
            }
            #[cfg(not(feature = "maa"))]
            {
                eprintln!("本次二进制没有编译 MaaFramework 支持，--record-window 不可用。");
                std::process::ExitCode::from(2)
            }
        }
        Some("--capture-window") => {
            #[cfg(feature = "maa")]
            {
                capture::run_capture(&args[1..])
            }
            #[cfg(not(feature = "maa"))]
            {
                eprintln!("本次二进制没有编译 MaaFramework 支持，--capture-window 不可用。");
                std::process::ExitCode::from(2)
            }
        }
        Some("--displays") => {
            let use_su = args.iter().any(|a| a == "--su");
            let list = maazure_runner::android_display::list_displays(use_su);
            println!("── display {} 块 ──", list.len());
            for (id, desc) in list {
                println!("  id={id:<20} {desc}");
            }
            std::process::ExitCode::SUCCESS
        }
        Some("--windows") => {
            #[cfg(feature = "maa")]
            {
                match maazure_runner::ffi::MaaEngine::list_windows() {
                    Ok(list) => {
                        println!("── 可控制窗口 {} 个 ──", list.len());
                        for (hwnd, class, title) in list {
                            println!("  hwnd={hwnd:<12} class={class:<28} title={title}");
                        }
                        std::process::ExitCode::SUCCESS
                    }
                    Err(e) => {
                        eprintln!("枚举窗口失败: {e}");
                        std::process::ExitCode::FAILURE
                    }
                }
            }
            #[cfg(not(feature = "maa"))]
            {
                eprintln!("本次二进制没有编译 MaaFramework 支持，--windows 不可用。");
                std::process::ExitCode::from(2)
            }
        }
        Some("--corpus-run") => {
            #[cfg(feature = "maa")]
            {
                corpus_run::run_corpus(&args[1..])
            }
            #[cfg(not(feature = "maa"))]
            {
                eprintln!("本次二进制没有编译 MaaFramework 支持，--corpus-run 不可用。");
                std::process::ExitCode::from(2)
            }
        }
        Some("--replay") => match args.get(1) {
            Some(dir) => replay(&PathBuf::from(dir)),
            None => {
                eprintln!("--replay 需要目录参数");
                std::process::ExitCode::from(2)
            }
        },
        _ => {
            println!("MaaAzure {version}（骨架阶段，尚未实现任务执行）");
            println!("执行内核 MaaFramework · 领域逻辑 Rust · 协议 AGPL-3.0");
            println!("── 自检 ──");
            println!("任务结果三态: {:?}", TaskOutcome::Ok);
            println!(
                "错误分支: Ending={:?} Recoverable={:?} Restart={:?} Fatal={:?}",
                ErrorBranch::Ending,
                ErrorBranch::Recoverable,
                ErrorBranch::Restart,
                ErrorBranch::Fatal
            );
            println!(
                "示例: {:?} 计入失败? {}",
                AzureError::CampaignEnd,
                AzureError::CampaignEnd.counts_as_failure()
            );
            std::process::ExitCode::SUCCESS
        }
    }
}
