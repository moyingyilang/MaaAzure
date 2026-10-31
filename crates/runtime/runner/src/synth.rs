//! 合成帧控制器：不用模拟器也能让 MaaFramework 跑**真实 Pipeline**。
//!
//! 原理：`Controller::new_custom` 允许我们提供自己的控制单元，`screencap()` 直接返回
//! PNG 字节。于是造一张 1280×720 的底板，把指定节点的模板图贴到它的 `roi` 上，
//! 就能骗过模板匹配，让 Pipeline 真的往下走。
//!
//! 这不是游戏模拟，而是**链路验证**：
//!
//! | 它验证什么 | 它不验证什么 |
//! |---|---|
//! | 模板路径、`roi`、阈值是否自洽 | 真实游戏画面是否长这样 |
//! | `next` 图与锚点跳转是否按预期走 | 截图链路的时延 |
//! | `Custom` 节点能否被调到 | OCR 在真实字体上的准确率 |
//! | FFI 全链路（Controller → Tasker → Pipeline） | 与真机的时序差异 |
//!
//! 用法见 `maazure-daemon --synth`，说明见 `docs/5-engineering/5.2-testing.md`。

use std::collections::BTreeMap;
use std::io::Cursor;
use std::path::{Path, PathBuf};
use std::sync::Arc;
use std::sync::Mutex;
use std::sync::atomic::{AtomicUsize, Ordering};

use image::{ImageFormat, Rgb, RgbImage};
use maa_framework::custom_controller::CustomControllerCallback;

/// 画布尺寸（沿用 1280×720 基准）。
pub const W: u32 = 1280;
/// 画布高度。
pub const H: u32 = 720;

/// 一帧合成画面。
#[derive(Clone)]
pub struct Frame {
    png: Vec<u8>,
    /// 这一帧贴了哪些节点（排查用）
    pub pasted: Vec<String>,
}

impl Frame {
    /// PNG 字节（导出语料用）。
    pub fn png_bytes(&self) -> &[u8] {
        &self.png
    }
}

/// 画布：在灰底上贴模板。
pub struct Canvas {
    img: RgbImage,
    pasted: Vec<String>,
}

impl Default for Canvas {
    fn default() -> Self {
        Self::new()
    }
}

impl Canvas {
    /// 新建灰底画布。
    pub fn new() -> Self {
        Self {
            // 20/255 的深灰：接近游戏背景，又不会命中任何浅色模板
            img: RgbImage::from_pixel(W, H, Rgb([20, 20, 20])),
            pasted: Vec::new(),
        }
    }

    /// 把某节点的模板图贴到它的 `roi` 上。
    ///
    /// 节点与 `roi` 都从资源包里现查——所以这一步同时在验证「节点的 roi 写得对不对」。
    /// 手写节点常常只写 `template` 不写 `roi`（让它全屏扫），这时回退到
    /// **同模板的生成节点**的 roi，否则合成帧不知道该贴哪。
    pub fn paste_node(&mut self, bundles: &[PathBuf], node: &str) -> Result<(), String> {
        let (tpl, mut roi, _hint) = resolve_node(bundles, node)?;
        if roi.is_none() {
            eprintln!("  提示：节点 {node} 没有 roi，改用它模板对应的 roi 贴图（全屏节点属正常）");
            roi = roi_of_template(bundles, &tpl)?;
        }
        let img = image::open(&tpl)
            .map_err(|e| format!("读模板 {} 失败: {e}", tpl.display()))?
            .to_rgb8();

        let (x, y) = match roi {
            Some(r) => (r[0], r[1]),
            None => {
                return Err(format!(
                    "节点 {node} 没有 roi，无法定位；合成帧只支持带 roi 的节点"
                ));
            }
        };
        // 模板尺寸通常等于 roi 尺寸；不一致时按 roi 缩放，避免「贴上去但不匹配」。
        // 注意：MaaFramework 的 roi 是 [x, y, w, h]，不是 [x1, y1, x2, y2]。
        let target_w = roi.map_or(img.width(), |r| r[2].max(1) as u32);
        let target_h = roi.map_or(img.height(), |r| r[3].max(1) as u32);
        let patch = if (img.width(), img.height()) == (target_w, target_h) {
            img
        } else {
            image::imageops::resize(
                &img,
                target_w,
                target_h,
                image::imageops::FilterType::Triangle,
            )
        };

        image::imageops::overlay(&mut self.img, &patch, x.max(0) as i64, y.max(0) as i64);
        self.pasted.push(node.to_string());
        Ok(())
    }

    /// 编码成 PNG 字节。
    pub fn to_frame(&self) -> Frame {
        let mut buf = Cursor::new(Vec::new());
        self.img
            .write_to(&mut buf, ImageFormat::Png)
            .expect("PNG 编码不会失败");
        Frame {
            png: buf.into_inner(),
            pasted: self.pasted.clone(),
        }
    }
}

/// 解析节点名 → (模板绝对路径, roi)。
/// 按模板路径反查某个带 roi 的节点，取它的 roi。
fn roi_of_template(bundles: &[PathBuf], tpl: &Path) -> Result<Option<[i32; 4]>, String> {
    let rel = tpl
        .strip_prefix(tpl.ancestors().nth(2).unwrap_or(Path::new("")))
        .unwrap_or(tpl)
        .to_string_lossy()
        .to_string();
    for bundle in bundles {
        for (_name, value) in load_nodes(&bundle.join("pipeline"))? {
            let Some(t) = value.get("template").and_then(first_template) else {
                continue;
            };
            if t.trim_end_matches('/') == rel {
                if let Some(r) = value.get("roi").and_then(parse_roi) {
                    return Ok(Some(r));
                }
            }
        }
    }
    Ok(None)
}

fn resolve_node(
    bundles: &[PathBuf],
    node: &str,
) -> Result<(PathBuf, Option<[i32; 4]>, Option<String>), String> {
    // 节点定义可能在手写包（common），图却在服务器包里——两件事分开找。
    for bundle in bundles {
        let pipeline = bundle.join("pipeline");
        for (name, value) in load_nodes(&pipeline)? {
            if name != node {
                continue;
            }
            let tpl = value
                .get("template")
                .and_then(first_template)
                .ok_or_else(|| format!("节点 {node} 没有 template"))?;
            let roi = value.get("roi").and_then(parse_roi);
            let rel = tpl.trim_end_matches('/');

            // 模板图按「先 common 后服务器」的顺序找，但实际存在的是服务器包那份
            for b in bundles {
                let candidate = b.join("image").join(rel);
                if candidate.is_file() {
                    return Ok((candidate, roi, None));
                }
                // 动图目录：取第一帧
                if candidate.is_dir() {
                    let mut frames: Vec<PathBuf> = std::fs::read_dir(&candidate)
                        .map(|rd| rd.flatten().map(|e| e.path()).collect())
                        .unwrap_or_default();
                    frames.sort();
                    if let Some(first) = frames.into_iter().next() {
                        return Ok((first, roi, None));
                    }
                }
            }
            return Err(format!(
                "节点 {node} 的模板 {rel} 在所有包里都找不到（看图是不是没生成）"
            ));
        }
    }
    Err(format!("在 {:?} 里找不到节点 {node}", bundles))
}

fn first_template(v: &serde_json::Value) -> Option<String> {
    match v {
        serde_json::Value::String(s) => Some(s.clone()),
        serde_json::Value::Array(a) => a.first().and_then(|x| x.as_str()).map(str::to_string),
        _ => None,
    }
}

fn parse_roi(v: &serde_json::Value) -> Option<[i32; 4]> {
    let a = v.as_array()?;
    if a.len() != 4 {
        return None;
    }
    let mut r = [0i32; 4];
    for (i, x) in a.iter().enumerate() {
        r[i] = x.as_i64()? as i32;
    }
    Some(r)
}

fn load_nodes(pipeline: &Path) -> Result<BTreeMap<String, serde_json::Value>, String> {
    let mut out = BTreeMap::new();
    let mut stack = vec![pipeline.to_path_buf()];
    while let Some(dir) = stack.pop() {
        let Ok(rd) = std::fs::read_dir(&dir) else {
            continue;
        };
        for e in rd.flatten() {
            let p = e.path();
            if p.is_dir() {
                stack.push(p);
                continue;
            }
            if p.extension().and_then(|x| x.to_str()) != Some("json") {
                continue;
            }
            let Ok(text) = std::fs::read_to_string(&p) else {
                continue;
            };
            let Ok(doc) = serde_json::from_str::<serde_json::Value>(&text) else {
                continue;
            };
            if let Some(obj) = doc.as_object() {
                for (k, v) in obj {
                    if !k.starts_with("_meta") && v.is_object() {
                        out.insert(k.clone(), v.clone());
                    }
                }
            }
        }
    }
    Ok(out)
}

/// 合成帧控制器。
///
/// 每次 `click`/`swipe` 推进到下一帧（如果还有），这样「点完之后画面变化」的流程
/// 也能被验证；只有一帧时就一直返回那一帧。
#[derive(Clone)]
pub struct SynthController {
    inner: Arc<Inner>,
}

struct Inner {
    frames: Vec<Vec<u8>>,
    labels: Vec<Vec<String>>,
    cursor: AtomicUsize,
    clicks: Mutex<Vec<(i32, i32)>>,
    swipes: Mutex<Vec<(i32, i32, i32, i32, i32)>>,
}

impl SynthController {
    /// 直接用 PNG 字节序列新建（语料回放走这条）。
    ///
    /// 与 [`Canvas`] 造帧的区别只在于字节从哪来：语料是真实截图，画布是贴图合成。
    pub fn from_png(frames: Vec<Vec<u8>>) -> Self {
        let n = frames.len();
        Self {
            inner: Arc::new(Inner {
                frames,
                labels: vec![Vec::new(); n],
                cursor: AtomicUsize::new(0),
                clicks: Mutex::new(Vec::new()),
                swipes: Mutex::new(Vec::new()),
            }),
        }
    }

    /// 用一组合成帧新建。
    pub fn new(frames: Vec<Frame>) -> Self {
        let labels = frames.iter().map(|f| f.pasted.clone()).collect();
        Self {
            inner: Arc::new(Inner {
                frames: frames.into_iter().map(|f| f.png).collect(),
                labels,
                cursor: AtomicUsize::new(0),
                clicks: Mutex::new(Vec::new()),
                swipes: Mutex::new(Vec::new()),
            }),
        }
    }

    /// 记录到的点击。
    pub fn clicks(&self) -> Vec<(i32, i32)> {
        self.inner
            .clicks
            .lock()
            .map(|v| v.clone())
            .unwrap_or_default()
    }

    /// 记录到的滑动。
    pub fn swipes(&self) -> Vec<(i32, i32, i32, i32, i32)> {
        self.inner
            .swipes
            .lock()
            .map(|v| v.clone())
            .unwrap_or_default()
    }

    /// 每帧贴了哪些节点（与 cursor 对应）。
    pub fn labels(&self) -> &[Vec<String>] {
        &self.inner.labels
    }

    /// 当前帧号。
    pub fn cursor(&self) -> usize {
        self.inner.cursor.load(Ordering::Relaxed)
    }

    fn advance(&self) {
        let cur = self.inner.cursor.load(Ordering::Relaxed);
        if cur + 1 < self.inner.frames.len() {
            self.inner.cursor.store(cur + 1, Ordering::Relaxed);
        }
    }
}

impl CustomControllerCallback for SynthController {
    fn connect(&self) -> bool {
        !self.inner.frames.is_empty()
    }

    fn screencap(&self) -> Option<Vec<u8>> {
        let cur = self
            .inner
            .cursor
            .load(Ordering::Relaxed)
            .min(self.inner.frames.len().saturating_sub(1));
        self.inner.frames.get(cur).cloned()
    }

    fn click(&self, x: i32, y: i32) -> bool {
        if let Ok(mut v) = self.inner.clicks.lock() {
            v.push((x, y));
        }
        self.advance();
        true
    }

    fn swipe(&self, x1: i32, y1: i32, x2: i32, y2: i32, duration: i32) -> bool {
        if let Ok(mut v) = self.inner.swipes.lock() {
            v.push((x1, y1, x2, y2, duration));
        }
        self.advance();
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn canvas_is_gray_1280x720() {
        let c = Canvas::new();
        let f = c.to_frame();
        assert!(!f.png.is_empty());
        assert_eq!(&f.png[1..4], b"PNG");
        assert!(f.pasted.is_empty());
    }

    #[test]
    fn frame_sequence_advances_on_click() {
        let a = Canvas::new().to_frame();
        let mut cb = Canvas::new();
        cb.pasted.push("x".into());
        let b = cb.to_frame();
        let ctrl = SynthController::new(vec![a, b]);
        assert_eq!(ctrl.cursor(), 0);
        assert!(ctrl.click(10, 20));
        assert_eq!(ctrl.cursor(), 1);
        // 到底后停在最后一帧
        assert!(ctrl.click(30, 40));
        assert_eq!(ctrl.cursor(), 1);
        assert_eq!(ctrl.clicks(), vec![(10, 20), (30, 40)]);
        assert!(ctrl.screencap().is_some());
    }

    #[test]
    fn empty_controller_cannot_connect() {
        let ctrl = SynthController::new(Vec::new());
        assert!(!ctrl.connect());
        assert!(ctrl.screencap().is_none());
    }
}
