# 2-battle · 战役与战斗

这里是 **JSON 外壳**：Pipeline 只回答「现在该干哪类事」，每类事内部是
`domain/battle` 的完整状态机。

| 目录 | 节点 | Rust |
|---|---|---|
| `campaign/` | `campaign_*` 9 个节点 | `CampaignMachine` |
| `combat/` | `combat_*` | `CombatMachine` |

示例见 [docs/3-authoring/3.3-examples.md](../../docs/3-authoring/3.3-examples.md) 示例二。
