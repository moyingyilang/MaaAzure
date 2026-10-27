//! 资源与代码生成工具（`cargo xtask <cmd>`）。
//!
//! 计划中的子命令（见 `docs/A3-resource-migration.md`）：
//!
//! | 子命令 | 作用 | 阶段 |
//! |---|---|---|
//! | `convert-images`   | `assets/` → `resource/image/` + `image_index.json` | P0 |
//! | `convert-assets`   | 61 个 `assets.py` → 节点骨架 JSON | P0 |
//! | `convert-campaign` | `campaign/*.py` → `data/campaign/*.json` | P0 |
//! | `convert-config`   | `args.json` → `interface.json` | P1 |
//! | `pipeline-check`   | 节点引用 / 图片存在性 / 孤儿节点 / 可达性 | P0 |
//! | `release`          | 统一版本号并打包 | P4 |
//!
//! 骨架阶段只有 `--list`，用来固定接口，避免各文档里的命令名漂移。

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let cmds = [
        "convert-images",
        "convert-assets",
        "convert-campaign",
        "convert-config",
        "pipeline-check",
        "release",
    ];

    match args.first().map(String::as_str) {
        Some("--list") | None => {
            println!("可用子命令（均未实现，P0 起逐个落地）：");
            for c in cmds {
                println!("  {c}");
            }
        }
        Some(other) => {
            eprintln!("子命令 `{other}` 尚未实现。用 `--list` 查看计划。");
            std::process::exit(2);
        }
    }
}
