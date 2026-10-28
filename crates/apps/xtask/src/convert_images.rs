//! `convert-images`：把旧仓 `assets/` 的模板迁到 `resource/<server>/image/`，并产出索引。
//!
//! ## 为什么要裁切
//!
//! 旧仓的 `Button` 资产是**整张 1280×720 截图**，真正参与匹配的是
//! `load_image(file, area)` 产生的裁切（`module/base/button.py: ensure_template`）。
//! 直接把整图当模板喂给 MaaFW 会得到 `templ size is too large`——
//! 这是合成帧测试抓到的第一批错误之一（见 `docs/5-engineering/5.2-testing.md`）。
//!
//! 所以规则是：
//!
//! | 旧声明 | 落盘 |
//! |---|---|
//! | `Button(... area=R, file=F)` | `F` 裁 `R` → `<module>/<node>.png` |
//! | `Template(file=F)`（无 area） | 整图 → `<module>/<node>.png` |
//! | 动图 | 每帧裁 `R` → `<module>/<node>/frame_NN.png` |
//!
//! ## 只搬被引用的
//!
//! 输入是 61 个 `assets.py` 的 `file=` 声明，不是目录树——`assets/` 里有大量
//! 未被引用的历史残留（969 个 GIF 里绝大多数是共享动效池，仍有被引用者）。

use std::collections::BTreeMap;
use std::fs;
use std::io::BufReader;
use std::path::{Path, PathBuf};
use std::process::Command;

use image::AnimationDecoder;
use serde::Serialize;

use crate::assets_py::{self, Kind};
use crate::naming;

/// 单个动图最多抽多少帧。
const MAX_GIF_FRAMES: usize = 64;

/// 运行参数。
pub struct Options {
    /// 旧仓根目录
    pub old_root: PathBuf,
    /// 新仓 `resource/` 目录
    pub resource_dir: PathBuf,
    /// 只统计不落盘
    pub dry_run: bool,
}

/// 运行结果摘要。
#[derive(Debug, Default)]
pub struct Summary {
    /// 声明数
    pub decls: usize,
    /// 有 `file` 的声明数
    pub with_files: usize,
    /// 写出的静态模板数
    pub png_copied: usize,
    /// 转换的动图数
    pub gif_converted: usize,
    /// 抽出的帧数
    pub frames_written: usize,
    /// 按 area 裁切过的数量（静态 + 动图）
    pub cropped: usize,
    /// 跳过项
    pub skipped: usize,
    /// 引用了但磁盘不存在的文件
    pub missing: Vec<String>,
    /// 目标路径冲突
    pub collisions: Vec<String>,
    /// 落盘字节
    pub bytes: u64,
}

#[derive(Serialize)]
struct Index {
    #[serde(rename = "_meta")]
    meta: Meta,
    entries: BTreeMap<String, Entry>,
    skipped: Vec<Skipped>,
    missing: Vec<String>,
    collisions: Vec<String>,
}

#[derive(Serialize)]
struct Meta {
    tool: &'static str,
    source_root: String,
    source_rev: String,
    schema_doc: &'static str,
    layout: &'static str,
    crop_rule: &'static str,
    counts: Counts,
}

#[derive(Serialize)]
struct Counts {
    decls: usize,
    with_files: usize,
    png: usize,
    gif: usize,
    frames: usize,
    cropped: usize,
}

#[derive(Serialize)]
struct Entry {
    module: String,
    kind: &'static str,
    node: String,
    /// server → 包内相对路径；目录（动图）以 `/` 结尾
    images: BTreeMap<String, String>,
    /// 是否有动图
    has_animated: bool,
}

#[derive(Serialize)]
struct Skipped {
    constant: String,
    module: String,
    reason: String,
}

/// 执行迁移。
pub fn run(opts: &Options) -> Result<Summary, String> {
    let mut sum = Summary::default();
    let mut entries: BTreeMap<String, Entry> = BTreeMap::new();
    let mut skipped: Vec<Skipped> = Vec::new();
    let mut seen_dest: BTreeMap<String, String> = BTreeMap::new();

    let asset_files = find_assets_py(&opts.old_root.join("module"));
    if asset_files.is_empty() {
        return Err(format!(
            "在 {} 下没找到 assets.py，检查 --old-root",
            opts.old_root.display()
        ));
    }

    for path in asset_files {
        let module = module_of(&path);
        let src =
            fs::read_to_string(&path).map_err(|e| format!("读 {} 失败: {e}", path.display()))?;

        for d in assets_py::parse_assets_py(&src, &module) {
            sum.decls += 1;

            if d.files.is_empty() {
                sum.skipped += 1;
                skipped.push(Skipped {
                    constant: d.constant.clone(),
                    module: module.clone(),
                    reason: "无 file（纯色检测按钮）".to_string(),
                });
                continue;
            }
            sum.with_files += 1;

            let node = naming::node_name(&module, &d.constant);
            let mut images = BTreeMap::new();

            for (server, rel) in &d.files {
                // 逻辑服务器 = `file` 字典的键；路径只用来找源文件。
                // 旧仓大量条目把 en/jp/tw 指向 cn 的同一张图（美术复用），
                // 用路径推断会把四份塌成一份，导致其它服务器包缺图（实测踩过）。
                if !naming::is_server(server) {
                    sum.skipped += 1;
                    skipped.push(Skipped {
                        constant: d.constant.clone(),
                        module: module.clone(),
                        reason: format!("非服务器键 `{server}`：{rel}"),
                    });
                    continue;
                }
                let srv = server.clone();
                if naming::split_server_path(rel).is_none() {
                    sum.skipped += 1;
                    skipped.push(Skipped {
                        constant: d.constant.clone(),
                        module: module.clone(),
                        reason: format!("路径不在 assets/<server>/ 下：{rel}"),
                    });
                    continue;
                }

                let abs = opts.old_root.join(rel.trim_start_matches("./"));
                if !abs.is_file() {
                    sum.missing.push(rel.clone());
                    continue;
                }

                // 裁切区域：Button 按本服务器的 area 裁；Template 无 area → 不裁
                let crop = area_of(&d, &srv);
                let is_gif = abs
                    .extension()
                    .and_then(|e| e.to_str())
                    .is_some_and(|e| e.eq_ignore_ascii_case("gif"));

                let in_bundle = if is_gif {
                    format!("{}/", naming::gif_dir(&module, &node))
                } else {
                    naming::image_path(&module, &node)
                };
                let dest_rel = format!("{}/{in_bundle}", naming::bundle_path(&srv));

                if let Some(prev) = seen_dest.get(&dest_rel) {
                    if prev != &d.constant {
                        let msg = format!("{dest_rel} ← {} 与 {prev} 争用", d.constant);
                        if !sum.collisions.contains(&msg) {
                            sum.collisions.push(msg);
                        }
                        continue;
                    }
                } else {
                    seen_dest.insert(dest_rel.clone(), d.constant.clone());
                }

                let dest_abs = opts.resource_dir.join(dest_rel.trim_end_matches('/'));

                if is_gif {
                    sum.gif_converted += 1;
                    if crop.is_some() {
                        sum.cropped += 1;
                    }
                    if opts.dry_run {
                        sum.frames_written += count_gif_frames(&abs).unwrap_or(1);
                    } else {
                        let n = write_gif_frames(&abs, &dest_abs, crop)?;
                        sum.frames_written += n;
                        sum.bytes += dir_bytes(&dest_abs);
                    }
                } else {
                    sum.png_copied += 1;
                    if crop.is_some() {
                        sum.cropped += 1;
                    }
                    if opts.dry_run {
                        sum.bytes += fs::metadata(&abs).map(|m| m.len()).unwrap_or(0);
                    } else {
                        sum.bytes += write_template(&abs, &dest_abs, crop)?;
                    }
                }

                images.insert(srv, in_bundle);
            }

            if images.is_empty() {
                continue;
            }
            entries.insert(
                format!("{}::{}", d.module, d.constant),
                Entry {
                    module: module.clone(),
                    kind: match d.kind {
                        Kind::Button => "button",
                        Kind::Template => "template",
                    },
                    node,
                    has_animated: images.values().any(|p| p.ends_with('/')),
                    images,
                },
            );
        }
    }

    if opts.dry_run {
        eprintln!("[dry-run] 未写入任何文件");
        return Ok(sum);
    }

    let doc = Index {
        meta: Meta {
            tool: "xtask convert-images",
            source_root: opts.old_root.display().to_string(),
            source_rev: git_rev(&opts.old_root),
            schema_doc: "docs/4-migration/4.1-images.md",
            layout: "resource/<server>/image/<module>/<node>[.png|/]",
            crop_rule: "Button 按 area 裁切；Template 不裁",
            counts: Counts {
                decls: sum.decls,
                with_files: sum.with_files,
                png: sum.png_copied,
                gif: sum.gif_converted,
                frames: sum.frames_written,
                cropped: sum.cropped,
            },
        },
        entries,
        skipped,
        missing: sum.missing.clone(),
        collisions: sum.collisions.clone(),
    };
    fs::create_dir_all(&opts.resource_dir).map_err(|e| format!("建目录失败: {e}"))?;
    let text = serde_json::to_string_pretty(&doc).map_err(|e| format!("序列化失败: {e}"))?;
    fs::write(
        opts.resource_dir.join("image_index.json"),
        format!("{text}\n"),
    )
    .map_err(|e| format!("写索引失败: {e}"))?;

    Ok(sum)
}

/// 取某服务器的裁切区域。
fn area_of(d: &assets_py::Decl, server: &str) -> Option<[u32; 4]> {
    d.areas
        .iter()
        .find(|(s, _)| s == server)
        .and_then(|(_, r)| {
            let (x1, y1, x2, y2) = *r;
            (x2 > x1 && y2 > y1).then_some([x1 as u32, y1 as u32, x2 as u32, y2 as u32])
        })
}

/// 写静态模板：有 crop 就裁，否则整图拷（**不改像素**）。
fn write_template(src: &Path, dest: &Path, crop: Option<[u32; 4]>) -> Result<u64, String> {
    if let Some(parent) = dest.parent() {
        fs::create_dir_all(parent).map_err(|e| format!("建目录失败: {e}"))?;
    }
    match crop {
        None => {
            fs::copy(src, dest).map_err(|e| format!("复制失败: {e}"))?;
            Ok(fs::metadata(dest).map(|m| m.len()).unwrap_or(0))
        }
        Some([x1, y1, x2, y2]) => {
            let img = image::open(src)
                .map_err(|e| format!("读 {} 失败: {e}", src.display()))?
                .to_rgba8();
            let w = (x2 - x1).min(img.width().saturating_sub(x1));
            let h = (y2 - y1).min(img.height().saturating_sub(y1));
            if w == 0 || h == 0 {
                return Err(format!(
                    "裁切区域越界：{} area={:?} 图={}x{}",
                    src.display(),
                    [x1, y1, x2, y2],
                    img.width(),
                    img.height()
                ));
            }
            let patch = image::imageops::crop_imm(&img, x1, y1, w, h).to_image();
            patch
                .save_with_format(dest, image::ImageFormat::Png)
                .map_err(|e| format!("写 {} 失败: {e}", dest.display()))?;
            Ok(fs::metadata(dest).map(|m| m.len()).unwrap_or(0))
        }
    }
}

/// 抽动图帧，逐帧裁切后写 `frame_NN.png`。
fn write_gif_frames(src: &Path, dest_dir: &Path, crop: Option<[u32; 4]>) -> Result<usize, String> {
    let file = fs::File::open(src).map_err(|e| format!("打开 {} 失败: {e}", src.display()))?;
    let decoder = image::codecs::gif::GifDecoder::new(BufReader::new(file))
        .map_err(|e| format!("解码 GIF 失败 {}: {e}", src.display()))?;
    let frames = decoder
        .into_frames()
        .collect_frames()
        .map_err(|e| format!("取帧失败 {}: {e}", src.display()))?;

    fs::create_dir_all(dest_dir).map_err(|e| format!("建目录失败: {e}"))?;
    let mut n = 0usize;
    for (i, frame) in frames.iter().take(MAX_GIF_FRAMES).enumerate() {
        let buf = frame.buffer();
        let out = dest_dir.join(format!("frame_{i:02}.png"));
        match crop {
            None => buf
                .save_with_format(&out, image::ImageFormat::Png)
                .map_err(|e| format!("写帧失败 {}: {e}", out.display()))?,
            Some([x1, y1, x2, y2]) => {
                let w = (x2 - x1).min(buf.width().saturating_sub(x1));
                let h = (y2 - y1).min(buf.height().saturating_sub(y1));
                if w == 0 || h == 0 {
                    return Err(format!("动图裁切越界：{}", src.display()));
                }
                let patch = image::imageops::crop_imm(buf, x1, y1, w, h).to_image();
                patch
                    .save_with_format(&out, image::ImageFormat::Png)
                    .map_err(|e| format!("写帧失败 {}: {e}", out.display()))?;
            }
        }
        n += 1;
    }
    Ok(n)
}

fn count_gif_frames(src: &Path) -> Option<usize> {
    let file = fs::File::open(src).ok()?;
    let decoder = image::codecs::gif::GifDecoder::new(BufReader::new(file)).ok()?;
    let frames = decoder.into_frames().collect_frames().ok()?;
    Some(frames.len().min(MAX_GIF_FRAMES))
}

fn dir_bytes(dir: &Path) -> u64 {
    fs::read_dir(dir)
        .map(|rd| {
            rd.flatten()
                .filter_map(|e| e.metadata().ok())
                .map(|m| m.len())
                .sum()
        })
        .unwrap_or(0)
}

fn find_assets_py(root: &Path) -> Vec<PathBuf> {
    let mut out = Vec::new();
    let mut stack = vec![root.to_path_buf()];
    while let Some(dir) = stack.pop() {
        let Ok(rd) = fs::read_dir(&dir) else { continue };
        for entry in rd.flatten() {
            let p = entry.path();
            if p.is_dir() {
                stack.push(p);
            } else if p.file_name().is_some_and(|n| n == "assets.py") {
                out.push(p);
            }
        }
    }
    out.sort();
    out
}

fn module_of(path: &Path) -> String {
    path.parent()
        .and_then(|p| p.file_name())
        .and_then(|n| n.to_str())
        .unwrap_or("misc")
        .to_string()
}

fn git_rev(root: &Path) -> String {
    Command::new("git")
        .args([
            "-C",
            &root.display().to_string(),
            "rev-parse",
            "--short",
            "HEAD",
        ])
        .output()
        .ok()
        .and_then(|o| String::from_utf8(o.stdout).ok())
        .map(|s| s.trim().to_string())
        .filter(|s| !s.is_empty())
        .unwrap_or_else(|| "unknown".to_string())
}
