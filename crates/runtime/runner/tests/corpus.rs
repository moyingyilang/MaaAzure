//! 真实语料的回放测试。
//!
//! 默认 `#[ignore]`：仓库里没有 corpus（1GB 级，走 release 附件），
//! 录完之后用 `MAZURE_CORPUS=<dir> cargo test -p maazure-runner --test corpus -- --ignored` 跑。
//!
//! 这里断言的是**语料自身的自洽性**，不是领域逻辑：
//! 帧数、决策帧号范围、动作落点是否在声明的区域内。
//! 等状态机接进来后，再在这里加「决策 vs 状态机输出」的比对。

use std::path::PathBuf;

use maazure_runner::Engine;
use maazure_runner::replay::{Case, ReplayEngine};

fn corpus_dir() -> PathBuf {
    match std::env::var("MAZURE_CORPUS") {
        // 测试进程的 cwd 是 crate 目录，相对路径要按仓库根解析
        Ok(p) if PathBuf::from(&p).is_relative() => workspace_root().join(p),
        Ok(p) => PathBuf::from(p),
        Err(_) => workspace_root().join("corpus"),
    }
}

/// `crates/runtime/runner` → 仓库根。
fn workspace_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .ancestors()
        .nth(3)
        .expect("CARGO_MANIFEST_DIR 至少三层")
        .to_path_buf()
}

#[test]
#[ignore = "需要先录制语料：MAZURE_CORPUS=<dir> cargo test -- --ignored"]
fn corpus_cases_are_self_consistent() {
    let root = corpus_dir();
    let mut cases = Vec::new();
    if let Ok(rd) = std::fs::read_dir(&root) {
        for e in rd.flatten() {
            let p = e.path();
            if p.is_dir() && p.join("case.json").is_file() {
                cases.push(p);
            }
        }
    }
    assert!(
        !cases.is_empty(),
        "{} 下没有用例目录（每个用例应含 case.json）",
        root.display()
    );
    cases.sort();

    for dir in &cases {
        let case = Case::load(dir).unwrap_or_else(|e| panic!("{}: {e}", dir.display()));
        assert!(!case.is_empty(), "{} 没有帧", dir.display());
        assert_eq!(
            case.len(),
            case.info.frames,
            "{}: case.json 说 {} 帧，实际 {} 帧",
            dir.display(),
            case.info.frames,
            case.len()
        );

        let total_frames = case.len();
        let total_decisions = case.decisions.len();
        let case_name = case.info.name.clone();
        let mut engine = ReplayEngine::new(case);
        loop {
            let frame = engine.screenshot().expect("取帧失败");
            if let Some(d) = engine.current_decision() {
                assert_eq!(
                    d.frame,
                    frame.seq,
                    "{}: 决策帧号与帧序号不一致",
                    dir.display()
                );
                if let Some(action) = &d.action
                    && let (Some(area), Some(p)) = (action.area(), action.point())
                {
                    let inside =
                        p.x >= area[0] && p.x <= area[2] && p.y >= area[1] && p.y <= area[3];
                    assert!(
                        inside,
                        "{}: 第 {} 帧的点击 {:?} 落在区域 {:?} 之外",
                        dir.display(),
                        frame.seq,
                        p,
                        area
                    );
                }
            }
            if !engine.advance() {
                break;
            }
        }

        println!("{case_name}: {total_frames} 帧 / {total_decisions} 条决策 ✓");
    }
}
