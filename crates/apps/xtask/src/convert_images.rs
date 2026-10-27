//! `convert-images`：把旧仓 `assets/` 的模板图迁到 `resource/image/`，并产出索引。
//!
//! 只迁移**被 `assets.py` 引用**的图片——目录里还有大量未被引用的历史文件，
//! 全量拷贝只会给 `pipeline-check` 增加噪声。
//!
//! ```text
//! 旧                                     新
//! assets/cn/commission/X.png      →      image/cn/commission/<node>.png
//! assets/cn/template/X.gif        →      image/cn/template/<node>/frame_00.png …
//! ```
//!
//! 动图单独成目录：MaaFW 的 `template` 字段支持目录路径（递归加载其中图片），
//! 等价于「任一帧命中即命中」，比逐帧写节点干净。

use std::collections::BTreeMap;
use std::fs;
use std::io::BufReader;
use std::path::{Path, PathBuf};
use std::process::Command;

use image::AnimationDecoder;
use serde::Serialize;

use crate::assets_py::{self, Kind};
use crate::naming;

/// 单个动图最多抽多少帧（防止异常动图把磁盘打满）。
const MAX_GIF_FRAMES: usize = 64;

/// 运行参数。
pub struct Options {
    /// 旧仓根目录（含 `module/` 与 `assets/`）
    pub old_root: PathBuf,
    /// 新仓 `resource/` 目录
    pub resource_dir: PathBuf,
    /// 只统计不落盘
    pub dry_run: bool,
}

/// 运行结果摘要。
#[derive(Debug, Default)]
pub struct Summary {
    /// 解析到的声明数
    pub decls: usize,
    /// 有 `file` 的声明数
    pub with_files: usize,
    /// 复制的 PNG 数
    pub png_copied: usize,
    /// 转换的 GIF 数
    pub gif_converted: usize,
    /// 抽取的帧总数
    pub frames_written: usize,
    /// 跳过项数
    pub skipped: usize,
    /// 声明里出现的 (server, path) 总条目数
    pub file_refs: usize,
    /// 唯一目标路径数
    pub dest_unique: usize,
    /// 内容重复的文件份数（磁盘上冗余，git 侧会被去重）
    pub dup_files: usize,
    /// 重复内容占用的字节
    pub dup_bytes: u64,
    /// 目标已存在且大小相同（重复运行时命中）
    pub already_same: usize,
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
    counts: Counts,
}

#[derive(Serialize)]
struct Counts {
    decls: usize,
    with_files: usize,
    png: usize,
    gif: usize,
    frames: usize,
}

#[derive(Serialize)]
struct Entry {
    module: String,
    kind: &'static str,
    node: String,
    /// server → `resource/` 下的相对路径。
    /// 目录（动图抽帧）以 `/` 结尾，读索引时据此判断，不要另设布尔字段。
    images: BTreeMap<String, String>,
    /// 是否有动图（任一 server 是目录）
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
    // 内容哈希 → 该内容首次出现时的字节数（用于统计冗余量）
    let mut by_content: BTreeMap<u64, u64> = BTreeMap::new();

    let asset_files = find_assets_py(&opts.old_root.join("module"));
    if asset_files.is_empty() {
        return Err(format!(
            "在 {} 下没找到 assets.py，检查 --old-root",
            opts.old_root.display()
        ));
    }

    for path in asset_files {
        let module = module_of(&path);
        let src = fs::read_to_string(&path)
            .map_err(|e| format!("读 {} 失败: {e}", path.display()))?;

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
            sum.file_refs += d.files.len();

            let node = naming::node_name(&module, &d.constant);
            let mut images = BTreeMap::new();

            for (server, rel) in &d.files {
                // 用 dict 的键（逻辑服务器），不用路径推断：
                // 旧仓大量条目把 en/jp/tw 指向 cn 的同一张图（美术复用），
                // 逻辑服务器才是资源包该放的目录。
                if !naming::is_server(server) {
                    sum.skipped += 1;
                    skipped.push(Skipped {
                        constant: d.constant.clone(),
                        module: module.clone(),
                        reason: format!("非服务器键 `{server}`，需人工确认：{rel}"),
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

                let is_gif = abs
                    .extension()
                    .and_then(|e| e.to_str())
                    .is_some_and(|e| e.eq_ignore_ascii_case("gif"));
                let dest_rel = if is_gif {
                    // 尾斜杠 = 目录（多帧）。写进索引即自描述。
                    format!("{}/", naming::gif_dir(&srv, &module, &node))
                } else {
                    naming::image_path(&srv, &module, &node)
                };

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
                    if opts.dry_run {
                        sum.frames_written += count_gif_frames(&abs).unwrap_or(1);
                    } else {
                        let n = write_gif_frames(&abs, &dest_abs)?;
                        sum.frames_written += n;
                        sum.bytes += dir_bytes(&dest_abs);
                    }
                } else {
                    sum.png_copied += 1;
                    debug_assert!(!dest_rel.ends_with('/'));
                    sum.bytes += fs::metadata(&abs).map(|m| m.len()).unwrap_or(0);
                    if !opts.dry_run {
                        if let Some(parent) = dest_abs.parent() {
                            fs::create_dir_all(parent).map_err(|e| format!("建目录失败: {e}"))?;
                        }
                        if same_size(&abs, &dest_abs) {
                            sum.already_same += 1;
                        }
                        if !same_size(&abs, &dest_abs) {
                            let key = content_hash(&abs);
                            // 硬链接在 Android 应用数据区被 EPERM 拒绝（实测），
                            // 所以不做链接去重，只统计「重复内容」给发布决策提供依据：
                            // 磁盘 138MB，其中唯一内容约 40MB（git 也只存唯一 blob）。
                            match key.and_then(|k| by_content.get(&k).copied()) {
                                Some(sz) => {
                                    sum.dup_files += 1;
                                    sum.dup_bytes += sz;
                                    fs::copy(&abs, &dest_abs)
                                        .map_err(|e| format!("复制失败: {e}"))?;
                                }
                                None => {
                                    fs::copy(&abs, &dest_abs)
                                        .map_err(|e| format!("复制失败: {e}"))?;
                                    if let Some(k) = key {
                                        by_content.insert(k, fs::metadata(&abs).map(|m| m.len()).unwrap_or(0));
                                    }
                                }
                            }
                        }
                    }
                }

                images.insert(srv, dest_rel);
            }

            sum.dest_unique = seen_dest.len();

            if images.is_empty() {
                continue;
            }
            let kind = match d.kind {
                Kind::Button => "button",
                Kind::Template => "template",
            };
            entries.insert(
                format!("{}::{}", d.module, d.constant),
                Entry {
                    module: module.clone(),
                    kind,
                    node,
                    has_animated: images.values().any(|p| p.ends_with('/')),
                    images,
                },
            );
        }
    }

    if opts.dry_run {
        eprintln!("[dry-run] 未写入任何文件");
    } else {
        let doc = Index {
            meta: Meta {
                tool: "xtask convert-images",
                source_root: opts.old_root.display().to_string(),
                source_rev: git_rev(&opts.old_root),
                schema_doc: "docs/4-migration/4.1-images.md",
                counts: Counts {
                    decls: sum.decls,
                    with_files: sum.with_files,
                    png: sum.png_copied,
                    gif: sum.gif_converted,
                    frames: sum.frames_written,
                },
            },
            entries,
            skipped,
            missing: sum.missing.clone(),
            collisions: sum.collisions.clone(),
        };
        fs::create_dir_all(&opts.resource_dir).map_err(|e| format!("建目录失败: {e}"))?;
        let text = serde_json::to_string_pretty(&doc).map_err(|e| format!("序列化失败: {e}"))?;
        fs::write(opts.resource_dir.join("image_index.json"), format!("{text}\n"))
            .map_err(|e| format!("写索引失败: {e}"))?;
    }

    Ok(sum)
}

/// 递归找 `module/**/assets.py`（有序，保证输出稳定）。
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

/// `module/commission/assets.py` → `commission`。
fn module_of(path: &Path) -> String {
    path.parent()
        .and_then(|p| p.file_name())
        .and_then(|n| n.to_str())
        .unwrap_or("misc")
        .to_string()
}

/// 把动图抽成 `frame_00.png` … 写入 `dest_dir`，返回帧数。
fn write_gif_frames(src: &Path, dest_dir: &Path) -> Result<usize, String> {
    let file = fs::File::open(src).map_err(|e| format!("打开 {} 失败: {e}", src.display()))?;
    let decoder = image::codecs::gif::GifDecoder::new(BufReader::new(file))
        .map_err(|e| format!("解码 GIF 失败 {}: {e}", src.display()))?;
    let frames = decoder
        .into_frames()
        .collect_frames()
        .map_err(|e| format!("取帧失败 {}: {e}", src.display()))?;

    fs::create_dir_all(dest_dir).map_err(|e| format!("建目录失败: {e}"))?;
    let mut n = 0;
    for (i, frame) in frames.iter().take(MAX_GIF_FRAMES).enumerate() {
        let out = dest_dir.join(format!("frame_{i:02}.png"));
        frame
            .buffer()
            .save_with_format(&out, image::ImageFormat::Png)
            .map_err(|e| format!("写帧失败 {}: {e}", out.display()))?;
        n += 1;
    }
    Ok(n)
}

/// 只数帧数，不解码像素。
fn count_gif_frames(src: &Path) -> Option<usize> {
    let file = fs::File::open(src).ok()?;
    let decoder = image::codecs::gif::GifDecoder::new(BufReader::new(file)).ok()?;
    let frames = decoder.into_frames().collect_frames().ok()?;
    Some(frames.len().min(MAX_GIF_FRAMES))
}

/// 文件内容的 FNV-1a 64 位哈希（够用、无依赖、比加密哈希快）。
fn content_hash(path: &Path) -> Option<u64> {
    let data = fs::read(path).ok()?;
    let mut h: u64 = 0xcbf2_9ce4_8422_2325;
    for b in data {
        h ^= b as u64;
        h = h.wrapping_mul(0x0000_0100_0000_01b3);
    }
    Some(h)
}

fn same_size(a: &Path, b: &Path) -> bool {
    match (fs::metadata(a), fs::metadata(b)) {
        (Ok(x), Ok(y)) => x.len() == y.len(),
        _ => false,
    }
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

/// 旧仓当前 commit（写进索引，便于追溯「这张图来自哪一版」）。
fn git_rev(root: &Path) -> String {
    Command::new("git")
        .args(["-C", &root.display().to_string(), "rev-parse", "--short", "HEAD"])
        .output()
        .ok()
        .and_then(|o| String::from_utf8(o.stdout).ok())
        .map(|s| s.trim().to_string())
        .filter(|s| !s.is_empty())
        .unwrap_or_else(|| "unknown".to_string())
}
