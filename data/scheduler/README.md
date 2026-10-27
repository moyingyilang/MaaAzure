# data/scheduler · 调度规则

任务的优先级与 NextRun 规则。**不写死在代码里**，这样调整排期不需要重新编译。

```jsonc
{
  "priority": ["Restart", "OpsiCrossMonth", "Commission", "Tactical", "Research",
               "Exercise", "Dorm", "Meowfficer", "Guild", "Gacha", "Reward",
               "ShopFrequent", "Main", "Main2", "Main3", "GemsFarming"],
  "when_empty": "stay_there",          // close_game | goto_main | stay_there
  "failure_limit": 3
}
```

行为语义见 [docs/2-architecture/2.2-runtime.md](../../docs/2-architecture/2.2-runtime.md)
与 [docs/6-plan/6.3-risks.md](../../docs/6-plan/6.3-risks.md) 的「有意为之的行为变更」。
