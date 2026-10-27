# data/campaign · 战役地图

由旧仓 `campaign/*.py`（1,436 个）经 `xtask convert-campaign` 提取而来。
**数据不编译进代码**：新增活动 = 加目录 + 重跑转换器，不改 Rust。

```
campaign/
├── campaign_main/         主线
├── campaign_hard/         困难
├── campaign_sos/          SOS
├── campaign_war_archives/ 作战档案
└── campaign_event/        活动（按当前活动增量添加，不追溯历史）
```

单个文件形状见 [docs/4-migration/4.3-campaign-data.md](../../docs/4-migration/4.3-campaign-data.md)。
