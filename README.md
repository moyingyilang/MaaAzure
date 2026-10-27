# MaaAzure

**碧蓝航线自动化 · MaaFramework 技术栈 · 零 Python · AGPL-3.0**

> 从 [AzurPilot](https://github.com/wess09/AzurPilot) 重写而来：保留领域知识，换掉整条执行栈。
> 本仓库独立演化，旧仓库冻结仅作数据源与回归基线。

---

## 一句话

AzurPilot 用 288,076 行 Python 自己实现了设备层、模板匹配、OCR、A* 寻路、状态循环、
调度器和 WebUI，且**游戏逻辑零自动化测试**。MaaAzure 把这些通用问题交还给成熟内核，
自己只留游戏知识。

| 层 | 谁来做 |
|---|---|
| 截图、输入、识别、流程执行 | **MaaFramework**（C++，LGPL-3.0） |
| 任务编排 | **Pipeline JSON**（声明式 `next` 图） |
| 跨帧状态逻辑 | **Rust**（战役寻路、大世界、战斗决策） |
| 配置 schema / 多语言 / GUI | **ProjectInterface v2** |
| 调度 / HTTP / MCP | **Rust**（tokio + axum） |

没有一个 Python 文件参与运行时。

## 树状组织

仓库按**层**分树，每个文件有唯一归属。判据：**不知道碧蓝航线是什么也能写出来的代码，不属于 `domain/`。**

```
MaaAzure/
├── crates/
│   ├── foundation/          1 地基 —— 无业务概念、无 IO，可在无 SDK 机器上测试
│   │   ├── core/            错误树 / 几何 / 任务结果
│   │   └── vision/          数字纠错 / 时长解析
│   ├── domain/              2 领域 —— 只有碧蓝航线知识，纯逻辑
│   │   ├── battle/          战役 · 地图 · 战斗 · 大世界
│   │   └── daily/           日常任务的最小决策
│   ├── runtime/             3 运行时 —— 唯一接触 MaaFramework
│   │   └── runner/          Engine trait / Tasker / 调度器
│   └── apps/                4 应用 —— 只有入口
│       ├── daemon/          守护进程
│       └── xtask/           资源转换 / 校验
├── resource/
│   ├── default_pipeline.json
│   ├── pipeline/            0-common / 1-daily / 2-battle / 3-world
│   └── image/<server>/      模板图（由 AzurPilot assets/ 转换）
├── data/
│   ├── campaign/            战役地图 JSON
│   └── scheduler/           优先级与 NextRun 规则
├── docs/                    六层文档树（先读 docs/README.md）
├── interface.json           PI v2 配置与 GUI 声明
└── tools/buildkit/          Termux chroot 构建工具链
```

依赖方向严格单向：`apps → runtime → domain → foundation`。运行 `maazure-daemon --tree` 可打印同一张树。

## 文档

从 [docs/README.md](docs/README.md) 进入。六层：

| 层 | 内容 |
|---|---|
| [1-foundation](docs/1-foundation/1.1-decisions.md) | 12 条决策（带代价）、边界判据、术语对照 |
| [2-architecture](docs/2-architecture/2.1-layers.md) | 分层、运行时、模块映射树、FFI |
| [3-authoring](docs/3-authoring/3.1-pipeline-spec.md) | Pipeline 规范、节点树、两个完整示例 |
| [4-migration](docs/4-migration/4.1-images.md) | 图片、资源、战役数据、配置的迁移方案 |
| [5-engineering](docs/5-engineering/5.1-build.md) | 构建、回归测试、发布 |
| [6-plan](docs/6-plan/6.1-roadmap.md) | 路线图、任务分批、风险 |

## 状态

```
📝 设计 + 骨架 —— workspace 可编译可测试，任务执行尚未实现
```

- `cargo test` 通过（16 个测试，零外部依赖，无需 MaaFramework SDK）
- `cargo run -p maazure-daemon -- --tree` 打印仓库树
- `cargo run -p xtask -- --list` 列出计划中的转换器

下一步：**P0** —— 建 golden corpus → 转换资源 → 接 FFI → 委托任务跑通。
见 [docs/6-plan/6.1-roadmap.md](docs/6-plan/6.1-roadmap.md)。

## 许可证

**AGPL-3.0**（见 [LICENSE](LICENSE)）。

| 上游 | 协议 | 关系 |
|---|---|---|
| AzurPilot | GPL-3.0 | 数据与领域知识来源；GPLv3 §13 允许并入 AGPLv3 作品 |
| MaaFramework | LGPL-3.0 | 动态链接、不修改、可替换 → 合规 |
| MaaEnd | AGPL-3.0 | 架构参照，未复制代码 |
| JMNeXt buildkit | MPL-2.0 | `tools/buildkit/` 派生，保留文件头 |

归属声明见 [NOTICE](NOTICE)。

## 免责声明

仅供学习与个人自动化使用。不用于商业用途、不用于竞技作弊、不修改任何游戏文件。
使用者自行承担账号风险，作者不承担任何责任。
