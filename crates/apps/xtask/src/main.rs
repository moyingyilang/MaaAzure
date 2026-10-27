//! 应用 · 工具链入口（`cargo run -p xtask -- <cmd>`）。
//!
//! **位置**：`4 应用` 下与 daemon 平级。不参与运行时，只在开发期生成与校验资源。
//!
//! | 子命令 | 作用 | 状态 | 文档 |
//! |---|---|---|---|
//! | `convert-images`   | `assets/` → `resource/<server>/image/` | ✅ | 4.1 |
//! | `convert-assets`   | 61 个 `assets.py` → 节点叶子 | ✅ | 4.2 |
//! | `pipeline-check`   | 引用 / 锚点 / 图片 / roi / 孤儿 | ✅ | 3.1 §7 |
//! | `convert-campaign` | `campaign/*.py` → `data/campaign/` | 待做 | 4.3 |
//! | `convert-config`   | `args.json` → `interface.json` | 待做 | 4.4 |
//! | `release`          | 统一版本号并打包 | 待做 | 5.3 |

mod assets_py;
mod campaign_py;
mod convert_assets;
mod convert_campaign;
mod convert_images;
mod naming;
mod pipeline_check;

use std::path::PathBuf;
use std::process::ExitCode;

const USAGE: &str = "\
xtask — MaaAzure 资源工具

用法:
  xtask convert-images [--old-root <dir>] [--resource <dir>] [--dry-run]
  xtask convert-assets [--old-root <dir>] [--resource <dir>] [--dry-run]
  xtask convert-campaign [--old-root <dir>] [--data <dir>] [--dry-run]
  xtask pipeline-check [--resource <dir>]
  xtask check                 一次跑完全部离线校验（CI 用这个）

参数:
  --old-root <dir>   旧仓根目录，默认 ../AzurPilot
  --resource <dir>   新仓资源目录，默认 resource
  --data <dir>       新仓数据目录（convert-campaign 用），默认 data
  --dry-run          只统计不落盘

未实现: convert-config release";

/// 解析到的通用参数。
struct Common {
    old_root: PathBuf,
    resource: PathBuf,
    data: PathBuf,
    dry_run: bool,
}

fn parse_common(args: &[String]) -> Result<Common, String> {
    let mut c = Common {
        old_root: PathBuf::from("../AzurPilot"),
        resource: PathBuf::from("resource"),
        data: PathBuf::from("data"),
        dry_run: false,
    };
    let mut it = args.iter();
    while let Some(a) = it.next() {
        match a.as_str() {
            "--old-root" => {
                c.old_root = PathBuf::from(it.next().ok_or("--old-root 缺参数")?);
            }
            "--resource" => {
                c.resource = PathBuf::from(it.next().ok_or("--resource 缺参数")?);
            }
            "--data" => {
                c.data = PathBuf::from(it.next().ok_or("--data 缺参数")?);
            }
            "--dry-run" => c.dry_run = true,
            other => return Err(format!("未知参数: {other}")),
        }
    }
    Ok(c)
}

fn main() -> ExitCode {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let cmd = args.first().map(String::as_str).unwrap_or("--help");

    let common = match parse_common(&args[1.min(args.len())..]) {
        Ok(c) => c,
        Err(e) => {
            eprintln!("{e}\n\n{USAGE}");
            return ExitCode::from(2);
        }
    };

    match cmd {
        "--list" | "--help" | "-h" => {
            println!("{USAGE}");
            ExitCode::SUCCESS
        }
        "convert-images" => {
            let opts = convert_images::Options {
                old_root: common.old_root,
                resource_dir: common.resource,
                dry_run: common.dry_run,
            };
            match convert_images::run(&opts) {
                Ok(s) => {
                    println!("── convert-images ──");
                    println!("声明总数        {}", s.decls);
                    println!("含 file 的声明  {}", s.with_files);
                    println!("PNG 复制        {}", s.png_copied);
                    println!(
                        "GIF 转帧        {} 个 → {} 帧",
                        s.gif_converted, s.frames_written
                    );
                    println!("file 条目总数    {}", s.file_refs);
                    println!("唯一目标路径    {}", s.dest_unique);
                    println!(
                        "重复内容        {} 份 / {:.1} MB（git 侧会去重）",
                        s.dup_files,
                        s.dup_bytes as f64 / 1_048_576.0
                    );
                    println!("跳过            {}", s.skipped);
                    println!("缺失            {}", s.missing.len());
                    println!("冲突            {}", s.collisions.len());
                    println!("落盘体积        {:.1} MB", s.bytes as f64 / 1_048_576.0);
                    for m in s.missing.iter().take(10) {
                        println!("  缺失: {m}");
                    }
                    for c in s.collisions.iter().take(10) {
                        println!("  冲突: {c}");
                    }
                    ExitCode::SUCCESS
                }
                Err(e) => {
                    eprintln!("convert-images 失败: {e}");
                    ExitCode::FAILURE
                }
            }
        }
        "convert-assets" => {
            let opts = convert_assets::Options {
                old_root: common.old_root,
                resource_dir: common.resource,
                dry_run: common.dry_run,
            };
            match convert_assets::run(&opts) {
                Ok(s) => {
                    println!("── convert-assets ──");
                    println!("声明总数        {}", s.decls);
                    println!("节点总数        {}（每声明 1 个 × 服务器）", s.nodes);
                    println!("  模板匹配      {}", s.template_nodes);
                    println!("  动图目录      {}", s.animated_nodes);
                    println!("  纯色匹配      {}", s.color_nodes);
                    println!("无 roi 的节点   {}", s.without_roi);
                    println!("服务器包        {}", s.servers);
                    println!("写出文件        {}", s.files);
                    ExitCode::SUCCESS
                }
                Err(e) => {
                    eprintln!("convert-assets 失败: {e}");
                    ExitCode::FAILURE
                }
            }
        }
        "convert-campaign" => {
            let opts = convert_campaign::Options {
                old_root: common.old_root,
                data_dir: common.data,
                dry_run: common.dry_run,
            };
            match convert_campaign::run(&opts) {
                Ok(s) => {
                    println!("── convert-campaign ──");
                    println!("扫描 .py       {}", s.scanned);
                    println!("地图文件       {}", s.maps);
                    println!("写出 JSON      {}", s.written);
                    println!("跳过（辅助）   {}", s.skipped);
                    println!("校验失败       {}", s.errors.len());
                    println!("待人工字段     {}", s.unresolved.len());
                    println!("未知地图符号   {}", s.unknown_tokens.len());
                    for e in s.errors.iter().take(8) {
                        println!("  ✗ {e}");
                    }
                    for u in s.unresolved.iter().take(5) {
                        println!("  ? {u}");
                    }
                    for u in s.unknown_tokens.iter().take(5) {
                        println!("  △ {u}");
                    }
                    if s.errors.is_empty() {
                        ExitCode::SUCCESS
                    } else {
                        ExitCode::FAILURE
                    }
                }
                Err(e) => {
                    eprintln!("convert-campaign 失败: {e}");
                    ExitCode::FAILURE
                }
            }
        }
        "pipeline-check" => match pipeline_check::run(&common.resource) {
            Ok(r) => {
                println!("── pipeline-check ──");
                println!("资源包          {}", r.bundles);
                println!("索引条目        {}", r.indexed);
                println!("节点总数        {}", r.nodes);
                println!("  其中生成叶子  {}（next 待人工接线）", r.generated);
                println!("错误            {}", r.errors.len());
                println!("警告            {}", r.warnings.len());
                for e in r.errors.iter().take(20) {
                    println!("  ✗ {e}");
                }
                for w in r.warnings.iter().take(10) {
                    println!("  ! {w}");
                }
                if r.ok() {
                    ExitCode::SUCCESS
                } else {
                    ExitCode::FAILURE
                }
            }
            Err(e) => {
                eprintln!("pipeline-check 失败: {e}");
                ExitCode::FAILURE
            }
        },
        // 一条命令跑完所有不需要旧仓、不需要设备的校验
        "check" => {
            println!("── xtask check ──");
            let script = std::path::Path::new("scripts/check-no-python.sh");
            let python_ok = if script.is_file() {
                match std::process::Command::new("bash").arg(script).status() {
                    Ok(s) if s.success() => true,
                    Ok(s) => {
                        eprintln!("零 Python 门禁失败（exit {s}）");
                        false
                    }
                    Err(e) => {
                        eprintln!("跑不了 {}: {e}", script.display());
                        false
                    }
                }
            } else {
                println!("零 Python 门禁  跳过（没找到 scripts/check-no-python.sh）");
                true
            };

            match pipeline_check::run(&common.resource) {
                Ok(r) => {
                    println!("资源包          {}", r.bundles);
                    println!("索引条目        {}", r.indexed);
                    println!("节点总数        {}", r.nodes);
                    println!("生成叶子        {}", r.generated);
                    println!("错误            {}", r.errors.len());
                    println!("警告            {}", r.warnings.len());
                    for e in r.errors.iter().take(20) {
                        println!("  ✗ {e}");
                    }
                    for w in r.warnings.iter().take(20) {
                        println!("  ! {w}");
                    }
                    if r.ok() && python_ok {
                        ExitCode::SUCCESS
                    } else {
                        ExitCode::FAILURE
                    }
                }
                Err(e) => {
                    eprintln!("pipeline-check 失败: {e}");
                    ExitCode::FAILURE
                }
            }
        }
        other => {
            eprintln!("子命令 `{other}` 尚未实现。\n\n{USAGE}");
            ExitCode::from(2)
        }
    }
}
