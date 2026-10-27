# buildkit（MaaAzure 版）

从 **Termux** 出发，在 Linux chroot 里构建 Rust + MaaFramework 项目。

派生自 [JMNeXt/tools/buildkit](https://github.com/moyingyilang/JMNeXt)（MPL-2.0），
保留其核心思路：**绑定挂载自检 + 产物新鲜度自检**。改造点：JDK/Gradle → rustup/cargo，
Android SDK → MaaFramework SDK。

## 它解决什么

在 Termux 上做需要 glibc 的构建，麻烦从来不是写代码，而是这几件反复发生的事：

1. **容器里看不到宿主机路径**：绑定挂载掉了，报错却是 `cd: 目录不存在`，很容易误判成项目被删；
2. **「看着成功其实没出包」**：构建失败时旧产物还在原地，`ls` 有文件就以为成功；
3. **`libclang` 缺失**：`maa-framework` 用 bindgen，缺 libclang 时报错在几百行深的 build script 里；
4. **MSVC 的 `.lib` 读不了**：Linux 上交叉 Windows 必须用 `features = ["dynamic"]`，见 docs/5-engineering/5.1-build.md §4；
5. **架构错配**：`MAA-linux-aarch64` 的 `.so` 在 x86 容器里跑会报 `Exec format error`。

## 用法

```bash
cp env.example build.env     # 改 PROJECT_DIR / CHROOT_ROOT / RUST_TARGET
./10-enter.sh                # 挂载自检 + 容器内可见性检查
./20-bootstrap.sh            # 一次性：rustup + libclang + mingw + 交叉 target
./30-build.sh release        # 容器内 cargo build
./40-verify.sh               # 架构 / 动态依赖 / 新鲜度 / 冒烟
```

## 目标矩阵

| RUST_TARGET | 用途 | 前提 |
|---|---|---|
| `aarch64-unknown-linux-gnu` | 本机开发、回放测试 | `MAA-linux-aarch64` |
| `x86_64-unknown-linux-gnu` | CI / Linux 用户 | `MAA-linux-x86_64` + `gcc-x86-64-linux-gnu` |
| `x86_64-pc-windows-gnu` | Windows 包（**推荐走 GH Actions MSVC**） | mingw + `dynamic` feature |

## 已验证 / 未验证

- **已验证**（宿主机实测）：chroot 存在且可进入；容器内有 `cmake`/`ninja`/`pkg-config`/
  `aarch64-linux-gnu-gcc`/`qemu-x86_64-static`；NDK `29.0.14206865` 在宿主机 `~/android-sdk/ndk`
- **未验证**：`20-bootstrap.sh` 的 apt 安装（需要容器内网络）、`30-build.sh` 首次全量编译、
  `x86_64-pc-windows-gnu` 交叉链接（R6/R5 风险，见 docs/6-plan/6.3-risks.md）
- **未包含**：GitHub Actions workflow（P1 建），发布打包脚本（P4）

## 许可

**MPL-2.0**（见 `LICENSE`）：按文件传染——修改了本目录的文件，那些文件及其修改需以 MPL 公开；
把它当工具使用、或与自己的代码放在别的文件里，则不受约束。与 GPL/AGPL 兼容（MPL §3.3）。
