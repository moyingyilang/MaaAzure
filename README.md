# MaaAzure

**碧蓝航线自动化 · MaaFramework 技术栈 · 零 Python**

> 从 [AzurPilot](https://github.com/wess09/AzurPilot) 重写而来：保留领域知识，换掉整条执行栈。
> 本仓库**独立演化**，旧仓库冻结仅作数据源与回归基线。

---

## 这个项目要解决什么

AzurPilot 是 288,076 行 Python 的自研自动化栈：自己写设备层、模板匹配、OCR、状态循环、调度器、WebUI。
这套东西能跑，但每加一个功能都要在 2,000 个文件里找位置，且没有一条游戏逻辑的自动化测试。

MaaAzure 的假设是：**把「看图 → 点哪里」这件事交给成熟的 C++ 内核，我们只保留游戏知识。**

| 层 | 谁来做 |
|---|---|
| 截图、输入、图像识别、任务执行引擎 | **MaaFramework**（C++，LGPL-3.0） |
| 任务流程编排 | **Pipeline JSON**（声明式，`next` 图 + 超时 + 错误分支） |
| 跨帧有状态逻辑（战役寻路、大世界、战斗决策） | **Rust** 自定义识别/动作 |
| 配置 schema、多语言、GUI 接入 | **ProjectInterface v2** |
| 调度、统计、通知、HTTP/MCP | **Rust**（axum + tokio） |

**没有一个 Python 文件。**

---

## 命名

`MaaAzure` = `Maa`（MaaFramework 生态前缀，官方鼓励 `MaaXXX` 命名）+ `Azure`（Azur Lane，碧蓝航线）。

全局无重名（GitHub 搜索 `MaaAzure` 零结果），不与其他项目混淆。

---

## 技术栈

| 组件 | 版本 | 用途 |
|---|---|---|
| [MaaFramework](https://github.com/MaaXYZ/MaaFramework) | v5.x（SDK ≥ 5.4.1） | 内核：控制器 / 识别 / Pipeline 执行 |
| [maa-framework (Rust)](https://github.com/MaaXYZ/maa-framework-rs) | `1` | 官方 Rust 绑定，静态或动态链接 |
| Rust | edition 2024 | 领域逻辑、守护进程、CLI |
| ProjectInterface v2 | v2.10.5 | 配置协议，兼容 MXU / MFAAvalonia |

不用：Python、PyWebIO、uiautomator2、自研 OCR、自研设备层。

---

## 目录

```
MaaAzure/
├── crates/                  Rust workspace
│   ├── maazure-core/        配置、调度、错误、实例管理
│   ├── maazure-vision/      自定义识别：网格检测、单应性、OCR 后处理
│   ├── maazure-battle/      战役/战斗/大世界状态机与寻路
│   ├── maazure-runner/      Tasker 驱动、Pipeline 入口、事件桥
│   ├── maazure-daemon/      axum HTTP/WS/MCP + CLI（bin）
│   └── xtask/               资源转换、代码生成
├── resource/                MaaFramework 资源包
│   ├── default_pipeline.json
│   ├── pipeline/            任务流程（JSON）
│   └── image/               模板图（由 AzurPilot assets/ 转换）
├── data/                    战役地图、岛屿、科研数据（JSON）
├── interface.json           PI v2 配置与 GUI 声明
├── docs/                    设计文档（先读 docs/00-INDEX.md）
└── tools/buildkit/          Termux chroot 构建工具链
```

---

## 状态

```
📝 设计阶段 —— 文档与骨架已建，P0 尚未实现
```

路线图见 [docs/A7-roadmap.md](docs/A7-roadmap.md)。当前里程碑 **P0**：
golden corpus 回归网 + Rust workspace 骨架 + 委托任务 Pipeline 跑通。

---

## 与上游的关系

| 项目 | 关系 | 许可证 |
|---|---|---|
| **AzurPilot** | 领域知识、模板图、战役数据、配置语义的来源 | GPL-3.0 |
| **MaaFramework** | 被执行内核，**动态链接、不修改** | LGPL-3.0 |
| **MaaEnd** | 架构参照（Pipeline + 自定义模块 + MXU），不同游戏 | AGPL-3.0 |
| **MaaAzure** | 本项目 | **GPL-3.0**（AzurPilot 衍生作品，必须同协议） |

MaaFramework 的 LGPL-3.0 与 GPL-3.0 兼容：本项目以独立进程动态链接方式使用，不修改其源码，
因此不触发 LGPL 的源码开放义务。详见 [docs/A8-risks.md](docs/A8-risks.md)。

---

## 免责声明

仅供学习与个人自动化使用。不用于商业用途，不用于竞技作弊，不修改任何游戏文件。
使用者自行承担账号风险，作者不承担任何责任。
