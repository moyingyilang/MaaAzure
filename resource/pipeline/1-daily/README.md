# 1-daily · 日常任务

一个任务一个子目录：`<domain>/entry.json`（任务入口）+ `<domain>/nodes.json`（业务节点）。

固定的五个节点（缺一即设计没做完）：

```
<domain>_start    入口（DirectHit）
<domain>_enter    从主界面进入
<domain>_page     功能页已确认（公共 next 目标）
<domain>_done     结束（next 为空）
<domain>_recover  异常归位（next: [Anchor]home）
```

节点预算：30–60 个；Custom 节点 **≥3 个说明分类错了**，见
[docs/1-foundation/1.2-boundaries.md](../../docs/1-foundation/1.2-boundaries.md)。
