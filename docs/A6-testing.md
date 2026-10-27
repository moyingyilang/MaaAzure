# A6 · 回归测试（golden corpus）

## 1. 为什么这是 P0 的第一件事

旧仓 `tests/` 有约 160 个测试，全部覆盖 WebUI / 进程管理 / 配置逻辑。
**游戏逻辑的自动化测试数量是 0。**（`CLAUDE.md` 原文：「游戏逻辑无自动化测试——通过在真实模拟器实例上运行任务完成」）

在这种状态下重写 288k 行的游戏逻辑，等于闭眼跳崖：改错了没人知道，等用户报「大世界卡住」时，
你已经忘了两个月前改了哪一行。

**所以先建网，再拆楼。**

## 2. golden corpus 是什么

一段真实运行的**逐帧录像 + 决策轨迹**，可以在没有游戏、没有模拟器的情况下回放，
断言新代码做出了同样的决策。

```
corpus/
├── manifest.json                       # 用例索引
└── commission/
    ├── 0001.png                        # 原始截图（1280×720 PNG，无损）
    ├── 0001.decision.json              # 这一帧的期望决策
    ├── 0002.png
    ├── 0002.decision.json
    └── case.json                       # 用例元信息
```

`case.json`：

```jsonc
{
  "name": "commission_pick_shortest",
  "source_rev": "86a9463d4",
  "task": "Commission",
  "server": "cn",
  "resolution": [1280, 720],
  "frames": 214,
  "preconditions": "主界面，4 条委托全部完成",
  "expected_final": "commission_done"
}
```

`0001.decision.json`：

```jsonc
{
  "frame": 1,
  "node": "commission_page",          // 旧版：哪个 handler 分支命中
  "hits": { "commission_pick": true, "commission_empty": false },
  "action": { "kind": "click", "point": [1092, 352] },
  "ocr": { "slots": ["08:00:00", "01:20:00", "04:10:00", "02:30:00"] },
  "elapsed_ms": 412
}
```

## 3. 录制器（一次性工具，跑在冻结的旧仓里）

**它不属于新线代码。** 放在旧仓 `../AzurPilot/dev_tools/record_golden.py`，约 300 行，
用一次就可以冻结。理由：录制必须在旧版真跑时进行，而旧版是 Python。

做的事：

1. 挂到 `ModuleBase.device.screenshot()` 上，每帧存 PNG + 当前时间戳
2. 挂到 `appear()` / `appear_then_click()` / `click()` 上，记录命中结果与点击坐标
3. 挂到 `Ocr` / `Digit` / `DigitCounter` 上，记录识别文本与区域
4. 任务结束时写 `case.json`（含任务名、服务器、结束原因）
5. 只保留**有决策的帧**：连续相同画面的帧折叠，体积能降到 1/5

体积估算：一个日常任务约 200–400 个决策帧 × ~120KB（720p PNG）≈ **30–50MB**。
20 个任务 ≈ 600MB–1GB。**不进 git**，作为 release 附件 + `~/.cache` 缓存，CI 里按需下载。

> 若嫌 PNG 大，可用无损 WebP（同样可被 `image` crate 读），能压到 1/3。
> 但**不要用 JPEG**：模板匹配对压缩伪影敏感，回放结论会和新环境不一致。

## 4. 回放器（新线）

关键设计：**回放不需要 MaaFW**。

`maazure-runner` 把 `Engine` 定义成 trait，生产实现是 MaaFW，测试实现是 `ReplayEngine`：

```rust
pub trait Engine {
    fn screenshot(&self) -> Result<Frame>;
    fn click(&mut self, p: Point) -> Result<()>;
    fn swipe(&mut self, from: Point, to: Point, ms: u64) -> Result<()>;
}

pub struct ReplayEngine { frames: Vec<ImageBuf>, cursor: usize, clicks: Vec<Point> }
```

于是：

```rust
#[test]
fn commission_picks_shortest() {
    let case = Case::load("corpus/commission/case.json");
    let mut machine = CommissionMachine::new(cfg!());
    let mut engine = ReplayEngine::from(case);

    for expected in case.decisions() {
        let cmd = machine.step(engine.frame(), &view(&engine.frame())).unwrap();
        assert_eq!(cmd, expected.command(), "frame {} node={}", expected.frame, expected.node);
        engine.apply(cmd);
    }
    assert_eq!(machine.finished(), Some(TaskOutcome::Ok));
}
```

**这就是为什么 `battle` / `vision` 不允许依赖 `maa-framework`**（A4 §1）：
它们要能在本机 Termux 上跑（无 SDK），也要能在 CI 的任意 runner 上跑。

## 5. 断言层次（从粗到细，逐层启用）

| 层 | 断言 | 何时启用 | 误报风险 |
|---|---|---|---|
| L1 | 最终状态一致（Ok / Failed / CampaignEnd） | 第一天 | 低 |
| L2 | 每帧选中的**节点**一致 | P1 | 低 |
| L3 | 每帧的**点击坐标**一致（±3px） | P2 | 中（随机落点） |
| L4 | OCR 文本一致 | P2 | 中（模型差异） |
| L5 | 帧耗时在预算内 | P3 | 低（机器差异需放宽） |

**L3 的坑**：旧版点击落点是 `random_rectangle_point(area)`（正态分布随机）。
录制时记录的是那一次的随机点，断言不能要求精确相等，只能要求「落在同一 button 区域内」。
所以 `decision.json` 里同时存 `area`，断言用「点在 area 内」而非点相等。

**L4 的坑**：新旧 OCR 引擎不同（旧版 AlOcr vs MaaFW 内置），同一张图可能给出不同结果。
对策：录制时同时存「区域截图」，回放测试跑的是**我们自己的后处理**（`maazure-vision` 的数字纠错），
OCR 引擎本身不参与 L4 断言，只断言后处理结果。

## 6. CI 策略

```yaml
# 快测（每次 push，< 3min）
cargo test -p maazure-core -p maazure-battle -p maazure-vision --lib
xtask pipeline-check

# 慢测（PR / 每日，10–20min）
cargo test --workspace --test replay -- --case-limit 5   # 抽样 5 个用例

# 全量（发版前，手动触发）
cargo test --workspace --test replay -- --all
```

## 7. 录制清单（P0 必须录到的 12 个用例）

| # | 用例 | 覆盖的知识 |
|---|---|---|
| 1 | 启动 → 登录 → 主界面 | 登录流程、公告弹窗 |
| 2 | 委托：领取 + 重新委托 | OCR 时间、列表选择 |
| 3 | 宿舍：补充食物 + 领取 | 滑动、多舰队 |
| 4 | 科研：收取 + 立项 | 项目列表、条件判断 |
| 5 | 战术学院：上课 | 简单 Pipeline |
| 6 | 商店：购买指定物品 | 滚动条定位 |
| 7 | 每日任务 + 奖励领取 | 多弹窗收敛 |
| 8 | 主线战役 3-4 推图 | 地图检测、寻路、战斗 |
| 9 | 活动战役（当前活动） | 关卡选择、HUD 差异 |
| 10 | 大世界：进入 + 一次短猫 | 世界地图、路径 |
| 11 | 战斗撤退（血量低） | 阈值决策 |
| 12 | 异常恢复：断连 → 重连 → 回主界面 | 错误路径 |

用例 12 最容易被忽略，但它覆盖的正是**最不可测、最容易在重写时丢掉**的部分。

## 8. 反模式

| 反模式 | 后果 |
|---|---|
| 只在 CI 里跑回放，本地不跑 | 开发时改坏了自己不知道，push 后才发现 |
| 断言精确点击像素 | 随机落点导致必然 flaky |
| 用 JPEG 当回放帧 | 模板匹配漂移，误判为逻辑 bug |
| 用例只覆盖 happy path | 异常恢复路径无人验证，正是最脆弱处 |
| corpus 进 git | 仓库 1GB，clone 5 分钟 |
