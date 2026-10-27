# CI 配置（待启用）

`ci/ci.yml` 是完整的 CI 定义：格式、clippy、单元测试、`pipeline-check`、零 Python 门禁，
以及一个带 MaaFramework SDK 编译 `--features maa` 的 FFI job。

## 为什么放在这里而不是 `.github/workflows/`

推送 `.github/workflows/**` 要求 token 带 **`workflow` scope**。当前这台机器上的
`gh` 登录态是 OAuth App token，只有 `gist / project / read:org / repo`，
GitHub 直接拒绝：

```
refusing to allow an OAuth App to create or update workflow
`.github/workflows/ci.yml` without `workflow` scope
```

## 启用方式（任选其一）

```bash
# A. 给现有登录态补 scope（会走一次浏览器设备码流程）
gh auth refresh -h github.com -s workflow
mv ci/ci.yml .github/workflows/ci.yml
git add .github/workflows/ci.yml && git commit -m "ci: 启用 GitHub Actions"
git push

# B. 在网页上直接新建 .github/workflows/ci.yml，粘贴 ci/ci.yml 的内容
```

启用后建议把 `ci/ci.yml` 删掉，避免两份配置漂移。

## 两个 job 在验证什么

| job | 验证 | 失败意味着 |
|---|---|---|
| `test` | fmt / clippy -D warnings / `cargo test --workspace` / `pipeline-check` / 零 Python | 代码质量或资源一致性退化 |
| `ffi` | 下 `MAA-linux-x86_64` SDK 后编 `--features maa` | FFI 层被悄悄改坏（本机 Termux 编不了，这是唯一的守门人） |
