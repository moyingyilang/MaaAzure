# 文档树

文档按**层**组织，每一层只回答一层的问题；越往下越具体。
读的时候按路径走，不要跳层——上层的结论是下层的约束。

```
docs/
├── 1-foundation/     为什么这样切：决策、边界、术语
├── 2-architecture/   切成什么形状：分层、运行时、模块映射、FFI
├── 3-authoring/      怎么写：Pipeline 规范、节点树、完整示例
├── 4-migration/      怎么搬：图片、资源、战役数据、配置
├── 5-engineering/    怎么建：构建、测试、发布
└── 6-plan/           什么时候做：路线图、任务树、风险
```

| 层 | 文档 | 一句话 |
|---|---|---|
| 1 | [1.1 决策](1-foundation/1.1-decisions.md) | 12 条架构决策，每条都带代价 |
| 1 | [1.2 边界](1-foundation/1.2-boundaries.md) | 什么东西必须写成哪一类，判据是三条 |
| 1 | [1.3 术语](1-foundation/1.3-glossary.md) | 旧名 → 新名对照表 |
| 2 | [2.1 分层](2-architecture/2.1-layers.md) | 四层 crate 树与依赖方向 |
| 2 | [2.2 运行时](2-architecture/2.2-runtime.md) | 进程/线程模型与一帧时序 |
| 2 | [2.3 模块映射](2-architecture/2.3-module-tree.md) | 旧 288k 行 → 新归属的树 |
| 2 | [2.4 FFI](2-architecture/2.4-ffi.md) | 唯一接触 MaaFramework 的文件 |
| 3 | [3.1 Pipeline 规范](3-authoring/3.1-pipeline-spec.md) | 命名、锚点、文件组织、校验 |
| 3 | [3.2 节点树](3-authoring/3.2-node-tree.md) | 节点按领域分树，谁归谁 |
| 3 | [3.3 示例](3-authoring/3.3-examples.md) | 委托（纯 JSON）与战役（JSON 外壳） |
| 4 | [4.1 图片](4-migration/4.1-images.md) | 7,786 张模板图的路径与阈值 |
| 4 | [4.2 资源](4-migration/4.2-assets.md) | 61 个 `assets.py` → 2,382 个节点骨架 |
| 4 | [4.3 战役数据](4-migration/4.3-campaign-data.md) | 1,436 个 `.py` → JSON |
| 4 | [4.4 配置](4-migration/4.4-config.md) | 2,401 项 → `interface.json`，砍到 3–8 项/任务 |
| 5 | [5.1 构建](5-engineering/5.1-build.md) | chroot + 四目标交叉 |
| 5 | [5.2 测试](5-engineering/5.2-testing.md) | golden corpus：先建网再拆楼 |
| 5 | [5.3 发布](5-engineering/5.3-release.md) | 产物树、版本号、DLL 布局 |
| 6 | [6.1 路线图](6-plan/6.1-roadmap.md) | P0–P4 与验收 |
| 6 | [6.2 任务树](6-plan/6.2-task-tree.md) | 93 个任务按 A/B/C/D 四类分树 |
| 6 | [6.3 风险](6-plan/6.3-risks.md) | 12 项风险 + 许可证分析 |
| 6 | [6.4 录制手册](6-plan/6.4-corpus-runbook.md) | 12 个用例在 Windows 上怎么录 |

## 按角色的阅读路径

| 你要做的事 | 读这条路径 |
|---|---|
| 判断这项目靠不靠谱 | 1.1 → 2.3 → 6.1 → 6.3 |
| 写一个新的任务流程 | 3.1 → 3.2 → 3.3 → 4.1（找图） |
| 写 Rust 领域逻辑 | 1.2 → 2.4 → 3.1 → 5.2（先有录像） |
| 做迁移工具 | 4.2 → 4.3 → 4.4 |
| 出包 | 5.1 → 5.3 |

## 三条铁律

1. **零 Python。** 新线任何一个 `.py` 进入 `crates/`、`resource/`、`data/` 都是 bug。
   旧仓 Python 只作为数据源与一次性录制工具。
2. **帧内无状态 → JSON；跨帧有状态 → Rust。** 判据见 [1.2](1-foundation/1.2-boundaries.md)。
3. **没有录像就不改逻辑。** 见 [5.2](5-engineering/5.2-testing.md)。

违反任一条的提交都会被 review 打回。
