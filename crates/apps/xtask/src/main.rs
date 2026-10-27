//! 应用 · 工具链入口（`cargo run -p xtask -- <cmd>`）。
//!
//! **位置**：`4 应用` 下与 daemon 平级。不参与运行时，只在开发期生成与校验资源。
//!
//! | 子命令 | 作用 | 阶段 | 文档 |
//! |---|---|---|---|
//! | `convert-images`   | `assets/` → `resource/image/` | P0 | 4.1 |
//! | `convert-assets`   | 61 个 `assets.py` → 节点骨架 | P0 | 4.2 |
//! | `convert-campaign` | `campaign/*.py` → `data/campaign/` | P0 | 4.3 |
//! | `convert-config`   | `args.json` → `interface.json` | P1 | 4.4 |
//! | `pipeline-check`   | 引用完整性 / 孤儿节点 / 可达性 | P0 | 3.1 §7 |
//! | `release`          | 统一版本号并打包 | P4 | 5.3 |
//!
//! 骨架阶段只有 `--list`：把子命令名固定下来，避免各文档里的命令漂移。

fn main() {
    let cmds: [(&str, &str); 6] = [
        ("convert-images", "assets/ → resource/image/"),
        ("convert-assets", "assets.py → 节点骨架"),
        ("convert-campaign", "campaign/*.py → data/campaign/"),
        ("convert-config", "args.json → interface.json"),
        ("pipeline-check", "引用 / 孤儿 / 可达性校验"),
        ("release", "统一版本号并打包"),
    ];

    match std::env::args().nth(1).as_deref() {
        None | Some("--list") => {
            println!("xtask 子命令（均未实现，P0 起逐个落地）：");
            for (name, desc) in cmds {
                println!("  {name:<18} {desc}");
            }
        }
        Some(other) => {
            eprintln!("子命令 `{other}` 尚未实现。用 `--list` 查看计划。");
            std::process::exit(2);
        }
    }
}
