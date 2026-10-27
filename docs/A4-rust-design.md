# A4 · Rust 侧设计

> 代码片段为**设计示意**，用于说明边界与签名；真实 API 以 `maa-framework` crate v1 为准。

## 1. Workspace 布局

```
crates/
├── maazure-core/     配置模型、错误、日志、时间、实例句柄
├── maazure-vision/   CustomRecognizer 实现（网格、单应性、OCR 后处理）
├── maazure-battle/   战役/战斗/大世界状态机（纯逻辑，不依赖 FFI）
├── maazure-runner/   Tasker 驱动、Pipeline 入口、事件桥、调度器
├── maazure-daemon/   axum HTTP/WS/MCP + CLI（唯一 bin）
└── xtask/            资源转换、pipeline 校验、代码生成
```

依赖方向（严格单向，`cargo deny` 校验）：

```
daemon → runner → battle → vision → core
                   └──────────────→ core
xtask  → core（只读旧仓数据，不依赖 runner）
```

**铁律：`maa-framework` 只允许出现在 `maazure-runner`。**
其他 crate 不得 `use maa_framework`。这样 MaaFW 升级（含破坏性变更）只影响一个 crate，
也让 `battle` / `vision` 能在没有 SDK 的机器上 `cargo test`（包括本机 Termux）。

```toml
# crates/maazure-runner/Cargo.toml
[dependencies]
maa-framework = { version = "1", features = ["toolkit", "adb", "custom"] }
# Windows 发布版额外：features = ["win32"]
```

`features = ["dynamic"]` 是 cross-build 的关键（见 A5 §4），但会引入运行时路径问题，
默认走静态；发布包用 `dynamic` + 同目录 DLL，便于用户替换 MaaFW 版本排障。

## 2. FFI 隔离层

`maazure-runner/src/ffi.rs` 是**唯一**接触 C ABI 的文件，提供窄接口：

```rust
pub struct Engine { /* Controller + Resource + Tasker */ }

impl Engine {
    pub fn connect(&mut self, cfg: &ControllerCfg) -> Result<()>;
    pub fn load_bundle(&mut self, paths: &[PathBuf]) -> Result<()>;
    pub fn run_pipeline(&mut self, entry: &str, overrides: &PipelineOverride) -> Result<TaskOutcome>;
    pub fn screenshot(&self) -> Result<Frame>;
    pub fn click(&self, p: Point) -> Result<()>;
    pub fn swipe(&self, from: Point, to: Point, ms: u64) -> Result<()>;
    pub fn stopping(&self) -> bool;
}
```

上层只拿到这些方法。`Context`、`Tasker`、`Job` 这些 MaaFW 类型不出这个文件。

**为什么**：MaaFW 的 `Job.wait()` 是阻塞的，`Context` 有生命周期且非 `Send`。
把它关在一个文件里，问题不会扩散到 5 个 crate。

## 3. 核心 trait

```rust
/// 一帧图像 + 元信息。图像由 MaaFW 传入（ROI 已按需裁剪）。
pub struct Frame<'a> {
    pub image: &'a ImageBuffer<Rgb<u8>, Vec<u8>>,
    pub seq: u64,          // 帧序号，用于状态机去重
}

/// 自定义识别：不点击、不睡眠、不改 pipeline，纯函数式。
pub trait Recognizer {
    type Out;
    fn recognize(&self, ctx: &RecogCtx<'_>, frame: Frame<'_>) -> Result<Option<Self::Out>>;
}

/// 自定义动作：可以投递点击/滑动，但必须立即返回。
pub trait ActionHandler {
    fn act(&mut self, ctx: &ActionCtx<'_>, arg: &serde_json::Value) -> Result<ActionOutcome>;
}
```

约定：

- `Recognizer` 必须**无副作用**。想在识别里点一下，说明它该是 Action。
- `ActionHandler` 允许持有跨帧状态（`&mut self`），这是战役/大世界状态机的落点。
- 所有 `Ctx` 都提供 `post_click`（异步投递）与 `stopping()`（响应停止请求）。
- 返回值只表达「成功/失败/未命中」，业务分支由 Pipeline 的 `next` 决定。

## 4. 状态机模式（替代旧版 `while` 循环）

旧版的核心写法是「一个 `while 1:` 里套 5 个 `if appear_then_click`」。
新版拆成显式状态枚举，好处是**可单测、可回放、可打点**：

```rust
pub enum CampaignState {
    EnterMap,
    DetectMap { grid: Grid },
    PlanRoute { path: Vec<Node> },
    MoveFleet { step: usize },
    InCombat,
    Settle,
    Done,
}

pub struct CampaignMachine {
    state: CampaignState,
    battles: u32,
    max_battles: u32,
}

impl CampaignMachine {
    /// 每次调用 = 一帧决策。返回 None 表示「这一帧不做决策，等下一帧」。
    pub fn step(&mut self, frame: Frame<'_>, view: &MapView) -> Result<Option<Command>>;
}

pub enum Command {
    Click(Point),
    Swipe { from: Point, to: Point, ms: u64 },
    WaitFrames(u8),
    Finish(TaskOutcome),
}
```

**这是整个重写里最重要的模式。** 旧版 93 个任务、每个任务一个 `while` 循环，
无法测试；新版每个领域一个 `Machine`，输入是（帧, 视图），输出是 `Command`，
纯函数式可测——`cargo test` 里喂 100 帧录像就能验证决策序列（A6）。

## 5. 错误模型

直接对照旧 `module/exception.py`，避免重写时丢掉错误分类语义：

```rust
#[derive(Debug, thiserror::Error)]
pub enum AzureError {
    // 正常结束，不算失败
    #[error("campaign finished")]
    CampaignEnd,
    #[error("oil exhausted")]
    OilExhausted,

    // 可自动恢复（对应旧版返回 'recoverable'）
    #[error("map detection failed: {0}")]
    MapDetection(String),
    #[error("enemy moved")]
    EnemyMoved,

    // 需要重启游戏（对应旧版 GameStuckError 等）
    #[error("game stuck for {0:?}")]
    GameStuck(Duration),
    #[error("too many clicks on {button}")]
    GameTooManyClick { button: String },

    // 不可恢复
    #[error("human takeover required: {0}")]
    HumanTakeover(String),

    // 基础设施
    #[error("controller error: {0}")]
    Controller(String),
    #[error(transparent)]
    Maa(#[from] maa_framework::Error),
}
```

**语义保留清单**（迁移时逐条对照，不能丢）：

| 旧行为 | 新实现 |
|---|---|
| 任务返回 `True` / `False` / `'recoverable'` | `TaskOutcome::{Ok, Failed, Recoverable}` |
| 同一任务连续失败 3 次 → `RequestHumanTakeover` | `Scheduler::record_failure` 计数 |
| `GameStuckError`：60s 无有效操作 | 调度器侧看门狗（不放在 Pipeline 里） |
| `GameTooManyClickError`：15 次操作同一按钮 ≥12 次 | `EventSink` 里统计，命中即 `Stopping` |
| `Error_StrictRestart` + 任务级 `Sensitive` | 配置项，控制「出错是重启游戏还是重试」 |

看门狗与点击统计放在 **runner 的 EventSink** 里，不放在 Pipeline：
这样所有任务自动获得保护，不需要每个任务自己判。

## 6. 并发红线段

MaaFW 的 Custom 回调运行在 C++ 工作线程，规则：

1. **禁止阻塞**：回调内不做 IO、不 `sleep`、不 `block_on`。
2. **禁止持有锁跨 FFI**：锁的临界区只操作内存。
3. **点击一律异步投递**：`post_click` 立即返回，需要确认时用 Pipeline 的 `post_wait_freezes`。
4. **共享状态**：`Arc<Mutex<InstanceState>>`，且 `InstanceState` 里不放 `Controller`。
5. **停止响应**：任何长逻辑（>200ms）必须每步检查 `ctx.stopping()`。

违反第 1 条的典型后果：整条流水线在等一帧，用户点停止没反应，只能杀进程。

## 7. 调度器

```rust
pub struct Scheduler {
    tasks: Vec<TaskSpec>,            // 93 个任务的规格
    queue: BinaryHeap<Scheduled>,    // 按 (优先级, NextRun) 排序
    failures: HashMap<TaskId, u8>,
}

impl Scheduler {
    pub fn next_due(&self, now: SystemTime) -> Option<&TaskSpec>;
    pub fn record(&mut self, id: TaskId, outcome: TaskOutcome, now: SystemTime);
}
```

- 优先级沿用旧版顺序（`Restart > OpsiCrossMonth > Commission > Tactical > Research > … > Main`），
  在 `data/scheduler.json` 里声明，不写死在代码
- `NextRun` 由任务自己算（科研 30min、委托 20min、宿舍固定时刻…），存 `config/state.json`
- 失败 3 次 → 该任务停摆 + 通知；其他任务继续（旧版是整脚本 takeover，新版更温和，
  这是**有意的行为变更**，写进 A8 变更清单）
- 空闲策略：`WhenTaskQueueEmpty: close_game | goto_main | stay_there`（沿用旧配置项语义）

## 8. 配置强类型

`interface.json` 是唯一 schema 源，`xtask` 生成 Rust 类型：

```rust
// 生成物：crates/maazure-core/src/config/generated.rs  #[allow(dead_code)]
#[derive(Deserialize)]
pub struct CommissionConfig {
    #[serde(default = "def_receive")] pub receive: bool,
    #[serde(default)] pub skip_urgent: bool,
}

fn def_receive() -> bool { true }
```

好处：写错配置键编译期报错；IDE 有补全；不用像旧版那样靠 `self.config.Commission_Receive`
的字符串拼接（旧版拼错只会运行时 KeyError）。

## 9. 日志与可观测

- `tracing` + `tracing-subscriber`，每条日志带 `instance` / `task` / `frame` 三个 span 字段
- 日志分级沿用旧版：`hr(title, level)` → 结构化 `Section { level, title }` 事件
- **截图归档**：ring buffer 保留最近 200 帧（每帧 JPEG q75，约 30KB → 6MB/实例），
  出错时把这 200 帧 + 决策轨迹打包成 zip，路径写进错误日志
  （旧版只存一张截图，复盘时经常不够）
- **决策轨迹**：每帧记录 `(frame_seq, node, hit, action, target)`，NDJSON 落盘，
  可直接喂给 golden corpus 做回归（A6）

## 10. 帧预算

| 阶段 | 预算 | 手段 |
|---|---|---|
| 截图 | 由 MaaFW 决定 | `display_short_side: 720` + 模拟器原生截图方法 |
| 识别（模板） | < 3 ms | ROI 必填，避免全屏扫 |
| 识别（OCR） | 30–80 ms | ROI 收紧；能模板就别 OCR |
| Rust 决策 | **< 2 ms** | 纯内存计算；地图检测单帧 < 15 ms |
| 动作投递 | < 1 ms | 异步，不 wait |

**唯一允许超 100ms 的 Rust 操作是地图检测**（单应性 + 网格预测）。
若实测超标，先把 ROI 缩到地图区域再降采样，不允许为此引入异步复杂度。
