//! 应用 · 守护进程入口。
//!
//! **位置**：`4 应用` 下的可执行入口。依赖 `3 运行时` 与 `1 地基`。
//! 职责（P0 起逐步落地）：实例管理、调度循环、HTTP/WS/MCP、日志分发。
//! 当前为骨架：只提供 `--version` / `--help` / `--tree`，供产物冒烟与结构自检。

use maazure_core::{AzureError, ErrorBranch, TaskOutcome};

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

fn main() {
    let version = env!("CARGO_PKG_VERSION");
    let args: Vec<String> = std::env::args().skip(1).collect();

    match args.first().map(String::as_str) {
        Some("--version") | Some("-V") => println!("MaaAzure {version}"),
        Some("--help") | Some("-h") => {
            println!("MaaAzure {version} — 碧蓝航线自动化（MaaFramework + Rust + AGPL-3.0）");
            println!();
            println!("用法:");
            println!("  maazure-daemon --version   版本");
            println!("  maazure-daemon --help      帮助");
            println!("  maazure-daemon --tree      打印仓库树状结构");
            println!();
            println!("P0 骨架：任务执行尚未实现，见 docs/6-plan/6.1-roadmap.md");
        }
        Some("--tree") => print_tree(),
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
            println!("示例: {:?} 计入失败? {}", AzureError::CampaignEnd, AzureError::CampaignEnd.counts_as_failure());
        }
    }
}
