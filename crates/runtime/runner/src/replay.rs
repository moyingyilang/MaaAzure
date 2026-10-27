//! 录像回放引擎：让领域逻辑在没有模拟器、没有 MaaFramework 的机器上可验证。
//!
//! 输入是 `tools/corpus/record_golden.py` 录出来的目录：
//!
//! ```text
//! corpus/<task>/
//! ├── case.json
//! ├── 0001.png
//! ├── 0001.decision.json
//! └── ...
//! ```
//!
//! 设计依据：`docs/5-engineering/5.2-testing.md`。

use std::collections::BTreeMap;
use std::fs;
use std::path::{Path, PathBuf};

use maazure_core::Point;
use serde::Deserialize;

use crate::{Engine, EngineError, Frame};

/// 一帧的期望决策。
#[derive(Debug, Clone, Deserialize, Default)]
pub struct Decision {
    /// 帧号
    pub frame: u64,
    /// 命中的节点名（新版比旧版多了这一层抽象）
    #[serde(default)]
    pub node: Option<String>,
    /// 各按钮的识别结果
    #[serde(default)]
    pub hits: BTreeMap<String, bool>,
    /// 动作
    #[serde(default)]
    pub action: Option<Action>,
    /// OCR 结果
    #[serde(default)]
    pub ocr: Vec<OcrHit>,
}

/// 动作。
#[derive(Debug, Clone, Deserialize, PartialEq, Eq)]
#[serde(tag = "kind", rename_all = "lowercase")]
pub enum Action {
    /// 点击
    Click {
        /// 旧版 button 区域
        #[serde(default)]
        area: Vec<i32>,
        /// 实际落点
        #[serde(default)]
        point: Vec<i32>,
    },
}

impl Action {
    /// 落点（若有）。
    pub fn point(&self) -> Option<Point> {
        match self {
            Self::Click { point, .. } if point.len() == 2 => Some(Point::new(point[0], point[1])),
            _ => None,
        }
    }

    /// 允许落点的区域（若有）。
    pub fn area(&self) -> Option<[i32; 4]> {
        match self {
            Self::Click { area, .. } if area.len() == 4 => {
                Some([area[0], area[1], area[2], area[3]])
            }
            _ => None,
        }
    }
}

/// 一条 OCR 结果。
#[derive(Debug, Clone, Deserialize)]
pub struct OcrHit {
    /// OCR 类型（`Ocr` / `Digit` / `DigitCounter` …）
    #[serde(default)]
    pub kind: String,
    /// 识别区域
    #[serde(default)]
    pub area: Vec<i32>,
    /// 识别文本
    #[serde(default)]
    pub text: Option<String>,
}

/// `case.json`。
#[derive(Debug, Clone, Deserialize)]
pub struct CaseInfo {
    /// 用例名
    pub name: String,
    /// 旧仓 commit
    #[serde(default)]
    pub source_rev: String,
    /// 帧数
    #[serde(default)]
    pub frames: usize,
    /// 结束原因
    #[serde(default)]
    pub outcome: String,
}

/// 一个用例。
#[derive(Debug)]
pub struct Case {
    /// 目录
    pub dir: PathBuf,
    /// 元信息
    pub info: CaseInfo,
    frames: Vec<(PathBuf, u32, u32)>,
    /// 每帧对应的决策（可能缺帧）
    pub decisions: BTreeMap<u64, Decision>,
}

impl Case {
    /// 从目录加载。
    pub fn load(dir: impl AsRef<Path>) -> Result<Self, String> {
        let dir = dir.as_ref().to_path_buf();
        let case_path = dir.join("case.json");
        let info: CaseInfo = serde_json::from_str(
            &fs::read_to_string(&case_path)
                .map_err(|e| format!("读 {} 失败: {e}", case_path.display()))?,
        )
        .map_err(|e| format!("解析 {} 失败: {e}", case_path.display()))?;

        let mut frames = Vec::new();
        let mut decisions = BTreeMap::new();

        let mut files: Vec<PathBuf> = fs::read_dir(&dir)
            .map_err(|e| format!("读目录 {} 失败: {e}", dir.display()))?
            .flatten()
            .map(|e| e.path())
            .collect();
        files.sort();

        for p in files {
            let name = p.file_name().and_then(|n| n.to_str()).unwrap_or_default().to_string();
            if name.ends_with(".decision.json") {
                let d: Decision = serde_json::from_str(
                    &fs::read_to_string(&p).map_err(|e| format!("读 {} 失败: {e}", p.display()))?,
                )
                .map_err(|e| format!("解析 {} 失败: {e}", p.display()))?;
                decisions.insert(d.frame, d);
            } else if name.ends_with(".png") {
                let (w, h) = png_size(&p)?;
                frames.push((p, w, h));
            }
        }

        if frames.is_empty() {
            return Err(format!("{} 里没有 PNG 帧", dir.display()));
        }

        Ok(Self {
            dir,
            info,
            frames,
            decisions,
        })
    }

    /// 帧数。
    pub fn len(&self) -> usize {
        self.frames.len()
    }

    /// 是否为空。
    pub fn is_empty(&self) -> bool {
        self.frames.is_empty()
    }

    /// 第 n 帧的路径。
    pub fn frame_path(&self, n: usize) -> Option<&Path> {
        self.frames.get(n).map(|(p, _, _)| p.as_path())
    }

    /// 第 n 帧的决策（帧号从 1 开始）。
    pub fn decision(&self, n: usize) -> Option<&Decision> {
        self.decisions.get(&((n + 1) as u64))
    }
}

/// 回放引擎：按顺序吐帧，记录被投递的点击。
#[derive(Debug)]
pub struct ReplayEngine {
    case: Case,
    cursor: usize,
    /// 实际投递的点击（供断言比对）
    pub clicks: Vec<Point>,
    /// 实际投递的滑动
    pub swipes: Vec<(Point, Point, u64)>,
}

impl ReplayEngine {
    /// 新建。
    pub fn new(case: Case) -> Self {
        Self {
            case,
            cursor: 0,
            clicks: Vec::new(),
            swipes: Vec::new(),
        }
    }

    /// 当前帧序号（0 基）。
    pub fn cursor(&self) -> usize {
        self.cursor
    }

    /// 用例信息。
    pub fn case(&self) -> &Case {
        &self.case
    }

    /// 推进到下一帧；到头返回 false（停在最后一帧，便于断言末帧决策）。
    pub fn advance(&mut self) -> bool {
        if self.cursor + 1 >= self.case.len() {
            return false;
        }
        self.cursor += 1;
        true
    }

    /// 当前帧的期望决策。
    pub fn current_decision(&self) -> Option<&Decision> {
        self.case.decision(self.cursor)
    }
}

impl Engine for ReplayEngine {
    fn screenshot(&self) -> Result<Frame, EngineError> {
        let (_, w, h) = self
            .case
            .frames
            .get(self.cursor.min(self.case.len().saturating_sub(1)))
            .ok_or(EngineError::NotConnected)?;
        Ok(Frame {
            // 与语料编号对齐（case 里的帧号是 1 基）
            seq: self.cursor as u64 + 1,
            width: *w,
            height: *h,
        })
    }

    fn click(&mut self, p: Point) -> Result<(), EngineError> {
        self.clicks.push(p);
        Ok(())
    }

    fn swipe(&mut self, from: Point, to: Point, ms: u64) -> Result<(), EngineError> {
        self.swipes.push((from, to, ms));
        Ok(())
    }
}

/// 从 PNG 的 IHDR 读宽高，不解码像素。
fn png_size(path: &Path) -> Result<(u32, u32), String> {
    let data = fs::read(path).map_err(|e| format!("读 {} 失败: {e}", path.display()))?;
    if data.len() < 24 || &data[12..16] != b"IHDR" {
        return Err(format!("{} 不是合法 PNG", path.display()));
    }
    let w = u32::from_be_bytes([data[16], data[17], data[18], data[19]]);
    let h = u32::from_be_bytes([data[20], data[21], data[22], data[23]]);
    Ok((w, h))
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;

    /// 造一个最小 PNG（1×1，无压缩数据也合法到能读 IHDR 即可）。
    fn write_png(path: &Path, w: u32, h: u32) {
        let mut f = fs::File::create(path).unwrap();
        f.write_all(&[0x89, b'P', b'N', b'G', 0x0d, 0x0a, 0x1a, 0x0a]).unwrap();
        f.write_all(&(13u32).to_be_bytes()).unwrap(); // IHDR 长度
        f.write_all(b"IHDR").unwrap();
        f.write_all(&w.to_be_bytes()).unwrap();
        f.write_all(&h.to_be_bytes()).unwrap();
        f.write_all(&[8, 2, 0, 0, 0]).unwrap(); // 位深/色型/压缩/滤波/隔行
        f.write_all(&[0, 0, 0, 0]).unwrap(); // 假 CRC
    }

    fn build_case(dir: &Path) {
        fs::create_dir_all(dir).unwrap();
        fs::write(
            dir.join("case.json"),
            r#"{"name":"demo","source_rev":"abc1234","frames":2,"outcome":"ok"}"#,
        )
        .unwrap();
        write_png(&dir.join("0001.png"), 1280, 720);
        write_png(&dir.join("0002.png"), 1280, 720);
        fs::write(
            dir.join("0001.decision.json"),
            r#"{"frame":1,"hits":{"BTN":true},"action":{"kind":"click","area":[10,20,30,40],"point":[20,30]}}"#,
        )
        .unwrap();
        fs::write(
            dir.join("0002.decision.json"),
            r#"{"frame":2,"hits":{"BTN":false}}"#,
        )
        .unwrap();
    }

    #[test]
    fn loads_case_and_replays_frames() {
        let dir = std::env::temp_dir().join("maazure_case_test_1");
        let _ = fs::remove_dir_all(&dir);
        build_case(&dir);

        let case = Case::load(&dir).unwrap();
        assert_eq!(case.len(), 2);
        assert_eq!(case.info.name, "demo");
        assert!(case.frame_path(0).unwrap().ends_with("0001.png"));

        let d1 = case.decision(0).unwrap();
        assert_eq!(d1.hits.get("BTN"), Some(&true));
        assert_eq!(d1.action.as_ref().unwrap().point(), Some(Point::new(20, 30)));
        assert_eq!(d1.action.as_ref().unwrap().area(), Some([10, 20, 30, 40]));

        let mut engine = ReplayEngine::new(case);
        let f = engine.screenshot().unwrap();
        assert_eq!((f.width, f.height), (1280, 720));
        assert_eq!(f.seq, 1);

        engine.click(Point::new(1, 2)).unwrap();
        assert_eq!(engine.clicks, vec![Point::new(1, 2)]);

        assert!(engine.advance());
        assert!(!engine.advance());
        assert_eq!(engine.screenshot().unwrap().seq, 2);

        fs::remove_dir_all(&dir).ok();
    }

    #[test]
    fn empty_dir_is_an_error() {
        let dir = std::env::temp_dir().join("maazure_case_test_2");
        let _ = fs::remove_dir_all(&dir);
        fs::create_dir_all(&dir).unwrap();
        fs::write(dir.join("case.json"), r#"{"name":"x"}"#).unwrap();
        let err = Case::load(&dir).unwrap_err();
        assert!(err.contains("没有 PNG 帧"), "{err}");
        fs::remove_dir_all(&dir).ok();
    }
}
