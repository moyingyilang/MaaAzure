# A5 · 构建与交叉编译

## 1. 环境现状（实测，非假设）

| 项 | 位置 / 版本 | 状态 |
|---|---|---|
| 宿主机 | Termux aarch64（Android bionic） | Rust 1.98.1 可编译纯 Rust，**不能链接 glibc 的 MaaFW** |
| chroot | `~/.local/share/tmoe-linux/containers/chroot/debian-bookworm_arm64` | ✅ 有 cmake / ninja / pkg-config / aarch64-linux-gnu-gcc |
| 用户态模拟 | 容器内 `qemu-x86_64-static` 等全套 | ✅ 可跑 x64 二进制（慢） |
| NDK | `~/android-sdk/ndk/29.0.14206865` | ✅ 可构建 `aarch64-linux-android` |
| **缺** | `libclang`（bindgen 需要）、`mingw-w64` | ❌ 需在容器内 `apt install` |
| MAA SDK | `MAA-linux-aarch64-*.zip` 等 5 个平台包 | 从 [Releases](https://github.com/MaaXYZ/MaaFramework/releases) 下载 |

结论：**开发在 chroot 内、发布在 CI、本机只做逻辑验证。**

## 2. 目标矩阵

| target | 用途 | 构建位置 | 说明 |
|---|---|---|---|
| `aarch64-unknown-linux-gnu` | 本机开发、golden corpus 回放 | chroot 原生 | 有 `MAA-linux-aarch64`，可跑真 MaaFW |
| `x86_64-unknown-linux-gnu` | CI、Linux 用户 | chroot 交叉 或 GH Actions | 交叉需 x64 链接器，见 §4 |
| `x86_64-pc-windows-gnu` | **发布主目标**（用户是 Windows 模拟器） | GH Actions `windows-latest`（MSVC）**首选** | 见 §5 |
| `aarch64-linux-android` | 远期（镜像 EndPilot 路线，框架跑在手机上） | chroot + NDK | 需要 JNI 壳，非 P0 范围 |

## 3. chroot 构建环境（复用 JMNeXt buildkit）

`tools/buildkit/` 已按本项目改造，沿用同一套「绑定挂载 + 容器内自检」的思路
（原工具来自 [JMNeXt](https://github.com/moyingyilang/JMNeXt) `tools/buildkit`，MPL-2.0，见该目录 LICENSE）。

```bash
cd tools/buildkit
cp env.example build.env        # 改 PROJECT_DIR / CHROOT_ROOT 等
./10-enter.sh                   # 挂载自检 + 进容器（首次会提示缺什么）
./20-bootstrap.sh               # 容器内一次性：rustup + libclang + mingw + 工具
./30-build.sh [debug|release]   # 容器内 cargo build
./40-verify.sh                  # 产物自检：ELF 架构、依赖库、能否 --version
```

`10-enter.sh` 的自检必须做两件事（踩过的坑，直接继承）：

1. **宿主机 → 容器可见性检查**：绑定挂载掉了时报错是 `目录不存在`，极易误判成项目被删；
2. **产物新鲜度检查**：构建失败时旧产物还在原地，`ls` 一看有文件就以为成功。

## 4. 容器内依赖安装（一次性）

```bash
# 容器内、root
apt-get update
apt-get install -y build-essential pkg-config libssl-dev \
                   clang libclang-dev llvm \      # bindgen 需要 libclang
                   mingw-w64 \                    # x86_64-pc-windows-gnu
                   gcc-x86-64-linux-gnu \         # x86_64-unknown-linux-gnu
                   unzip curl git
# rustup（不要用发行版自带的旧 rustc）
curl --proto '=https' -sSf https://sh.rustup.rs | sh -s -- -y --profile minimal
rustup target add x86_64-unknown-linux-gnu x86_64-pc-windows-gnu aarch64-linux-android
```

**为什么必须 `dynamic` feature 做 Windows 交叉**：`maa-framework` 的静态链接需要 MaaFW 的
`.dll` + **MSVC 导入库 `.lib`**，mingw 的 `ld` 读不了 MSVC 的 `.lib`。
改用 `features = ["dynamic"]` 后在运行时 `libloading` 加载 `MaaFramework.dll`，
构建期就不再需要导入库——这是 arm64 Linux 上出 Windows 包的唯一现实路径。

> **待 P0 验证**：`dynamic` feature 是否真的让 build 阶段不依赖 `.lib`（RS 绑定文档只写了
> 「手动 `load_library`」，未明说 build 脚本行为）。若验证失败，Windows 包一律走 GH Actions，
> 本机只出 Linux 产物——不影响主线。

## 5. 发布流程（推荐形态）

```yaml
# .github/workflows/release.yml（P1 建立）
strategy:
  matrix:
    include:
      - os: windows-latest, target: x86_64-pc-windows-msvc
      - os: ubuntu-latest,  target: x86_64-unknown-linux-gnu
      - os: macos-14,       target: aarch64-apple-darwin
steps:
  - 下载对应平台 MAA SDK 并解压到 ./MAA-<platform>/
  - cargo build --release --target ${{ matrix.target }}
  - 组装发布目录（§6）
  - 打 zip + 生成 sha256
```

**不**做：让用户自己装 MaaFW。发布包自带 SDK，版本固定（A1 §8）。

## 6. 产物布局

```
MaaAzure-v0.1.0-win-x64/
├── MaaAzure.exe
├── MaaFramework.dll
├── MaaToolkit.dll            # 设备发现
├── resource/                 # pipeline + image
├── data/                     # campaign / scheduler
├── interface.json
└── config/                   # 用户配置（首次启动生成）
```

`sccache` 在 arm64 chroot 里收益有限（首次编译全量），但**增量编译**很关键：
`cargo build` 单次约 40–80s（chroot 原生 arm64），够用。
**不要**在 Termux 侧直接 `cargo build` 整个 workspace——链接阶段必然失败在 `MaaFramework.so`。

## 7. 版本号

| 号 | 位置 | 谁改 |
|---|---|---|
| 项目版本 | `interface.json` 的 `version` | 发布脚本 |
| 代码版本 | `Cargo.toml` workspace `version` | 发布脚本 |
| 协议版本 | `interface.json` 的 `interface_version: 2` | 只在 PI 主版本变化时改 |
| SDK 版本 | `MAA-*/README` 记录 | 升级 MaaFW 时人工确认 |

发布脚本 `xtask release` 负责把前三个改成同一个值，并断言 SDK 版本在支持范围内
（MaaFW 5.x + `maa-framework` 1.x）。
