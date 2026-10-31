//! `--record-window <pkg>`：把游戏窗口的画面录成 golden corpus。
//!
//! 与 `--capture-window` 的区别是它**连续录**：每隔一段时间抓一帧，直到 Ctrl-C 或
//! 达到 `--frames N`。录出来的目录结构与 `record_golden.py` 一致：
//!
//! ```text
//! corpus/<name>/
//! ├── 0001.png 0002.png ...
//! └── case.json     { name, source_rev, task, server, resolution, frames, outcome }
//! ```
//!
//! 于是设备上录的语料可以直接喂 `--corpus-run`，与合成帧走同一条断言路径。

use std::path::PathBuf;

use maazure_runner::game_window;

/// 执行。
pub fn run_record(args: &[String]) -> std::process::ExitCode {
    let mut package = None;
    let mut out_dir: Option<PathBuf> = None;
    let mut frames = 3usize;
    let mut interval_ms = 1200u64;
    let mut task = String::new();
    let mut server = "cn".to_string();
    let mut use_su = true;

    let mut it = args.iter();
    while let Some(a) = it.next() {
        let mut next = |what: &str| -> String {
            it.next()
                .cloned()
                .unwrap_or_else(|| panic!("{what} 缺参数"))
        };
        match a.as_str() {
            "--out" => out_dir = Some(PathBuf::from(next("--out"))),
            "--frames" => frames = next("--frames").parse().unwrap_or(frames),
            "--interval-ms" => interval_ms = next("--interval-ms").parse().unwrap_or(interval_ms),
            "--task" => task = next("--task"),
            "--server" => server = next("--server"),
            "--no-su" => use_su = false,
            other if other.starts_with("--") => {
                eprintln!("未知参数 {other}");
                return std::process::ExitCode::from(2);
            }
            other => package = Some(other.to_string()),
        }
    }

    let (Some(package), Some(out_dir)) = (package, out_dir) else {
        eprintln!(
            "用法: maazure-daemon --record-window <包名> --out corpus/<名字> [--frames 3] [--task X]"
        );
        return std::process::ExitCode::from(2);
    };

    if let Err(e) = std::fs::create_dir_all(&out_dir) {
        eprintln!("建目录失败: {e}");
        return std::process::ExitCode::FAILURE;
    }

    let mut saved = 0usize;
    for i in 0..frames {
        // 每帧都重新定位窗口：用户可能正在拖动它
        let rect = match game_window::resolve(&package, use_su) {
            Ok(r) => r,
            Err(e) => {
                eprintln!("第 {} 帧定位窗口失败: {e}", i + 1);
                return std::process::ExitCode::FAILURE;
            }
        };
        let full_path = "/data/local/tmp/maazure-full.png";
        let cmd = if use_su {
            format!("su -c 'screencap -p {full_path}'")
        } else {
            format!("screencap -p {full_path}")
        };
        if !maazure_runner::android_display::run_shell(&cmd) {
            eprintln!("第 {} 帧截图失败", i + 1);
            return std::process::ExitCode::FAILURE;
        }
        let full = match std::fs::read(full_path) {
            Ok(b) => b,
            Err(e) => {
                eprintln!("读截图失败: {e}");
                return std::process::ExitCode::FAILURE;
            }
        };
        let Some(cropped) = game_window::crop_resize_to_720p(&full, &rect) else {
            eprintln!("第 {} 帧裁切失败", i + 1);
            return std::process::ExitCode::FAILURE;
        };
        let path = out_dir.join(format!("{:04}.png", i + 1));
        if let Err(e) = std::fs::write(&path, &cropped) {
            eprintln!("写 {} 失败: {e}", path.display());
            return std::process::ExitCode::FAILURE;
        }
        saved += 1;
        println!(
            "第 {} 帧  {}（窗口 {}x{} @ {},{}）",
            i + 1,
            path.display(),
            rect.width,
            rect.height,
            rect.left,
            rect.top
        );
        if i + 1 < frames {
            std::thread::sleep(std::time::Duration::from_millis(interval_ms));
        }
    }

    let case = serde_json::json!({
        "name": out_dir.file_name().and_then(|n| n.to_str()).unwrap_or("recorded"),
        "source_rev": "device",
        "task": task,
        "server": server,
        "resolution": [1280, 720],
        "frames": saved,
        "outcome": "",
        "note": "由 --record-window 在真机上录制；outcome 留空表示录制时没有跑 Pipeline",
    });
    let text = match serde_json::to_string_pretty(&case) {
        Ok(t) => t,
        Err(e) => {
            eprintln!("序列化 case.json 失败: {e}");
            return std::process::ExitCode::FAILURE;
        }
    };
    if let Err(e) = std::fs::write(out_dir.join("case.json"), format!("{text}\n")) {
        eprintln!("写 case.json 失败: {e}");
        return std::process::ExitCode::FAILURE;
    }

    println!("已录制 {saved} 帧 → {}", out_dir.display());
    std::process::ExitCode::SUCCESS
}
