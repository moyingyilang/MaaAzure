# A1 · 架构总览

## 1. 问题定义

AzurPilot 的现状（数据取自 `git log -1 = 86a9463d4`）：

| 指标 | 数值 |
|---|---|
| Python 代码 | 288,076 行 / 2,037 文件 |
| 任务方法 | 93 个（`alas.py` 共 124 个方法） |
| 模板图 | 7,786 张 / 152 MB |
| Button/Template 常量 | 2,185 个 |
| 配置项 | 2,401 项（`args.json` 展开后） |
| 战役地图文件 | 1,436 个 `.py`（数据即代码） |
| 游戏逻辑自动化测试 | **0** |
| 单帧耗时构成 | 截图 ~350 ms（99%），图像处理 2.5 ms，OCR 100–180 ms |

结论：**瓶颈不在语言，在「没有测试」和「自研栈的维护面积」。** 因此 MaaAzure 的目标不是提速，
而是把 288k 行的维护面积压到可审计的规模，并且让每一块逻辑都能被录像回归。

### 1.1 非目标

- 不改游戏、不碰内存、不做无 UI 的协议级自动化
- 不支持真机 7×24（沿用 AzurPilot 的结论：长时间运行黑屏卡死、截图压缩、OCR 迁移问题）
- 不追求功能对齐以外的增强；P0–P3 期间**只做 parity**
- 不为「性能」引入复杂度。若某处 Rust 化只带来毫秒级收益，就不做

## 2. 目标架构

```
┌──────────────────────────────────────────────────────────────────┐
│  UI 层（二选一，核心 crate 不感知）                                │
│  ① maazure-daemon 内建 Web（axum + 静态资源）  ← 默认，7×24 调度    │
│  ② 任意 PI Client（MXU / MFAAvalonia）+ interface.json             │
└───────────────┬──────────────────────────────────────────────────┘
                │ HTTP/WS · 进程内调用
┌───────────────▼──────────────────────────────────────────────────┐
│  maazure-runner                                                   │
│  实例管理 · 调度器（NextRun/优先级/失败接管）· 事件桥 · 截图归档     │
└───────────────┬──────────────────────────────────────────────────┘
                │ maa-framework (Rust)  ── FFI ──▶ MaaFramework (C++)
┌───────────────▼──────────────────────────────────────────────────┐
│  MaaFramework v5                                                   │
│  Controller(ADB/Win32) · Resource(JSON+PNG) · Tasker(Pipeline 执行)│
│  识别: TemplateMatch / OCR / ColorMatch / FeatureMatch / NN / Custom│
│  动作: Click / Swipe / Scroll / Key / Shell / Screencap / Custom    │
└───────────────┬──────────────────────────────────────────────────┘
                │ Custom 回调（同进程，零 IPC）
┌───────────────▼──────────────────────────────────────────────────┐
│  maazure-vision（识别）        maazure-battle（决策）              │
│  网格检测 / 单应性 / OCR 后处理  战役寻路 / 大世界 / 战斗状态机      │
└──────────────────────────────────────────────────────────────────┘
```

**关键点：Custom 回调与 Tasker 同进程。** 不用 MaaFramework 的 Agent 子进程协议（ZMQ），
因为那会为每个节点引入一次跨进程序列化，而我们每帧预算只有个位数毫秒。
PI 的 `agent` 字段只作为「被 MXU 驱动」时的兼容路径保留。

## 3. 架构决策记录

| ID | 决策 | 理由 | 代价 |
|---|---|---|---|
| **DEC-001** | 用 MaaFramework 做执行内核，不自研设备/视觉层 | 12,831 行设备层 + 2,258 行 OCR 换来一个 C++ 实现、有 CI 单测、有生态工具链的内核 | 受上游 API 演进约束；LGPL 合规需动态链接 |
| **DEC-002** | 复杂逻辑用 **Rust**，不用 Go/Python | 与 C++ 内核 FFI 成本最低；`maa-framework` crate 官方维护；单二进制分发；no-GC 让帧预算可预测 | 编译慢、地图检测等 CV 代码要自己写 |
| **DEC-003** | 流程用 **Pipeline JSON**，判据见 §4 | 2,185 个 Button 的「看→点」本就不该是代码；`next` 图替代 `module/ui/` 的 A* 寻路（-2,220 行） | JSON 无类型检查，靠 schema + CI 校验 |
| **DEC-004** | **独立仓库**，旧仓冻结打 tag | 双轨灰度：旧版能跑、能录制基线；新版逐任务 parity 后再切 | 数据需单向同步（见 A3） |
| **DEC-005** | 默认 **自托管 daemon**，不把调度交给 PI Client | PI 的 `task` 列表表达不了「NextRun 到期、按优先级挑下一个、连败三次转人工、配置热重载」 | 要自己写 Web（可复用旧仓静态资源） |
| **DEC-006** | **interface.json 是唯一配置 schema** | 2,401 个配置项 → PI 的 `option`/`task`，多语言与 GUI 直接免费 | 旧配置迁移器必须完备（A3 §4） |
| **DEC-007** | 分辨率走 `display_short_side: 720` 归一化 | 7,786 张 1280×720 图**原样复用**，不做重截 | 依赖 MaaFW 缩放质量，需 P0 实测 |
| **DEC-008** | 战役地图 `.py` → `data/*.json`，**数据不编译进代码** | 1,436 个文件是纯字面量，`ast.literal_eval` 即可提取；改数据不用重编译 | 需要一次性转换器 + 校验 |
| **DEC-009** | 先建 **golden corpus**，再迁移逻辑 | 现有 0 测试；不建基线就是闭眼改 288k 行 | 需要旧版配合录制（一次性，几百行） |
| **DEC-010** | 许可证 **GPL-3.0** | AzurPilot 是 GPL-3.0，衍生作品不得改用其他协议 | 生态上与 AGPL 项目（如 MaaEnd）不可合并代码 |

## 4. Pipeline 与 Rust 的边界（硬判据）

一条逻辑，满足**全部**下列条件时写成 Pipeline 节点；否则写成 Rust Custom：

1. **帧内闭合**：识别与决策只依赖当前这张截图；
2. **无跨帧记忆**：不需要「上次走到哪、还剩几次、第几页」；
3. **动作是原子 UI 操作**：点击、滑动、按键、等待画面稳定，而非计算后连续多步。

| 例子 | 归属 | 依据 |
|---|---|---|
| 主界面 → 委托 → 领取 → 返回 | Pipeline | 每步都是「看见就点」 |
| 翻页直到找到目标关卡 | Rust（或 Pipeline + `max_hit`） | 需要「第几页」记忆 |
| 委托列表 OCR 出 4 条并选择剩余时间最短 | Rust | 需要比较与排序 |
| 战役地图：识别网格 → A* 找路 → 连续移动 | Rust | 计算 + 跨帧路径状态 |
| 大世界：世界地图 → 港口 → 交换 → 回地图 | Rust 外层 + Pipeline 内层 | 外层有状态，内层是点击序列 |
| 战斗：血量低于阈值撤退 | Rust | 阈值判断 + 跨帧计数 |
| 「关闭所有弹窗」 | Pipeline | 纯点击序列，用 `next` 全连接 |

**反模式（出现即 review 打回）：**

- CustomAction 里 `for` 循环点 8 次 —— 应该是 8 个 node 或 `Scroll` + `max_hit`
- Pipeline 里用 `max_hit` 当循环计数器做业务分支 —— 应该是 Custom
- CustomRecognizer 里调用 `controller.post_screencap().wait()` 再识别 —— 用 `argv.image`，别自己截

## 5. 进程与线程模型

```
maazure-daemon (1 进程)
├── tokio runtime (multi-thread)
│   ├── HTTP/WS 服务线程池
│   ├── 调度器 task（单线程，tick 1s）
│   └── 每实例 1 个 supervisor task
├── 实例进程（process-per-instance，默认）
│   ├── Controller (ADB) ── adb server ── 模拟器
│   ├── Resource (bundle)
│   ├── Tasker (Pipeline executor)
│   └── Custom 回调（Rust，在 MaaFW 的 C++ 工作线程上执行）
└── 共享：日志 ring buffer、截图归档、统计队列
```

**为什么 process-per-instance**：MaaFramework 在 ADB 断开/驱动卡死时可能让 Tasker 卡住，
进程隔离保证一个实例挂了不影响其他实例与 Web。与旧版 `multiprocessing.Process` per config 语义一致。
单实例调试可用 `--in-process` 关掉隔离。

**线程安全红线**：

- MaaFW 的 Custom 回调运行在 C++ 线程，**禁止**在回调里 `block_on` 或长时间 `wait()`
- 所有跨越回调边界的共享状态用 `Arc<Mutex<..>>`，且临界区不得包含 IO
- 回调里只做两件事：读 `argv.image` / `context`，返回结果；需要点击就 `post_click()` 异步投递

## 6. 一帧的完整时序（以委托为例）

```
t=0    Pipeline: node "commission_enter" 识别 COMMISSION_ENTRY (TemplateMatch)
t=1    hit → action Click(target) → 投递点击
t=2    pre_delay 200ms → post_wait_freezes 500ms（等画面稳定）
t=3    next: ["commission_page", "home", [JumpBack]]  依次识别
t=4    "commission_page" 命中 → 动作 OCR 读取 4 条剩余时间
t=5    action Custom("commission_pick")  ← 进 Rust
         ├─ 解析 4 个 Duration → 选最小值 → 计算点击坐标
         └─ context.tasker.controller.post_click(x, y)（不 wait）
t=6    返回 true → 进入 next: ["commission_claim", "commission_recover"]
t=7    "commission_claim" 命中 → Click → next: ["commission_page", "home"]
...
t=N    某节点 next 为空 → 任务结束；或命中 "[JumpBack]" 沿 anchor 回跳
```

一帧内 Rust 侧期望耗时：**< 5 ms**（不含截图）。OCR 由 MaaFW 承担，结果以 `Detail` 回传。

## 7. 模块映射表

`LOC` 取自旧仓实测。**去留**一列说明这块知识的新归属。

| 旧模块 | LOC | 去留 | 新归属 |
|---|---|---|---|
| `module/device/` | 12,831 | **删除** | MaaFramework Controller + `Toolkit::find_adb_devices()` |
| `module/ocr/` | 2,258 | **删除**（保留后处理规则） | MaaFW 内置 OCR；`maazure-vision` 只留数字纠错（I→1、S→5 等） |
| `module/ui/` | 2,220 | **删除** | Pipeline `next` 图 + `anchor` / `[JumpBack]` |
| `module/webui/` | 18,573 | **删除** | `maazure-daemon`（axum）+ 复用 `assets/spa` 静态资源 |
| `module/base/template.py` `button.py` `mask.py` | ~1,500 | **删除** | TemplateMatch / ColorMatch / FeatureMatch 节点 |
| `module/config/` | 5,966 | **重写** | `interface.json`（PI v2）+ `maazure-core::config`（4,000 项 → 生成强类型） |
| `alas.py` 调度器 | 2,097 | **重写** | `maazure-runner::scheduler` |
| `module/map_detection/` | 2,783 | **重写** | `maazure-vision::grid`（单应性 + 网格预测，纯 Rust） |
| `module/map/` | 5,522 | **重写** | `maazure-battle::map` |
| `module/campaign/` | 4,905 | **重写** | `maazure-battle::campaign` + `data/` |
| `module/combat/` | 2,651 | **重写** | `maazure-battle::combat` |
| `module/handler/` | 3,347 | **拆分** | 弹窗/登录 → Pipeline 节点；自动搜索/敌人 → `maazure-battle` |
| `module/os*/`（6 个） | ~19,000 | **重写** | `maazure-battle::os`（分期，最后做） |
| `module/island/` | 14,098 | **重写** | `maazure-battle::island`（分期） |
| 日常任务（委托/宿舍/科研/商店/…） | ~30,000 | **Pipeline 化** | `resource/pipeline/*.json` |
| `module/statistics/` `azur_stats/` | ~6,000 | **重写** | `maazure-daemon::stats` |
| `module/notify/` `llm.py` | ~2,000 | **重写** | `maazure-daemon::notify` / `llm` |
| `mcp_server_sse.py` | 574 | **重写** | `maazure-daemon::mcp`（18 个工具同名保留） |
| `assets/` 7,786 PNG | — | **转换复用** | `resource/image/`（A3） |
| `campaign/` 1,436 `.py` | — | **转换复用** | `data/campaign/*.json`（A3） |

**预估新线规模**：Rust 45k–70k 行 + Pipeline JSON 8k–12k 行 + data JSON 自动生成。
相比 288k 行，维护面积降到约 **1/4**，且其中 60% 是数据而非逻辑。

## 8. 与 MaaFramework 版本的对齐策略

- 固定 SDK 版本（`resource/` 与二进制一起发布），**不依赖用户系统里的 MaaFW**
- 升级只允许改 `maazure-runner/src/ffi.rs` 一个文件；上层 crate 不得直接 `use maa_framework::*`
- 升级流程：改 Cargo.lock → 跑 golden corpus → 看 Pipeline 协议变更日志（v5.x 有版本表）
- PI 协议版本与 `interface_version: 2` 解耦（PI 文档明确二者不必一致）

## 9. 未决问题

| # | 问题 | 影响 | 何时定 |
|---|---|---|---|
| Q1 | MaaFW `display_short_side: 720` 缩放后的模板匹配阈值漂移有多大 | DEC-007 是否成立 | P0 实测 |
| Q2 | Win32 控制器能否覆盖 MuMu/雷电的「后台窗口」场景（不抢焦点） | 是否提供 Win32 模式 | P1 |
| Q3 | 大世界是否拆成独立 crate | 编译时间 vs 边界清晰 | P2 前 |
| Q4 | 旧仓配置迁移是否需要双向同步（用户已在旧版跑着） | 迁移器复杂度 | P1 |
