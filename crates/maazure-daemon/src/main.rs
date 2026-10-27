//! MaaAzure 守护进程入口。
//!
//! P0 目标：`--version` / `--help` 可用，能启动 HTTP 服务并列出实例。
//! 当前为骨架，只打印版本信息，便于 `tools/buildkit/40-verify.sh` 做产物冒烟。

use maazure_core::TaskOutcome;

fn main() {
    let version = env!("CARGO_PKG_VERSION");
    let args: Vec<String> = std::env::args().skip(1).collect();

    match args.first().map(String::as_str) {
        Some("--version") | Some("-V") => println!("MaaAzure {version}"),
        Some("--help") | Some("-h") => {
            println!("MaaAzure {version} — 碧蓝航线自动化（MaaFramework + Rust）");
            println!();
            println!("用法:");
            println!("  maazure-daemon --version   显示版本");
            println!("  maazure-daemon --help      显示帮助");
            println!();
            println!("P0 骨架：任务执行尚未实现，见 docs/A7-roadmap.md");
        }
        _ => {
            // 骨架阶段不启动服务，避免让人以为已经能跑任务
            println!("MaaAzure {version} (骨架阶段，尚未实现任务执行)");
            println!("派生自 AzurPilot，执行内核为 MaaFramework。");
            println!("当前状态: TaskOutcome 已定义 = {:?}", TaskOutcome::Ok);
        }
    }
}
