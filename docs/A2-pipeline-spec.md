# A2 · Pipeline 编写规范

> 本文的 JSON 示例用于说明**设计意图**，字段细节以
> [任务流水线协议 v5.8](https://github.com/MaaXYZ/MaaFramework/blob/main/docs/zh_cn/3.1-任务流水线协议.md) 为准。
> 每个示例落地时都要过 CI 的 schema 校验（§7）。

## 1. 为什么用 Pipeline

旧版 `module/ui/` 用 2,220 行实现了「页面检测 + A* 寻路」。Pipeline 的 `next` 图天然就是带超时、
错误分支、回跳锚点的寻路器：

| 旧版概念 | Pipeline 对应 |
|---|---|
| `Page(check_button, links)` | 节点 `next` 列表 |
| `ui_goto(page)` A* 最短路径 | 运行时逐个尝试 `next`，天然是广度优先 |
| `appear_then_click(BTN, interval=2)` | `rate_limit: 2000` |
| 循环退出条件 `appear(END)` | 该节点 `next` 指向结束节点；未命中则 `on_error` |
| `ui_additional()` 弹窗兜底 | 每个 `next` 列表末尾挂 `[JumpBack]popup_close` |
| 状态循环里的 `skip_first_screenshot` | `pre_wait_freezes`（等画面不动），不需要复用截图 |

**净效果：删掉 2,220 行寻路代码，且不再有「忘记 `continue` 导致重复点击」这类 bug。**

## 2. 文件组织

```
resource/
├── default_pipeline.json        # 全局基线（rate_limit / threshold / pre_delay）
├── pipeline/
│   ├── 00_common.json           # 主界面、弹窗、错误恢复、返回
│   ├── 10_daily/
│   │   ├── commission.json
│   │   ├── dorm.json
│   │   ├── research.json
│   │   ├── tactical.json
│   │   └── ...
│   ├── 20_battle/
│   │   ├── campaign.json
│   │   └── combat.json
│   ├── 30_world/                # 大世界（P3）
│   └── 90_task/                 # 任务入口层：一个任务一个文件
│       ├── commission.json      # {"CommissionStart": {"next": [...]}}
│       └── ...
└── image/                       # 模板图，路径即节点名的小写下划线形式
```

**加载顺序即覆盖顺序**：`00_common` → `10_daily` → … → `90_task`。
同名的后加载覆盖先加载（MaaFW 的 bundle 语义），因此调试补丁可以放在单独的 bundle 里，
不污染主线（`default_pipeline.json` 的「多 Bundle 加载行为」一节）。

## 3. 命名规范

| 类型 | 规则 | 例 |
|---|---|---|
| 领域前缀 | 模块名小写 | `commission` `dorm` `campaign` `os_` |
| 状态节点 | `<domain>_<state>` | `commission_page` `dorm_fill` |
| 入口 | `<Domain>Start`（大驼峰，与任务名一致） | `CommissionStart` |
| 结束 | `<domain>_end` / `<domain>_done` | `commission_done` |
| 恢复 | `<domain>_recover` | `commission_recover` |
| 公共 | `global_*` | `global_home` `global_popup_close` |
| 图文件 | `image/<domain>/<state>.png` | `image/commission/enter.png` |

**一个状态节点对应一张图，节点名 = 图路径去扩展名、斜杠换下划线。**
这条规则让 `xtask` 能自动生成 60% 的节点骨架（A3 §2）。

## 4. 通用约定

`default_pipeline.json`：

```jsonc
{
  "Default": {
    "rate_limit": 1000,        // 同节点最快 1s 重试一次
    "timeout": 20000,          // next 列表识别 20s 超时
    "pre_delay": 200,
    "post_wait_freezes": 500   // 动作后等画面稳定 500ms
  },
  "TemplateMatch": { "threshold": 0.75 },
  "OCR": { "threshold": 0.3 }
}
```

四条强制规则：

1. **每个任务的 `next` 末尾必须有 `[JumpBack]global_recover`**，保证任何意外弹窗都能回主界面。
2. **退出条件用「命中即结束」，绝不用「未命中即结束」**（旧版 CLAUDE.md 同款约束）。
3. **不在节点里做业务计数**。需要「第几页/第几次」→ Rust。
4. **`timeout: -1`（无限等待）只允许出现在 `global_recover`**，其他位置一律有界超时。

锚点约定（`anchor` 与 `[Anchor]`）：

```jsonc
// 在主界面设锚
"global_home": {
  "recognition": "TemplateMatch", "template": "common/home_flag.png",
  "anchor": "home",
  "action": "DoNothing",
  "next": []
}

// 任意任务的失败分支都回到锚点
"commission_recover": { "next": ["[Anchor]home"] }
```

这样「回主界面」不需要在每个任务里重复写一遍路径，也没有旧版 `ui_goto_main()` 的搜索开销。

## 5. 示例一：委托（纯 Pipeline，可 100% 声明式）

场景：进委托 → OCR 四条剩余时间 → 选最短 → 领取 → 重新委托 → 回主界面。

```jsonc
{
  "CommissionStart": {
    "recognition": "DirectHit",
    "next": ["commission_enter", "[Anchor]home"]
  },

  "commission_enter": {
    "recognition": "TemplateMatch",
    "template": "commission/entry_button.png",
    "action": "Click",
    "next": ["commission_page"]
  },

  "commission_page": {
    "recognition": "TemplateMatch",
    "template": "commission/page_title.png",
    "action": "Screencap",                  // 只为驱动下一节点的输入
    "next": ["commission_pick", "commission_empty", "commission_confirm"]
  },

  // 跨帧决策：交给 Rust（见 A4 §5），返回 true 表示已投递点击
  "commission_pick": {
    "recognition": "Custom",
    "custom_recognition": "CommissionPick",
    "action": "DoNothing",
    "next": ["commission_confirm", "[Anchor]home"]
  },

  "commission_empty": {
    "recognition": "OCR",
    "roi": [360, 200, 920, 400],
    "expected": ["没有可领取", "No commission"],
    "action": "DoNothing",
    "next": ["commission_goto_finish"]
  },

  "commission_confirm": {
    "recognition": "TemplateMatch",
    "template": "commission/confirm.png",
    "action": "Click",
    "next": ["commission_page", "commission_goto_finish"]
  },

  "commission_goto_finish": {
    "recognition": "DirectHit",
    "next": ["[Anchor]home"]
  },

  "commission_done": {
    "recognition": "DirectHit",
    "next": []
  }
}
```

要点：

- `commission_pick` 是**唯一**的 Custom 节点，因为它要 OCR 四行文本、解析 `08:00:00`、
  比较取最小、算点击坐标——这是「跨帧计算」，不是「看见就点」。
- 其余 7 个节点没有一行代码。
- `commission_page.next` 同时挂了 `pick`/`empty`/`confirm`，一帧内三者都能被尝试，天然并行判断。

## 6. 示例二：战役（Pipeline 外壳 + Rust 内核）

战役的难点在 `module/map_detection/`（2,783 行：透视校正、单应性、网格预测）和
`module/map/`（5,522 行：寻路、舰队移动）。**这部分无法 Pipeline 化**，但外壳可以：

```jsonc
{
  "CampaignStart": {
    "recognition": "DirectHit",
    "next": ["campaign_battle_prepare", "campaign_select_chapter", "[Anchor]home"]
  },

  "campaign_select_chapter": {
    "recognition": "Custom",
    "custom_recognition": "CampaignChapterSelect",   // 翻页到目标章节 + 关卡
    "action": "DoNothing",
    "next": ["campaign_battle_prepare", "campaign_error"]
  },

  "campaign_battle_prepare": {
    "recognition": "TemplateMatch",
    "template": "campaign/battle_prepare.png",
    "action": "Custom",
    "custom_action": "CampaignPrepare",              // 设置舰队/阵型/自动搜索
    "next": ["campaign_in_map", "campaign_error"]
  },

  // 一次地图决策：识别网格 → A* → 移动舰队 → 返回
  "campaign_in_map": {
    "recognition": "Custom",
    "custom_recognition": "CampaignMapStep",
    "action": "DoNothing",
    "max_hit": 200,
    "next": ["campaign_combat", "campaign_map_end", "campaign_in_map"]
  },

  "campaign_combat": {
    "recognition": "TemplateMatch",
    "template": "campaign/in_combat.png",
    "action": "Custom",
    "custom_action": "CampaignCombat",               // 战斗内循环，直到结算
    "next": ["campaign_in_map", "campaign_result"]
  },

  "campaign_map_end": {
    "recognition": "Custom",
    "custom_recognition": "CampaignFinished",        // 战斗次数用完 / 油尽 / Boss 已清
    "action": "DoNothing",
    "next": ["campaign_result", "campaign_in_map"]
  },

  "campaign_result": {
    "recognition": "TemplateMatch",
    "template": "campaign/result_screen.png",
    "action": "Click",
    "next": ["campaign_done"]
  },

  "campaign_done": { "recognition": "DirectHit", "next": [] }
}
```

**设计意图**：Pipeline 只负责「现在该干哪一类事」，每类事内部是一个完整的 Rust 状态机。
这样的好处是——Rust 侧逻辑可以脱离 MaaFW 做单元测试（`maazure-battle` 不依赖 FFI），
而 Pipeline 层的变化（比如战场 UI 改版）不需要动 Rust。

## 7. 校验与 CI

| 检查 | 工具 | 何时 |
|---|---|---|
| JSON schema | `interface.schema.json` + `pipeline` 自建 schema（`xtask pipeline-check`） | 每次提交 |
| 节点引用完整性 | `xtask pipeline-check`：`next`/`on_error`/`anchor` 引用的节点必须存在；`[Anchor]` 必须有对应 `anchor` 设置点 | 每次提交 |
| 图片存在性 | 同上：`template` 指向的文件必须在 `image/` 下存在 | 每次提交 |
| 孤儿节点 | 同上：无入边且非 `*Start` 的节点报警告 | 每次提交 |
| 可达性 | 从每个 `*Start` 做图遍历，报告不可达节点 | 每次提交 |

`xtask pipeline-check` 是**必须**的：JSON 没有编译期检查，这是唯一的防线。约 200 行 Rust。

## 8. 反模式清单

| 反模式 | 问题 | 正确写法 |
|---|---|---|
| 一个节点里 `next` 挂了 20 个候选 | 每帧做 20 次模板匹配，帧预算爆掉 | 按页面分组，先进页再细辨 |
| 用 `max_hit` 当循环计数 | 语义漂移，日志不可读 | Rust Custom 或拆成有限节点 |
| `timeout: -1` 到处用 | 卡死时永远不报错 | 只给 `global_recover` |
| 模板图从 1280×720 截图里随便裁 | 阈值漂移、误命中 | 用 `dev_tools/button_extract` 同一套裁法，A3 §1 |
| 在 CustomAction 里 `sleep` | 阻塞 C++ 线程，拖慢整条流水线 | `post_click().wait()` 或用 `pre_delay` |
| 节点名 `step1` `step2` | 日志无法定位领域 | 见 §3 命名规范 |
