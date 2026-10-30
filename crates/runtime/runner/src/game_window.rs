//! 「只读游戏窗口」：整屏截图 → 裁到游戏窗口 → 缩放到 1280×720。
//!
//! 为什么需要它：手机上的 `screencap` 抓的是**整块屏**，通知、悬浮窗、其他 App 都在里面。
//! 但设备若跑在 freeform（桌面模式）下，游戏本身就是一个**浮窗**，位置和大小都能查出来：
//!
//! ```text
//! dumpsys window windows → Window{... com.bilibili.azurlane/...}
//!     Frames: ... frame=[100,200][2404,1496]      ← 逻辑桌面坐标
//! wm size → Physical size: 1440x3200              ← 物理像素
//! dumpsys window displays → largest app 3200x3200 ← 逻辑桌面尺寸
//! scale = 1440 / 3200 = 0.45                      ← 桌面缩放
//! ```
//!
//! 于是物理矩形 = 逻辑矩形 × scale，裁出来缩放回 1280×720 就是干净的「只有游戏」。
//! 点击反向换算即可（1280×720 → 物理 → `input tap`）。
//!
//! 好处：游戏还是那个浮窗，用户能照常用别的 App；我们只读这个窗口。

use std::process::Command;

/// 逻辑坐标 → 物理坐标的换算，外加裁剪尺寸。
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct WindowRect {
    /// 物理像素下的左上角 x
    pub left: i32,
    /// 物理像素下的左上角 y
    pub top: i32,
    /// 物理像素下的宽
    pub width: i32,
    /// 物理像素下的高
    pub height: i32,
}

impl WindowRect {
    /// 逻辑矩形 + 缩放比 → 物理矩形。
    pub fn from_logical(l: i32, t: i32, r: i32, b: i32, scale: f32) -> Self {
        let left = (l as f32 * scale).round() as i32;
        let top = (t as f32 * scale).round() as i32;
        let right = (r as f32 * scale).round() as i32;
        let bottom = (b as f32 * scale).round() as i32;
        Self {
            left,
            top,
            width: (right - left).max(1),
            height: (bottom - top).max(1),
        }
    }

    /// 游戏逻辑坐标（1280×720）→ 屏幕物理坐标。
    pub fn to_screen(&self, x: i32, y: i32) -> (i32, i32) {
        let sx = self.left + (x as f32 * self.width as f32 / 1280.0).round() as i32;
        let sy = self.top + (y as f32 * self.height as f32 / 720.0).round() as i32;
        (sx, sy)
    }
}

/// 跑一条命令拿 stdout。
fn output(cmd: &str) -> Option<String> {
    let out = Command::new("sh").arg("-c").arg(cmd).output().ok()?;
    if !out.status.success() {
        return None;
    }
    Some(String::from_utf8_lossy(&out.stdout).to_string())
}

/// 逻辑桌面尺寸。
///
/// 桌面模式下 `dumpsys window displays` 的 `mOverrideDisplayInfo` 里会写
/// `largest app 3200 x 3200`（注意 x 两边有空格），这才是应用能用的逻辑尺寸；
/// `mBaseDisplayInfo` 里那个是物理尺寸，不能拿来算缩放。
pub fn logical_desktop(use_su: bool) -> Option<(i32, i32)> {
    // 注意：`dumpsys display` 里才有 mOverrideDisplayInfo；
    // `dumpsys window displays` 只有 init/cur/rng 那一行。
    let cmd = if use_su {
        "su -c 'dumpsys display'"
    } else {
        "dumpsys display"
    };
    let text = output(cmd)?;

    // 优先 mOverrideDisplayInfo（桌面/自由窗口模式下的真实逻辑尺寸）
    for key in ["mOverrideDisplayInfo", "mBaseDisplayInfo"] {
        let Some(idx) = text.find(key) else { continue };
        let tail = &text[idx..];
        let Some(line) = tail.lines().next() else { continue };
        if let Some(size) = parse_largest_app(line) {
            return Some(size);
        }
    }
    None
}

/// 从 `... largest app 3200 x 3200, ...` 里取出 (3200, 3200)。
fn parse_largest_app(line: &str) -> Option<(i32, i32)> {
    let rest = &line[line.find("largest app ")? + "largest app ".len()..];
    // 形如 "3200 x 3200, smallest app ..."
    let mut parts = rest.split_whitespace();
    let w: i32 = parts.next()?.parse().ok()?;
    if parts.next()? != "x" {
        return None;
    }
    let h: i32 = parts
        .next()?
        .trim_end_matches(',')
        .parse()
        .ok()?;
    Some((w, h))
}

/// 物理屏幕尺寸（`wm size`）。
pub fn physical_size(use_su: bool) -> Option<(i32, i32)> {
    let cmd = if use_su { "su -c 'wm size'" } else { "wm size" };
    let text = output(cmd)?;
    for line in text.lines() {
        if let Some(rest) = line.split("Physical size:").nth(1) {
            let (w, h) = rest.trim().split_once('x')?;
            return Some((w.parse().ok()?, h.parse().ok()?));
        }
    }
    None
}

/// 找某个包名窗口的逻辑 frame（`Frames: ... frame=[l,t][r,b]`）。
pub fn window_frame(package: &str, use_su: bool) -> Option<(i32, i32, i32, i32)> {
    let cmd = if use_su {
        "su -c 'dumpsys window windows'"
    } else {
        "dumpsys window windows"
    };
    let text = output(cmd)?;
    let mut in_window = false;
    for line in text.lines() {
        if line.contains("Window{") && line.contains(package) {
            in_window = true;
            continue;
        }
        if in_window && line.contains("Window{") {
            in_window = false;
        }
        if in_window && line.contains("frame=[") {
            let start = line.find("frame=[")? + "frame=[".len();
            let rest = &line[start..];
            let (l, rest) = rest.split_once(',')?;
            let (t, rest) = rest.split_once("][")?;
            let (r, rest) = rest.split_once(',')?;
            let (b, _) = rest.split_once(']')?;
            return Some((
                l.trim().parse().ok()?,
                t.trim().parse().ok()?,
                r.trim().parse().ok()?,
                b.trim().parse().ok()?,
            ));
        }
    }
    None
}

/// 解析出窗口的物理矩形（含桌面缩放）。
pub fn resolve(package: &str, use_su: bool) -> Result<WindowRect, String> {
    let (lw, _lh) = logical_desktop(use_su).ok_or("拿不到逻辑桌面尺寸（dumpsys window displays）")?;
    let (pw, _ph) = physical_size(use_su).ok_or("拿不到物理屏幕尺寸（wm size）")?;
    let scale = pw as f32 / lw as f32;

    let (l, t, r, b) =
        window_frame(package, use_su).ok_or_else(|| format!("找不到 {package} 的窗口 frame"))?;

    let rect = WindowRect::from_logical(l, t, r, b, scale);
    if rect.width < 64 || rect.height < 64 {
        return Err(format!(
            "窗口太小（{}x{}），可能没在前台",
            rect.width, rect.height
        ));
    }
    Ok(rect)
}

/// 整屏 PNG → 裁到窗口 → 缩放到 1280×720 的 PNG。
///
/// 缩放用 Lanczos3：模板是从 1280×720 原图裁的，缩放质量直接影响匹配率。
#[cfg(feature = "maa")]
pub fn crop_resize_to_720p(png: &[u8], rect: &WindowRect) -> Option<Vec<u8>> {
    use image::imageops::FilterType;

    let img = image::load_from_memory(png).ok()?.to_rgb8();
    let (w, h) = (img.width() as i32, img.height() as i32);
    if rect.left >= w || rect.top >= h {
        return None;
    }
    let rw = rect.width.min(w - rect.left).max(1) as u32;
    let rh = rect.height.min(h - rect.top).max(1) as u32;

    let cropped = image::imageops::crop_imm(&img, rect.left as u32, rect.top as u32, rw, rh).to_image();
    let scaled = image::imageops::resize(&cropped, 1280, 720, FilterType::Lanczos3);

    let mut out = std::io::Cursor::new(Vec::new());
    image::DynamicImage::ImageRgb8(scaled)
        .write_to(&mut out, image::ImageFormat::Png)
        .ok()?;
    Some(out.into_inner())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_largest_app_with_spaces() {
        assert_eq!(
            parse_largest_app("real 1440 x 3200, largest app 3200 x 3200, smallest app 1440 x 1440"),
            Some((3200, 3200))
        );
        assert_eq!(parse_largest_app("no size here"), None);
    }

    #[test]
    fn logical_to_physical_uses_scale() {
        // 1440 物理 / 3200 逻辑 = 0.45
        let r = WindowRect::from_logical(100, 200, 2404, 1496, 0.45);
        assert_eq!((r.left, r.top), (45, 90));
        assert_eq!((r.width, r.height), (1037, 583));
    }

    #[test]
    fn click_maps_into_window() {
        let r = WindowRect {
            left: 45,
            top: 90,
            width: 1280,
            height: 720,
        };
        assert_eq!(r.to_screen(0, 0), (45, 90));
        assert_eq!(r.to_screen(640, 360), (685, 450));
        assert_eq!(r.to_screen(1279, 719), (1324, 809));
    }

    #[test]
    fn scaled_click_maps_back() {
        // 窗口 1037x583：1280x720 坐标要按比例缩
        let r = WindowRect {
            left: 0,
            top: 0,
            width: 1037,
            height: 583,
        };
        let (x, y) = r.to_screen(640, 360);
        assert!((x - 519).abs() <= 1, "x={x}");
        assert!((y - 292).abs() <= 1, "y={y}");
    }
}
