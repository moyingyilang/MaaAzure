//! `--capture-window <pkg> [--out frame.png]`：抓一帧「只有游戏窗口」的画面。
//!
//! 用途：
//! 1. 验证窗口定位 + 裁切 + 缩放这条链对不对（尺寸、亮度、颜色数）
//! 2. 给 golden corpus 录制备用（真机帧）

use maazure_runner::game_window;

/// 执行。
pub fn run_capture(args: &[String]) -> std::process::ExitCode {
    let mut package = None;
    let mut out = "frame.png".to_string();
    let mut use_su = true;

    let mut it = args.iter();
    while let Some(a) = it.next() {
        match a.as_str() {
            "--out" => out = it.next().cloned().unwrap_or(out),
            "--no-su" => use_su = false,
            other if other.starts_with("--") => return usage(&format!("未知参数 {other}")),
            other => package = Some(other.to_string()),
        }
    }

    let Some(package) = package else {
        return usage("缺少包名");
    };

    let rect = match game_window::resolve(&package, use_su) {
        Ok(r) => r,
        Err(e) => {
            eprintln!("定位窗口失败: {e}");
            return std::process::ExitCode::FAILURE;
        }
    };
    println!(
        "窗口物理矩形  left={} top={} {}x{}",
        rect.left, rect.top, rect.width, rect.height
    );
    println!(
        "换算检查      1280x720 中心点 → 屏幕 {:?}",
        rect.to_screen(640, 360)
    );

    let full_path = "/data/local/tmp/maazure-full.png";
    let screencap = if use_su {
        format!("su -c 'screencap -p {full_path}'")
    } else {
        format!("screencap -p {full_path}")
    };
    if !maazure_runner::android_display::run_shell(&screencap) {
        eprintln!("截图失败（screencap）");
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
        eprintln!("裁切/缩放失败");
        return std::process::ExitCode::FAILURE;
    };

    // 统计一下：全黑说明裁错地方了
    let img = image::load_from_memory(&cropped).expect("刚生成的 PNG 一定能解码");
    let rgb = img.to_rgb8();
    let mut sum: u64 = 0;
    let mut colors = std::collections::HashSet::new();
    for p in rgb.pixels() {
        sum += p[0] as u64 + p[1] as u64 + p[2] as u64;
        if colors.len() < 4096 {
            colors.insert((p[0], p[1], p[2]));
        }
    }
    let n = (rgb.width() * rgb.height()) as u64 * 3;
    println!("裁切后尺寸    {}x{}", rgb.width(), rgb.height());
    println!("平均亮度      {:.1}/255", sum as f64 / n as f64);
    println!(
        "不同颜色      {}{}",
        colors.len(),
        if colors.len() >= 4096 {
            " (已封顶)"
        } else {
            ""
        }
    );

    match std::fs::write(&out, &cropped) {
        Ok(()) => {
            println!("已保存        {out}（{} 字节）", cropped.len());
            std::process::ExitCode::SUCCESS
        }
        Err(e) => {
            eprintln!("写 {out} 失败: {e}");
            std::process::ExitCode::FAILURE
        }
    }
}

fn usage(msg: &str) -> std::process::ExitCode {
    eprintln!("{msg}\n用法: maazure-daemon --capture-window <包名> [--out frame.png] [--no-su]");
    std::process::ExitCode::from(2)
}
