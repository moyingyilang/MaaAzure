# 文档索引

先读 [A1 架构总览](A1-architecture.md)，再按需要跳转。

| 文档 | 内容 | 什么时候读 |
|---|---|---|
| [A1-architecture.md](A1-architecture.md) | 架构总览、10 条架构决策、进程模型、数据流、模块映射表 | **所有人先读这篇** |
| [A2-pipeline-spec.md](A2-pipeline-spec.md) | Pipeline 编写规范、节点命名、anchor 约定、2 个完整示例 | 写任务流程时 |
| [A3-resource-migration.md](A3-resource-migration.md) | 模板图 / 战役数据 / 2401 个配置项如何迁移，转换器设计 | 做迁移工具时 |
| [A4-rust-design.md](A4-rust-design.md) | crate 边界、核心 trait、错误模型、并发与帧预算 | 写 Rust 代码时 |
| [A5-build-and-cross.md](A5-build-and-cross.md) | chroot 构建环境、四目标交叉编译、产物布局 | 要出包时 |
| [A6-testing.md](A6-testing.md) | golden corpus 格式、录制器、回放器、断言层次 | 写测试 / 改逻辑前 |
| [A7-roadmap.md](A7-roadmap.md) | P0–P4 里程碑、验收标准、任务迁移矩阵 | 排期与验收 |
| [A8-risks.md](A8-risks.md) | 风险登记册与对策、许可证分析 | 决策前 |

## 三条不可协商的约定

1. **零 Python。** 新线任何一个 `.py` 文件进入 `crates/`、`resource/`、`data/` 都算 bug。
   旧仓库的 Python 只作为**数据源**和**一次性录制工具**存在，不参与运行时。
2. **帧内无状态 → Pipeline；跨帧有状态 → Rust。** 判据见 [A1 §4](A1-architecture.md)。
   写了 Custom 却只是「看见就点」，是设计错误；写了 Pipeline 却在节点里做计数循环，也是设计错误。
3. **没有 golden corpus 就不改逻辑。** 见 [A6](A6-testing.md)。
   任何声称「迁移完成」的模块，必须能在 `cargo test` 里对着录像跑通。
