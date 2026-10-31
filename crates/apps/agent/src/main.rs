//! MaaFwApp 的 agent 子进程：把 MaaAzure 的自定义识别注册给 Android 外壳。
//!
//! 契约（见 MaaFwApp `INTEGRATION.md`）：
//!
//! ```text
//! <executable> args... <identifier>
//! ```
//!
//! 最后一位参数是连接标识符，工作目录是资源解包根；注册完回调后
//! `MaaAgentServerStartUp(identifier)` → `MaaAgentServerJoin()` 挂起等待。
//!
//! 库是**运行期**加载的（`dynamic` 模式），路径来自外壳注入的
//! `MAAFW_BINARY_PATH`，所以这个二进制本身不链接 MaaFramework。

//! 只有开了 `maa-dynamic` 才有意义：不带这个 feature 时编译成一个说明性 stub，
//! 免得 `cargo test --workspace` 因为找不到 maa_framework 而挂掉。

#[cfg(feature = "maa-dynamic")]
fn main() -> std::process::ExitCode {
    real::main()
}

#[cfg(not(feature = "maa-dynamic"))]
fn main() -> std::process::ExitCode {
    eprintln!(
        "maazure-agent 需要 --features maa-dynamic 才能编出可用的 agent（运行期加载 libMaaFramework.so）。"
    );
    std::process::ExitCode::from(2)
}

#[cfg(feature = "maa-dynamic")]
mod real {
    use std::path::{Path, PathBuf};

    use maazure_runner::CommissionPick;

    pub fn main() -> std::process::ExitCode {
        let args: Vec<String> = std::env::args().collect();

        // 最后一个参数是 identifier；没有就报用法
        let Some(identifier) = args
            .last()
            .filter(|a| !a.starts_with('-') && args.len() > 1)
        else {
            eprintln!(
                "用法: {} <args...> <identifier>\n\
             这是 MaaFwApp 的 agent 子进程，通常由外壳拉起，不需要手动运行。",
                args.first().map(String::as_str).unwrap_or("maazure-agent")
            );
            return std::process::ExitCode::from(2);
        };

        // 1) 运行期加载 MaaFramework（dynamic 模式）
        if let Err(e) = load_framework() {
            eprintln!("加载 MaaFramework 失败: {e}");
            return std::process::ExitCode::FAILURE;
        }

        // 2) 注册自定义识别（名字必须与 Pipeline 里的 custom_recognition 一致）
        if let Err(e) = maa_framework::agent_server::AgentServer::register_custom_recognition(
            "CommissionPick",
            Box::new(CommissionPick),
        ) {
            eprintln!("注册 CommissionPick 失败: {e}");
            return std::process::ExitCode::FAILURE;
        }
        eprintln!("[maazure-agent] 已注册 CommissionPick，identifier={identifier}");

        // 3) 连上外壳并挂起
        if let Err(e) = maa_framework::agent_server::AgentServer::start_up(identifier) {
            eprintln!("AgentServer 启动失败: {e}");
            return std::process::ExitCode::FAILURE;
        }
        eprintln!("[maazure-agent] 已连接，等待任务");
        maa_framework::agent_server::AgentServer::join();
        std::process::ExitCode::SUCCESS
    }

    /// 把 `MAAFW_BINARY_PATH` 目录下的兄弟库全部 `dlopen(RTLD_GLOBAL)`。
    ///
    /// 为什么需要：Android 版把 `MaaAgentServerRegisterCustomRecognition` 这类符号放在
    /// **libMaaAgentServer.so**，而 `maa_framework::load_library` 只加载
    /// `libMaaFramework.so`。不预加载的话，注册回调时 shim 会 panic：
    /// `Function MaaAgentServerRegisterCustomRecognition not loaded`。
    fn preload_siblings() -> Result<(), String> {
        let Ok(dir) = std::env::var("MAAFW_BINARY_PATH") else {
            return Ok(());
        };
        let dir = std::path::PathBuf::from(dir);
        const SIBLINGS: [&str; 4] = [
            "libMaaAgentServer.so",
            "libMaaAgentClient.so",
            "libMaaToolkit.so",
            "libMaaUtils.so",
        ];
        for name in SIBLINGS {
            let path = dir.join(name);
            if !path.is_file() {
                continue;
            }
            let Ok(c_path) = std::ffi::CString::new(path.to_string_lossy().as_bytes()) else {
                continue;
            };
            // RTLD_NOW(2) | RTLD_GLOBAL(0x100)
            let handle = unsafe { libc::dlopen(c_path.as_ptr(), 2 | 0x100) };
            if handle.is_null() {
                let err = unsafe {
                    let e = libc::dlerror();
                    if e.is_null() {
                        "未知错误".to_string()
                    } else {
                        std::ffi::CStr::from_ptr(e).to_string_lossy().to_string()
                    }
                };
                return Err(format!("dlopen {name} 失败: {err}"));
            }
        }
        Ok(())
    }

    /// 从 `MAAFW_BINARY_PATH`（外壳注入）找到 MaaFramework 动态库并加载。
    ///
    /// Android 上外壳把 `.so` 放在它自己的 native lib 目录里，文件名是
    /// `libMaaFramework.so`；桌面端则是 `MaaFramework.dll` / `libMaaFramework.so`。
    fn load_framework() -> Result<(), String> {
        let mut candidates: Vec<PathBuf> = Vec::new();

        if let Ok(dir) = std::env::var("MAAFW_BINARY_PATH") {
            let dir = PathBuf::from(dir);
            for name in [
                "libMaaFramework.so",
                "MaaFramework.dll",
                "libMaaFramework.dylib",
            ] {
                candidates.push(dir.join(name));
            }
            candidates.push(dir);
        }
        // 兜底：当前目录
        for name in [
            "libMaaFramework.so",
            "MaaFramework.dll",
            "libMaaFramework.dylib",
        ] {
            candidates.push(PathBuf::from(name));
        }

        for c in &candidates {
            if is_library(c) {
                return maa_framework::load_library(c);
            }
        }
        Err(format!(
            "在 MAAFW_BINARY_PATH={:?} 和当前目录都没找到 MaaFramework 动态库",
            std::env::var("MAAFW_BINARY_PATH").ok()
        ))
    }

    fn is_library(p: &Path) -> bool {
        p.is_file()
            && p.file_name()
                .and_then(|n| n.to_str())
                .is_some_and(|n| n.contains("MaaFramework"))
    }
}
