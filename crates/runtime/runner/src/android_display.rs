//! 通过 shell 命令控制一块**指定 display**（Android 虚拟屏 / overlay display）。
//!
//! 背景：ADB `screencap` 抓的是物理屏，通知、悬浮窗、其他 App 全在里面。Android 的
//! `overlay_display_devices`（开发者选项里的「叠加视图」）能造一块独立的虚拟屏，
//! 而 `screencap -d <id>` / `input -d <id>` 可以只操作那一块屏：
//!
//! ```bash
//! # 造一块 1280x720 的虚拟屏（root）
//! settings put global overlay_display_devices "1280x720/240"
//! dumpsys SurfaceFlinger --display-id      # 拿到 display id
//! # 把游戏启动到那块屏上
//! am start --display <id> -n com.bilibili.azurlane/.MainActivity
//! ```
//!
//! 于是「截屏干扰」从根上消失：抓的是虚拟屏，物理屏上你照常刷手机。
//!
//! 这个控制器**不做任何图像处理**，只负责三件事：拿 PNG 字节、注入点击、注入滑动。

use std::process::Command;

/// 命令模板里会被替换的占位符：`{id}` `{x}` `{y}` `{x1}` `{y1}` `{x2}` `{y2}` `{ms}` `{tmp}`
#[derive(Debug, Clone)]
pub struct DisplayCommands {
    /// display id（`dumpsys SurfaceFlinger --display-id` 里的数字）
    pub display_id: String,
    /// 截图命令模板：把 PNG 写到 `{tmp}`
    pub screencap: String,
    /// 点击命令模板
    pub click: String,
    /// 滑动命令模板
    pub swipe: String,
    /// 临时文件路径（每次截图覆写）
    pub tmp_png: String,
    /// 是否用 `su -c` 包裹（Termux 里 dumpsys/screencap -d 需要 root）
    pub use_su: bool,
}

impl Default for DisplayCommands {
    fn default() -> Self {
        Self {
            display_id: String::new(),
            screencap: "screencap -d {id} -p {tmp}".to_string(),
            click: "input -d {id} tap {x} {y}".to_string(),
            swipe: "input -d {id} swipe {x1} {y1} {x2} {y2} {ms}".to_string(),
            tmp_png: "/data/local/tmp/maazure-frame.png".to_string(),
            use_su: false,
        }
    }
}

impl DisplayCommands {
    /// 用给定的 display id 构造默认命令集。
    pub fn new(display_id: impl Into<String>) -> Self {
        Self {
            display_id: display_id.into(),
            ..Default::default()
        }
    }

    /// 需要 root 时（Termux 里默认要）。
    pub fn with_su(mut self) -> Self {
        self.use_su = true;
        self
    }

    /// 按 `use_su` 包裹一条命令。
    fn wrap(&self, cmd: &str) -> String {
        if self.use_su {
            format!("su -c {}", shell_quote(cmd))
        } else {
            cmd.to_string()
        }
    }

    /// 渲染截图命令。
    pub fn render_screencap(&self) -> String {
        let raw = self
            .screencap
            .replace("{id}", &self.display_id)
            .replace("{tmp}", &self.tmp_png);
        self.wrap(&raw)
    }

    /// 渲染点击命令。
    pub fn render_click(&self, x: i32, y: i32) -> String {
        let raw = self
            .click
            .replace("{id}", &self.display_id)
            .replace("{x}", &x.to_string())
            .replace("{y}", &y.to_string());
        self.wrap(&raw)
    }

    /// 渲染滑动命令。
    pub fn render_swipe(&self, x1: i32, y1: i32, x2: i32, y2: i32, ms: i32) -> String {
        let raw = self
            .swipe
            .replace("{id}", &self.display_id)
            .replace("{x1}", &x1.to_string())
            .replace("{y1}", &y1.to_string())
            .replace("{x2}", &x2.to_string())
            .replace("{y2}", &y2.to_string())
            .replace("{ms}", &ms.to_string());
        self.wrap(&raw)
    }
}

/// 单引号包裹，内部单引号用 `'\''` 逃逸。
fn shell_quote(s: &str) -> String {
    format!("'{}'", s.replace('\'', "'\\''"))
}

/// 跑一条 shell 命令，返回是否成功（stderr 不打印，避免每帧刷屏）。
pub fn run_shell(cmd: &str) -> bool {
    Command::new("sh")
        .arg("-c")
        .arg(cmd)
        .stdout(std::process::Stdio::null())
        .stderr(std::process::Stdio::null())
        .status()
        .map(|s| s.success())
        .unwrap_or(false)
}

/// 截一张 PNG：跑截图命令 → 读临时文件。
pub fn capture_png(cmds: &DisplayCommands) -> Option<Vec<u8>> {
    if !run_shell(&cmds.render_screencap()) {
        return None;
    }
    std::fs::read(&cmds.tmp_png)
        .ok()
        .filter(|b| b.starts_with(&[0x89, b'P', b'N', b'G']))
}

/// 列出所有 display（`dumpsys SurfaceFlinger --display-id`）。
pub fn list_displays(use_su: bool) -> Vec<(String, String)> {
    let cmd = if use_su {
        "su -c 'dumpsys SurfaceFlinger --display-id'"
    } else {
        "dumpsys SurfaceFlinger --display-id"
    };
    let out = Command::new("sh").arg("-c").arg(cmd).output().ok();
    let Some(out) = out else { return Vec::new() };
    let text = String::from_utf8_lossy(&out.stdout);
    text.lines()
        .filter_map(|line| {
            // Display 11529215049488481477 (Virtual display): displayName="叠加视图 #1"
            let rest = line.strip_prefix("Display ")?;
            let mut it = rest.splitn(2, ' ');
            let id = it.next()?.to_string();
            let desc = it.next().unwrap_or("").to_string();
            Some((id, desc))
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn renders_default_commands() {
        let c = DisplayCommands::new("11529215049488481477");
        assert_eq!(
            c.render_screencap(),
            "screencap -d 11529215049488481477 -p /data/local/tmp/maazure-frame.png"
        );
        assert_eq!(
            c.render_click(10, 20),
            "input -d 11529215049488481477 tap 10 20"
        );
        assert_eq!(
            c.render_swipe(1, 2, 3, 4, 300),
            "input -d 11529215049488481477 swipe 1 2 3 4 300"
        );
    }

    #[test]
    fn custom_templates_are_honoured() {
        let c = DisplayCommands {
            display_id: "7".into(),
            screencap: "screencap -d {id} -p {tmp}".into(),
            click: "input -d {id} tap {x} {y}".into(),
            swipe: String::new(),
            tmp_png: "/tmp/f.png".into(),
            use_su: true,
        };
        assert_eq!(c.render_screencap(), "su -c 'screencap -d 7 -p /tmp/f.png'");
        assert_eq!(c.render_click(1, 2), "su -c 'input -d 7 tap 1 2'");
    }
}
