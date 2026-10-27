//! 应用 · 工具链入口（`cargo run -p xtask -- <cmd>`）。
//!
//! **位置**：`4 应用` 下与 daemon 平级。不参与运行时，只在开发期生成与校验资源。
//!
//! | 子命令 | 作用 | 阶段 | 文档 |
//! |---|---|---|---|
//! | `convert-images`   | `assets/` → `resource/image/` | ✅ P0 | 4.1 |
//! | `convert-assets`   | 61 个 `assets.py` → 节点骨架 | P0 | 4.2 |
//! | `convert-campaign` | `campaign/*.py` → `data/campaign/` | P0 | 4.3 |
//! | `convert-config`   | `args.json` → `interface.json` | P1 | 4.4 |
//! | `pipeline-check`   | 引用完整性 / 孤儿节点 / 可达性 | P0 | 3.1 §7 |
//! | `release`          | 统一版本号并打包 | P4 | 5.3 |

mod assets_py;
mod convert_images;
mod naming;

use std::path::PathBuf;

const USAGE: &str = "\
xtask — MaaAzure 资源工具

用法:
  xtask convert-images [--old-root <dir>] [--resource <dir>] [--dry-run]

参数:
  --old-root <dir>   旧仓根目录，默认 ../AzurPilot
  --resource <dir>   新仓资源目录，默认 resource
  --dry-run          只统计不落盘

其它子命令（未实现）: convert-assets convert-campaign convert-config pipeline-check release";

fn main() -> std::process::ExitCode {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let cmd = args.first().map(String::as_str).unwrap_or("--list");

    match cmd {
        "--list" | "--help" | "-h" => {
            println!("{USAGE}");
            std::process::ExitCode::SUCCESS
        }
        "convert-images" => {
            let mut old_root = PathBuf::from("../AzurPilot");
            let mut resource = PathBuf::from("resource");
            let mut dry_run = false;

            let mut it = args.iter().skip(1);
            while let Some(a) = it.next() {
                match a.as_str() {
                    "--old-root" => match it.next() {
                        Some(v) => old_root = PathBuf::from(v),
                        None => {
                            eprintln!("--old-root 缺参数");
                            return std::process::ExitCode::from(2);
                        }
                    },
                    "--resource" => match it.next() {
                        Some(v) => resource = PathBuf::from(v),
                        None => {
                            eprintln!("--resource 缺参数");
                            return std::process::ExitCode::from(2);
                        }
                    },
                    "--dry-run" => dry_run = true,
                    other => {
                        eprintln!("未知参数: {other}\n\n{USAGE}");
                        return std::process::ExitCode::from(2);
                    }
                }
            }

            let opts = convert_images::Options {
                old_root,
                resource_dir: resource,
                dry_run,
            };
            match convert_images::run(&opts) {
                Ok(s) => {
                    println!("── convert-images ──");
                    println!("声明总数        {}", s.decls);
                    println!("含 file 的声明  {}", s.with_files);
                    println!("PNG 复制        {}", s.png_copied);
                    println!("GIF 转帧        {} 个 → {} 帧", s.gif_converted, s.frames_written);
                    println!("file 条目总数    {}", s.file_refs);
                    println!("唯一目标路径    {}", s.dest_unique);
                    println!(
                        "重复内容        {} 份 / {:.1} MB（git 侧会去重）",
                        s.dup_files,
                        s.dup_bytes as f64 / 1_048_576.0
                    );
                    println!("已存在跳过      {}", s.already_same);
                    println!("跳过            {}", s.skipped);
                    println!("缺失文件        {}", s.missing.len());
                    println!("路径冲突        {}", s.collisions.len());
                    println!("落盘体积        {:.1} MB", s.bytes as f64 / 1_048_576.0);
                    for m in s.missing.iter().take(10) {
                        println!("  缺失: {m}");
                    }
                    for c in s.collisions.iter().take(10) {
                        println!("  冲突: {c}");
                    }
                    std::process::ExitCode::SUCCESS
                }
                Err(e) => {
                    eprintln!("convert-images 失败: {e}");
                    std::process::ExitCode::FAILURE
                }
            }
        }
        other => {
            eprintln!("子命令 `{other}` 尚未实现。\n\n{USAGE}");
            std::process::ExitCode::from(2)
        }
    }
}
