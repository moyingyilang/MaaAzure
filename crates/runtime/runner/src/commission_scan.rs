//! 委托列表的跨帧累积器。
//!
//! 上游 `_commission_scan_list` 的做法是「OCR 一屏 → 去重合并 → 滑动 → 重复」，
//! 本质是**跨帧累积一张列表**。MaaFW 的节点是无状态的，做不了这件事，
//! 所以累积逻辑放在 Rust 侧（见 docs/4-migration/4.6）。
//!
//! 这个模块只做纯数据：给一帧 OCR 出来的若干条文本，判断有没有新条目、
//! 累积总数是多少。不碰图像、不碰框架，因此可以单独单测。

use std::collections::HashMap;

/// 一张委托卡在列表里的位置与内容。
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ScannedCommission {
    /// 卡片在屏幕上的纵向中心（用于去重时容差比较）
    pub y: i32,
    /// OCR 出来的原始文本（名字/剩余时间的组合）
    pub text: String,
}

/// 跨帧累积器。
///
/// 去重规则：文本相同且纵向位置相差不超过 `tolerance`，视为同一张卡。
/// 只按文本去重会误合并「同名但不同位置」的卡；只按位置去重会误合并
/// 滑动后落到同一坐标的另一张卡。
#[derive(Debug)]
pub struct ScanAccumulator {
    entries: Vec<ScannedCommission>,
    tolerance: i32,
    /// 已经见过多少条（含去重掉的）
    seen_frames: usize,
}

impl Default for ScanAccumulator {
    fn default() -> Self {
        Self::new(12)
    }
}

impl ScanAccumulator {
    /// 用给定的纵坐标容差创建累积器。
    pub fn new(tolerance: i32) -> Self {
        Self {
            entries: Vec::new(),
            tolerance,
            seen_frames: 0,
        }
    }

    /// 喂一帧的识别结果，返回这一帧里**新出现**的条目数。
    ///
    /// 返回 0 表示这一屏没有新卡——对 Pipeline 来说就是「滑到底了」。
    pub fn observe(&mut self, frame: &[ScannedCommission]) -> usize {
        self.seen_frames += 1;
        let mut added = 0;
        for cand in frame {
            let dup = self.entries.iter().any(|e| {
                e.text == cand.text && (e.y - cand.y).abs() <= self.tolerance
            });
            if !dup {
                self.entries.push(cand.clone());
                added += 1;
            }
        }
        added
    }

    /// 已累积的条目数。
    pub fn len(&self) -> usize {
        self.entries.len()
    }

    /// 是否还没有任何条目。
    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }

    /// 已累积的全部条目。
    pub fn entries(&self) -> &[ScannedCommission] {
        &self.entries
    }

    /// 一共喂过多少帧。
    pub fn seen_frames(&self) -> usize {
        self.seen_frames
    }

    /// 按文本统计每个名字出现几次（同名不同位置的卡会累加）。
    pub fn name_counts(&self) -> HashMap<&str, usize> {
        let mut m = HashMap::new();
        for e in &self.entries {
            *m.entry(e.text.as_str()).or_insert(0) += 1;
        }
        m
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn c(y: i32, text: &str) -> ScannedCommission {
        ScannedCommission {
            y,
            text: text.to_string(),
        }
    }

    #[test]
    fn 首帧全部是新的() {
        let mut acc = ScanAccumulator::default();
        assert_eq!(acc.observe(&[c(100, "A"), c(200, "B")]), 2);
        assert_eq!(acc.len(), 2);
    }

    #[test]
    fn 同一帧重复喂不会重复计数() {
        let mut acc = ScanAccumulator::default();
        acc.observe(&[c(100, "A")]);
        assert_eq!(acc.observe(&[c(100, "A")]), 0);
        assert_eq!(acc.len(), 1);
    }

    #[test]
    fn 滑动后位置变化视为新卡() {
        let mut acc = ScanAccumulator::default();
        acc.observe(&[c(300, "A")]);
        // 滑上去之后同一张卡跑到 y=100，位移超过容差 → 当作新条目
        assert_eq!(acc.observe(&[c(100, "A")]), 1);
        assert_eq!(acc.len(), 2);
    }

    #[test]
    fn 容差内的小位移仍然去重() {
        let mut acc = ScanAccumulator::default();
        acc.observe(&[c(300, "A")]);
        assert_eq!(acc.observe(&[c(305, "A")]), 0);
    }

    #[test]
    fn 滑到底时返回零() {
        let mut acc = ScanAccumulator::default();
        acc.observe(&[c(100, "A"), c(200, "B")]);
        // 第二屏只有已见过的卡
        assert_eq!(acc.observe(&[c(100, "A"), c(200, "B")]), 0);
        assert_eq!(acc.seen_frames(), 2);
    }

    #[test]
    fn 同名不同位置分别计数() {
        let mut acc = ScanAccumulator::default();
        acc.observe(&[c(100, "A"), c(400, "A")]);
        assert_eq!(acc.len(), 2);
        assert_eq!(acc.name_counts().get("A"), Some(&2));
    }
}

// ── 行检测（照上游 lines_detect 实现）─────────────────────────────────────
//
// 上游用「委托列表左侧那条白色分割线」定位每张卡：
//   crop(image, (597, 0, 619, 720)) → 逐行灰度均值 → find_peaks(height>200, distance>100)
//   → 丢掉 y <= 67+117 的头部 → 每个峰就是一张卡的**底边**
// 卡片高度 117，列表头部 67。

/// 委托列表分割线所在的横向区域（上游 `(597, 0, 619, 720)`）。
pub const LINE_X0: usize = 597;
/// 同上，右边界（不含）。
pub const LINE_X1: usize = 619;
/// 单张委托卡片的高度（上游注释：117）。
pub const CARD_HEIGHT: i32 = 117;
/// 列表头部高度（上游注释：67）。
pub const LIST_HEADER: i32 = 67;
/// 灰度峰值阈值（上游 `height: 200`）。
pub const PEAK_HEIGHT: f32 = 200.0;
/// 相邻峰的最小距离（上游 `distance: 100`）。
pub const PEAK_DISTANCE: usize = 100;

/// 从「每行灰度均值」里找分割线，返回每张卡的**底边 y**。
///
/// 这是上游 `lines_detect` 的等价实现：阈值 + 局部极大 + 最小间距，
/// 不需要 scipy。输入是整屏高度 720 的逐行均值。
pub fn detect_lines(row_means: &[f32]) -> Vec<i32> {
    let mut peaks: Vec<usize> = Vec::new();
    for y in 1..row_means.len().saturating_sub(1) {
        let v = row_means[y];
        if v < PEAK_HEIGHT {
            continue;
        }
        // 局部极大（左右都不比自己亮）
        if v >= row_means[y - 1] && v >= row_means[y + 1] {
            match peaks.last() {
                Some(&prev) if y - prev < PEAK_DISTANCE => {
                    // 间距太近，留更亮的那个
                    if v > row_means[prev] {
                        peaks.pop();
                        peaks.push(y);
                    }
                }
                _ => peaks.push(y),
            }
        }
    }
    let header_limit = LIST_HEADER + CARD_HEIGHT;
    peaks
        .into_iter()
        .map(|y| y as i32)
        .filter(|y| *y > header_limit)
        .collect()
}

/// 由底边 y 还原卡片的纵向范围 `[top, bottom]`。
pub fn card_bounds(bottom: i32) -> (i32, i32) {
    (bottom - CARD_HEIGHT, bottom)
}

#[cfg(test)]
mod line_tests {
    use super::*;

    /// 造一列「黑底 + 若干白色亮线」的行均值。
    fn rows_with_lines(h: usize, lines: &[usize]) -> Vec<f32> {
        let mut v = vec![40.0f32; h];
        for &y in lines {
            v[y] = 250.0;
            // 让峰成为单点局部极大
            if y > 0 {
                v[y - 1] = 120.0;
            }
            if y + 1 < h {
                v[y + 1] = 120.0;
            }
        }
        v
    }

    #[test]
    fn 识别三条分割线() {
        let rows = rows_with_lines(720, &[250, 367, 484]);
        assert_eq!(detect_lines(&rows), vec![250, 367, 484]);
    }

    #[test]
    fn 丢掉列表头部() {
        // 150 < 67+117=184，应被过滤
        let rows = rows_with_lines(720, &[150, 300]);
        assert_eq!(detect_lines(&rows), vec![300]);
    }

    #[test]
    fn 间距过近只留更亮的() {
        // 间距 60 < 100：scipy 的 find_peaks(distance=100) 保留**更亮**的那个，
        // 亮度相同时保留靠前的。这里造一明一暗，验证保留的是亮的那个。
        let mut rows = rows_with_lines(720, &[300, 360]);
        rows[360] = 255.0;
        assert_eq!(detect_lines(&rows), vec![360]);
    }

    #[test]
    fn 卡片高度是117() {
        assert_eq!(card_bounds(400), (283, 400));
    }

    #[test]
    fn 没有线时返回空() {
        let rows = rows_with_lines(720, &[]);
        assert!(detect_lines(&rows).is_empty());
    }
}

// ── 卡片解析区域（照上游 Commission.__init__ 的 area）──────────────────────
//
// 上游：
//   self.y = y                      # 分割线底边
//   self.area = (188, y - 119, 1199, y)
// 即 x∈[188,1199)、y∈[y-119, y)。注意卡高在解析时用 119，而行检测用的是 117
// （上游两处常量不同，不是笔误：行检测留 2px 余量避免把上一张的线算进来）。

/// 解析区域的左边界。
pub const CARD_X0: i32 = 188;
/// 解析区域的右边界（不含）。
pub const CARD_X1: i32 = 1199;
/// 解析区域相对分割线的高度（上游 `y - 119`）。
pub const CARD_PARSE_SPAN: i32 = 119;

/// 由分割线底边 y 得到该委托的解析区域，返回 MaaFramework 的 `[x, y, w, h]`。
pub fn card_area(bottom: i32) -> [i32; 4] {
    [
        CARD_X0,
        bottom - CARD_PARSE_SPAN,
        CARD_X1 - CARD_X0,
        CARD_PARSE_SPAN,
    ]
}

#[cfg(test)]
mod card_area_tests {
    use super::*;

    #[test]
    fn 解析区域与上游一致() {
        // y=400 → (188, 281, 1199, 400) → [188, 281, 1011, 119]
        assert_eq!(card_area(400), [188, 281, 1011, 119]);
    }

    #[test]
    fn 宽度是右边界减左边界() {
        let a = card_area(500);
        assert_eq!(a[0] + a[2], CARD_X1);
        assert_eq!(a[1] + a[3], 500);
    }
}

// ── 行均值（上游 rgb2gray + np.mean(axis=1) 的等价实现）──────────────────
//
// 上游：
//   color_height = np.mean(rgb2gray(crop(image, (597, 0, 619, 720))), axis=1)
//
// 这里只做纯计算：给定原始字节、宽高、stride、每像素字节数与通道顺序，
// 算出 x∈[x0,x1) 这条竖带的逐行灰度均值。不碰框架，可单测。

/// 像素通道顺序。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PixelOrder {
    /// R, G, B
    Rgb,
    /// B, G, R
    Bgr,
    /// R, G, B, A（A 忽略）
    Rgba,
}

/// 取竖带 `x∈[x0,x1)` 的逐行灰度均值。
///
/// * `data`   —— 整帧原始字节
/// * `height` —— 帧高
/// * `stride` —— 每行字节数（可能大于 `width * bpp`，有 padding 时必须按它走）
/// * `bpp`    —— 每像素字节数
pub fn row_means(
    data: &[u8],
    height: usize,
    stride: usize,
    bpp: usize,
    order: PixelOrder,
    x0: usize,
    x1: usize,
) -> Vec<f32> {
    let mut out = Vec::with_capacity(height);
    if bpp < 3 || x1 <= x0 {
        return vec![0.0; height];
    }
    let (ri, gi, bi) = match order {
        PixelOrder::Rgb | PixelOrder::Rgba => (0, 1, 2),
        PixelOrder::Bgr => (2, 1, 0),
    };
    let n = (x1 - x0) as f32;
    for y in 0..height {
        let base = y * stride;
        let mut sum = 0.0f32;
        for x in x0..x1 {
            let p = base + x * bpp;
            if p + 2 >= data.len() {
                break;
            }
            let r = data[p + ri] as f32;
            let g = data[p + gi] as f32;
            let b = data[p + bi] as f32;
            // 与上游 rgb2gray 一致的权重
            sum += 0.299 * r + 0.587 * g + 0.114 * b;
        }
        out.push(sum / n);
    }
    out
}

#[cfg(test)]
mod row_means_tests {
    use super::*;

    #[test]
    fn 纯白竖带得到255() {
        let (w, h) = (10usize, 4usize);
        let data = vec![255u8; w * h * 3];
        let m = row_means(&data, h, w * 3, 3, PixelOrder::Rgb, 0, w);
        assert!(m.iter().all(|v| (v - 255.0).abs() < 0.01));
    }

    #[test]
    fn 纯黑竖带得到0() {
        let (w, h) = (8usize, 3usize);
        let data = vec![0u8; w * h * 3];
        let m = row_means(&data, h, w * 3, 3, PixelOrder::Rgb, 0, w);
        assert!(m.iter().all(|v| v.abs() < 0.01));
    }

    #[test]
    fn 只有竖带内的列参与计算() {
        // 左侧白、右侧黑；只取右半边应得 0
        let (w, h) = (8usize, 2usize);
        let mut data = vec![0u8; w * h * 3];
        for y in 0..h {
            for x in 0..4 {
                let p = (y * w + x) * 3;
                data[p] = 255;
                data[p + 1] = 255;
                data[p + 2] = 255;
            }
        }
        let left = row_means(&data, h, w * 3, 3, PixelOrder::Rgb, 0, 4);
        let right = row_means(&data, h, w * 3, 3, PixelOrder::Rgb, 4, 8);
        assert!(left.iter().all(|v| (v - 255.0).abs() < 0.01));
        assert!(right.iter().all(|v| v.abs() < 0.01));
    }

    #[test]
    fn 支持stride_padding() {
        // 每行多 4 字节 padding，结果应与无 padding 相同
        let (w, h, pad) = (6usize, 2usize, 4usize);
        let stride = w * 3 + pad;
        let mut data = vec![0u8; stride * h];
        for y in 0..h {
            for x in 0..w {
                let p = y * stride + x * 3;
                data[p] = 200;
                data[p + 1] = 200;
                data[p + 2] = 200;
            }
        }
        let m = row_means(&data, h, stride, 3, PixelOrder::Rgb, 0, w);
        assert!(m.iter().all(|v| (v - 200.0).abs() < 0.01), "{m:?}");
    }

    #[test]
    fn bgr顺序能正确取灰() {
        // 纯蓝 (0,0,255) 在 BGR 下 B=255 → 灰度 = 0.114*255
        let (w, h) = (2usize, 1usize);
        let data = vec![255u8, 0, 0, 255, 0, 0];
        let m = row_means(&data, h, w * 3, 3, PixelOrder::Bgr, 0, w);
        assert!((m[0] - 0.114 * 255.0).abs() < 0.5, "{}", m[0]);
    }
}
