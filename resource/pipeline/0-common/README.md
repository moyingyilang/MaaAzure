# 0-common · 公共地基

所有任务的祖先。**先写这里，再写业务拓扑**——地基没稳就写业务，后面每个任务都要返工。

| 文件 | 节点 | 作用 |
|---|---|---|
| `home.json` | `global_home` | 主界面检测，设置 `anchor: "home"` |
| `popup.json` | `global_popup_close` | 通用弹窗关闭（每个 `next` 末尾挂 `[JumpBack]`） |
| `recover.json` | `global_recover` | 任意失败后的归位，唯一允许 `timeout: -1` 的节点 |
| `login.json` | `global_login` | 启动 / 登录 / 公告 |

规范见 [docs/3-authoring/3.1-pipeline-spec.md](../../docs/3-authoring/3.1-pipeline-spec.md) §5。
