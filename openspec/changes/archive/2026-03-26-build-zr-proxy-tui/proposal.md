## Why

每次需要通过代理启动 macOS 应用时，都要手动输入 `open -na "App" --args --proxy-server="http://127.0.0.1:7890"` 这样的长命令，很不方便。需要一个简洁的 TUI 工具来管理常用应用的代理启动配置。

## What Changes

- 新增一个 Rust 编写的 TUI 工具 `zr-proxy`
- 支持扫描和选择 macOS 应用程序
- 支持配置代理服务器地址
- 支持保存常用应用列表，快速启动
- 支持生成代理启动命令供复制使用

## Capabilities

### New Capabilities
- `app-discovery`: 发现和列出 macOS 系统中的应用程序
- `app-selection`: TUI 界面选择要配置代理的应用
- `proxy-config`: 管理代理服务器地址配置
- `app-launch`: 通过代理启动选中的应用
- `config-persistence`: 保存和加载用户配置

### Modified Capabilities
- （无）

## Impact

- 新增 Rust 项目，使用 ratatui 作为 TUI 库
- 配置存储在 `~/Library/Application Support/zr-proxy/`
- 依赖 macOS 系统命令 `open` 来启动应用
