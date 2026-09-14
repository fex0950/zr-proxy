# zr-proxy

一个简洁的 TUI 工具，用于在 macOS 上通过代理启动应用程序。

## 功能特性

- 🚀 快速扫描和选择 macOS 应用程序
- 🔧 配置代理服务器地址
- 💾 保存常用应用列表
- 📋 复制代理启动命令到剪贴板
- ⌨️ 支持 Ctrl+C 优雅退出（防误触）

## 安装

### 一键安装（推荐，无需 Rust 环境）

```bash
curl -fsSL https://raw.githubusercontent.com/fex0950/zr-proxy/master/install.sh | sh
```

脚本会自动识别 Mac 芯片架构（Apple Silicon / Intel），从 [GitHub Releases](https://github.com/fex0950/zr-proxy/releases) 下载预编译二进制并安装到 PATH 目录（优先 `/usr/local/bin`，否则 `~/.local/bin`），安装完成后即可在任意目录运行 `zr-proxy`。

### 从 GitHub 直接安装（需要 Rust 工具链）

无需克隆仓库：

```bash
cargo install --git https://github.com/fex0950/zr-proxy --locked
```

### 从源码构建

```bash
git clone https://github.com/fex0950/zr-proxy.git
cd zr-proxy
cargo build --release
```

可执行文件将位于 `target/release/zr-proxy`

### 全局安装（本地源码）

```bash
cargo install --path . --locked
```

二进制将被安装到 `~/.cargo/bin/zr-proxy`（需确保该目录已在 `PATH` 中），之后可在任意目录直接运行 `zr-proxy`。

代码更新后重新执行上述命令（可加 `--force` 强制覆盖）即可升级。

## 使用方法

```bash
./target/release/zr-proxy
```

### 快捷键

| 按键 | 功能 |
|------|------|
| `↑`/`↓` 或 `k`/`j` | 导航列表 |
| `Enter` | 通过代理启动选中的应用 |
| `C` | 复制启动命令到剪贴板 |
| `A` | 添加新应用 |
| `E` | 编辑代理地址 |
| `D` | 删除应用 |
| `Q` 或 `Ctrl+C` | 退出（需要按两次确认） |

## 配置

配置文件保存在 `~/Library/Application Support/zr-proxy/config.toml`

## 技术栈

- Rust
- [ratatui](https://github.com/ratatui-org/ratatui) - TUI 库
- [crossterm](https://github.com/crossterm-rs/crossterm) - 终端操作

## 许可证

MIT
