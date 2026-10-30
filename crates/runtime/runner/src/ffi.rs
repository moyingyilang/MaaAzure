//! MaaFramework 的实际接入层。
//!
//! **整个仓库只有这个文件可以出现 `maa_framework::`**（`docs/2-architecture/2.4-ffi.md`）。
//! 上层只看到 [`Engine`](crate::Engine) 的窄接口，MaaFW 的 `Context` / `Tasker` / `Job`
//! 一律不出这个模块。
//!
//! 编译需要 MaaFramework SDK：
//!
//! ```bash
//! # SDK 解压到 ./MAA-<platform>/，然后
//! MAA_SDK_PATH=$PWD/MAA-linux-aarch64 cargo build -p maazure-runner --features maa
//! ```

use std::path::Path;
use std::path::PathBuf;
use std::sync::atomic::{AtomicU64, Ordering};

use maa_framework::common::MaaStatus;
use maa_framework::controller::Controller;
use maa_framework::resource::Resource;
use maa_framework::tasker::Tasker;
use maa_framework::toolkit::Toolkit;
use maazure_core::{Point, TaskOutcome};

use crate::{Engine, EngineError, Frame};

/// MaaFramework 支持的引擎实现。
pub struct MaaEngine {
    controller: Controller,
    resource: Resource,
    tasker: Tasker,
    seq: AtomicU64,
    /// 最近一次截图的分辨率（MaaFW 会按 `display_short_side` 归一化）
    resolution: (u32, u32),
}

impl MaaEngine {
    /// 连上一个 ADB 设备并完成 Toolkit / Controller / Resource / Tasker 的装配。
    ///
    /// `config_json` 是 MaaFW 的 ADB 配置（截图/输入方式），传 `"{}"` 让它自动选最优。
    pub fn connect_adb(adb_path: &str, address: &str, config_json: &str) -> Result<Self, String> {
        Toolkit::init_option("./", "{}").map_err(|e| format!("Toolkit 初始化失败: {e}"))?;

        let controller = Controller::new_adb(adb_path, address, config_json, "")
            .map_err(|e| format!("创建 ADB 控制器失败: {e}"))?;
        let conn = controller
            .post_connection()
            .map_err(|e| format!("连接请求失败: {e}"))?;
        controller.wait(conn);
        if !controller.connected() {
            return Err(format!("连不上设备 {address}"));
        }

        Self::from_controller(controller)
    }

    /// 连一个 Windows 窗口（模拟器窗口 / PC 客户端）。
    ///
    /// `screencap`：`background`（FramePool|PrintWindow，后台截窗口，不受其他窗口干扰）、
    /// `foreground`、`all`、`gdi`、`framepool`、`printwindow`、`dxgi`。
    /// `input`：`post`（PostMessage，不抢鼠标）、`send`、`post_cursor`、`send_cursor`、
    /// `post_window`、`send_window`、`seize`、`anchored_touch`。
    #[cfg(target_os = "windows")]
    pub fn connect_win32(hwnd: usize, screencap: &str, input: &str) -> Result<Self, String> {
        use maa_framework::common::{Win32InputMethod, Win32ScreencapMethod};

        let sc = match screencap.to_ascii_lowercase().as_str() {
            "" | "background" => Win32ScreencapMethod::BACKGROUND,
            "foreground" => Win32ScreencapMethod::FOREGROUND,
            "all" => Win32ScreencapMethod::ALL,
            "gdi" => Win32ScreencapMethod::GDI,
            "framepool" => Win32ScreencapMethod::FRAME_POOL,
            "printwindow" => Win32ScreencapMethod::PRINT_WINDOW,
            "dxgi" => Win32ScreencapMethod::DXGI_DESKTOP_DUP_WINDOW,
            other => return Err(format!("未知截图方式 {other}")),
        };
        let input = match input.to_ascii_lowercase().as_str() {
            "" | "post" => Win32InputMethod::POST_MESSAGE,
            "send" => Win32InputMethod::SEND_MESSAGE,
            "post_cursor" => Win32InputMethod::POST_MESSAGE_WITH_CURSOR_POS,
            "send_cursor" => Win32InputMethod::SEND_MESSAGE_WITH_CURSOR_POS,
            "post_window" => Win32InputMethod::POST_MESSAGE_WITH_WINDOW_POS,
            "send_window" => Win32InputMethod::SEND_MESSAGE_WITH_WINDOW_POS,
            "seize" => Win32InputMethod::SEIZE,
            "anchored_touch" => Win32InputMethod::ANCHORED_TOUCH,
            other => return Err(format!("未知输入方式 {other}")),
        };

        Toolkit::init_option("./", "{}").map_err(|e| format!("Toolkit 初始化失败: {e}"))?;
        let controller = Controller::new_win32(
            hwnd as *mut std::ffi::c_void,
            sc,
            input,
            Win32InputMethod::POST_MESSAGE,
        )
        .map_err(|e| format!("创建 Win32 控制器失败: {e}"))?;
        let conn = controller
            .post_connection()
            .map_err(|e| format!("连接请求失败: {e}"))?;
        controller.wait(conn);
        if !controller.connected() {
            return Err(format!("连不上窗口 hwnd={hwnd}"));
        }
        Self::from_controller(controller)
    }

    /// 列出可控制的桌面窗口（Windows/macOS/Linux）。
    pub fn list_windows() -> Result<Vec<(usize, String, String)>, String> {
        Toolkit::init_option("./", "{}").map_err(|e| format!("Toolkit 初始化失败: {e}"))?;
        let list = Toolkit::find_desktop_windows()
            .map_err(|e| format!("枚举窗口失败: {e}"))?;
        Ok(list
            .into_iter()
            .map(|w| (w.hwnd, w.class_name, w.window_name))
            .collect())
    }

    /// 用任意已创建的控制器装配引擎（合成帧控制器走这条路）。
    pub fn from_controller(controller: Controller) -> Result<Self, String> {
        let resource = Resource::new().map_err(|e| format!("创建 Resource 失败: {e}"))?;
        let tasker = Tasker::new().map_err(|e| format!("创建 Tasker 失败: {e}"))?;
        tasker
            .bind_resource(&resource)
            .map_err(|e| format!("bind_resource 失败: {e}"))?;
        tasker
            .bind_controller(&controller)
            .map_err(|e| format!("bind_controller 失败: {e}"))?;
        if !tasker.inited() {
            return Err("tasker 未完成初始化".to_string());
        }

        let resolution = controller
            .resolution()
            .map(|(w, h)| (w as u32, h as u32))
            .unwrap_or((1280, 720));

        Ok(Self {
            controller,
            resource,
            tasker,
            seq: AtomicU64::new(0),
            resolution,
        })
    }

    /// 用「只读游戏窗口」的控制器装配引擎：整屏截图 → 裁窗口 → 1280×720。
    ///
    /// 游戏在 freeform 浮窗里时，这样只读游戏那块区域，通知/其他 App 都不进来。
    #[cfg(feature = "maa")]
    pub fn from_game_window(package: &str, use_su: bool) -> Result<Self, String> {
        use crate::game_window;

        let rect = game_window::resolve(package, use_su)?;

        struct WindowController {
            package: String,
            rect: game_window::WindowRect,
            use_su: bool,
            tmp: String,
            clicks: std::sync::Mutex<Vec<(i32, i32)>>,
        }

        impl WindowController {
            fn shell(&self, cmd: &str) -> bool {
                if self.use_su {
                    crate::android_display::run_shell(&format!("su -c '{}'", cmd.replace('\'', "'\\''")))
                } else {
                    crate::android_display::run_shell(cmd)
                }
            }
        }

        impl maa_framework::custom_controller::CustomControllerCallback for WindowController {
            fn connect(&self) -> bool {
                self.screencap().is_some()
            }

            fn screencap(&self) -> Option<Vec<u8>> {
                if !self.shell(&format!("screencap -p {}", self.tmp)) {
                    return None;
                }
                let full = std::fs::read(&self.tmp).ok()?;
                game_window::crop_resize_to_720p(&full, &self.rect)
            }

            fn click(&self, x: i32, y: i32) -> bool {
                if let Ok(mut c) = self.clicks.lock() {
                    c.push((x, y));
                }
                let (sx, sy) = self.rect.to_screen(x, y);
                self.shell(&format!("input tap {sx} {sy}"))
            }

            fn swipe(&self, x1: i32, y1: i32, x2: i32, y2: i32, duration: i32) -> bool {
                let (ax, ay) = self.rect.to_screen(x1, y1);
                let (bx, by) = self.rect.to_screen(x2, y2);
                self.shell(&format!("input swipe {ax} {ay} {bx} {by} {duration}"))
            }

            fn get_info(&self) -> String {
                let n = self.clicks.lock().map(|c| c.len()).unwrap_or(0);
                format!(
                    r#"{{"package":"{}","rect":[{},{},{},{}],"clicks":{}}}"#,
                    self.package, self.rect.left, self.rect.top, self.rect.width, self.rect.height, n
                )
            }
        }

        let controller = Controller::new_custom(WindowController {
            package: package.to_string(),
            rect,
            use_su,
            tmp: "/data/local/tmp/maazure-full.png".to_string(),
            clicks: std::sync::Mutex::new(Vec::new()),
        })
        .map_err(|e| format!("创建窗口控制器失败: {e}"))?;
        let conn = controller
            .post_connection()
            .map_err(|e| format!("窗口控制器连接失败: {e}"))?;
        controller.wait(conn);
        if !controller.connected() {
            return Err(format!("连不上 {package} 的窗口（截图或裁切失败）"));
        }
        Self::from_controller(controller)
    }

    /// 用「shell 命令控制某块 display」的控制器装配引擎（Android 虚拟屏走这条）。
    #[cfg(feature = "maa")]
    pub fn from_display_commands(
        cmds: crate::android_display::DisplayCommands,
    ) -> Result<Self, String> {
        struct DisplayController {
            cmds: crate::android_display::DisplayCommands,
            seq: AtomicU64,
        }

        impl maa_framework::custom_controller::CustomControllerCallback for DisplayController {
            fn connect(&self) -> bool {
                crate::android_display::capture_png(&self.cmds).is_some()
            }

            fn screencap(&self) -> Option<Vec<u8>> {
                crate::android_display::capture_png(&self.cmds)
            }

            fn click(&self, x: i32, y: i32) -> bool {
                let cmd = self.cmds.render_click(x, y);
                crate::android_display::run_shell(&cmd)
            }

            fn swipe(&self, x1: i32, y1: i32, x2: i32, y2: i32, duration: i32) -> bool {
                let cmd = self.cmds.render_swipe(x1, y1, x2, y2, duration);
                crate::android_display::run_shell(&cmd)
            }

            fn get_info(&self) -> String {
                format!(
                    r#"{{"display_id":"{}","frames":{}}}"#,
                    self.cmds.display_id,
                    self.seq.load(std::sync::atomic::Ordering::Relaxed)
                )
            }
        }

        let ctrl = DisplayController {
            cmds,
            seq: AtomicU64::new(0),
        };
        let controller = Controller::new_custom(ctrl)
            .map_err(|e| format!("创建 display 控制器失败: {e}"))?;
        let conn = controller
            .post_connection()
            .map_err(|e| format!("display 控制器连接失败: {e}"))?;
        controller.wait(conn);
        if !controller.connected() {
            return Err("display 控制器连不上（截图命令失败？）".to_string());
        }
        Self::from_controller(controller)
    }

    /// 用合成帧控制器装配引擎（不需要模拟器）。
    ///
    /// 这层存在的意义：让上层（daemon）完全不出现 `maa_framework` 的类型，
    /// 保持「只有 ffi.rs 能碰 MaaFW」这条规矩。
    pub fn from_synth(ctrl: crate::synth::SynthController) -> Result<Self, String> {
        let controller =
            Controller::new_custom(ctrl).map_err(|e| format!("创建合成控制器失败: {e}"))?;
        let conn = controller
            .post_connection()
            .map_err(|e| format!("合成控制器连接失败: {e}"))?;
        controller.wait(conn);
        if !controller.connected() {
            return Err("合成控制器连接后未处于已连接状态".to_string());
        }
        Self::from_controller(controller)
    }

    /// 按顺序加载资源包（`common` 先，服务器包后，后者覆盖前者）。
    pub fn load_bundles(&self, paths: &[&str]) -> Result<(), String> {
        for p in paths {
            if !Path::new(p).exists() {
                return Err(format!("资源包不存在: {p}"));
            }
            let job = self
                .resource
                .post_bundle(p)
                .map_err(|e| format!("加载 {p} 失败: {e}"))?;
            let status = job.wait();
            if status != MaaStatus::SUCCEEDED {
                return Err(format!("加载 {p} 状态异常: {status:?}"));
            }
        }
        Ok(())
    }

    /// 跑一条 Pipeline，等到结束。
    pub fn run_pipeline(&self, entry: &str, override_json: &str) -> Result<TaskOutcome, String> {
        self.run_pipeline_traced(entry, override_json)
            .map(|(outcome, _nodes)| outcome)
    }

    /// 跑一条 Pipeline，并返回**执行过的节点名序列**。
    ///
    /// 合成帧 fixture 靠这个断言「走的是预期路径」而不是「碰巧点了两次」。
    pub fn run_pipeline_traced(
        &self,
        entry: &str,
        override_json: &str,
    ) -> Result<(TaskOutcome, Vec<String>), String> {
        let job = self
            .tasker
            .post_task(entry, override_json)
            .map_err(|e| format!("提交任务 {entry} 失败: {e}"))?;
        let status = job.wait();

        let mut visited = Vec::new();
        if let Ok(Some(detail)) = job.get(false) {
            for node in detail.nodes.into_iter().flatten() {
                visited.push(node.node_name);
            }
        }

        let outcome = match status {
            MaaStatus::SUCCEEDED => TaskOutcome::Ok,
            MaaStatus::FAILED => TaskOutcome::Failed,
            _ => TaskOutcome::Recoverable,
        };
        Ok((outcome, visited))
    }

    /// 当前分辨率（归一化后）。
    pub fn resolution(&self) -> (u32, u32) {
        self.resolution
    }

    /// 注册自定义识别。`Box<dyn CustomRecognition>` 由调用方从领域层构造。
    pub fn register_recognition(
        &self,
        name: &str,
        reco: Box<dyn maa_framework::custom::CustomRecognition>,
    ) -> Result<(), String> {
        self.resource
            .register_custom_recognition(name, reco)
            .map_err(|e| format!("注册识别 {name} 失败: {e}"))
    }

    /// 注册自定义动作。
    pub fn register_action(
        &self,
        name: &str,
        action: Box<dyn maa_framework::custom::CustomAction>,
    ) -> Result<(), String> {
        self.resource
            .register_custom_action(name, action)
            .map_err(|e| format!("注册动作 {name} 失败: {e}"))
    }
}

impl Engine for MaaEngine {
    fn stopping(&self) -> bool {
        self.tasker.stopping()
    }

    fn screenshot(&self) -> Result<Frame, EngineError> {
        let id = self
            .controller
            .post_screencap()
            .map_err(|e| EngineError::Unsupported(format!("截图请求失败: {e}")))?;
        self.controller.wait(id);
        let (w, h) = self
            .controller
            .resolution()
            .map_err(|e| EngineError::Unsupported(format!("取分辨率失败: {e}")))?;
        Ok(Frame {
            seq: self.seq.fetch_add(1, Ordering::Relaxed),
            width: w as u32,
            height: h as u32,
        })
    }

    fn click(&mut self, p: Point) -> Result<(), EngineError> {
        let id = self
            .controller
            .post_click(p.x, p.y)
            .map_err(|e| EngineError::Unsupported(format!("点击失败: {e}")))?;
        self.controller.wait(id);
        Ok(())
    }

    fn swipe(&mut self, from: Point, to: Point, ms: u64) -> Result<(), EngineError> {
        let id = self
            .controller
            .post_swipe(from.x, from.y, to.x, to.y, ms as i32)
            .map_err(|e| EngineError::Unsupported(format!("滑动失败: {e}")))?;
        self.controller.wait(id);
        Ok(())
    }
}

/// 动态模式下的库初始化：从 `MAAFW_BINARY_PATH` 或仓库里的 SDK 目录找 `libMaaFramework.so`。
///
/// 静态模式（`maa` feature 且不带 dynamic）什么都不做——链接期已经绑好了。
pub fn init_framework() -> Result<(), String> {
    if !cfg!(feature = "maa-dynamic") {
        return Ok(());
    }

    let mut candidates: Vec<PathBuf> = Vec::new();
    if let Ok(dir) = std::env::var("MAAFW_BINARY_PATH") {
        let dir = PathBuf::from(dir);
        for name in LIB_NAMES {
            candidates.push(dir.join(name));
        }
        candidates.push(dir);
    }
    for sdk in ["MAA-android-aarch64", "MAA-linux-aarch64"] {
        for name in LIB_NAMES {
            candidates.push(PathBuf::from(sdk).join("bin").join(name));
        }
    }
    for name in LIB_NAMES {
        candidates.push(PathBuf::from(name));
    }

    for c in &candidates {
        if c.is_file()
            && c.file_name()
                .and_then(|n| n.to_str())
                .is_some_and(|n| n.contains("MaaFramework"))
        {
            return maa_framework::load_library(c);
        }
    }
    Err(format!(
        "找不到 libMaaFramework.so：设 MAAFW_BINARY_PATH 或在仓库里放 MAA-android-aarch64/bin（找过 {:?}）",
        candidates.first()
    ))
}

const LIB_NAMES: [&str; 3] = ["libMaaFramework.so", "MaaFramework.dll", "libMaaFramework.dylib"];
