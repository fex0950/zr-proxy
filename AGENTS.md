# 仓库贡献指南

## 项目结构与模块组织

本项目是一个使用 Rust 2024 编写的 macOS TUI 应用。

- `src/main.rs`：初始化终端并驱动 UI 事件循环。
- `src/app.rs`：管理应用状态、交互模式和键盘事件。
- `src/config.rs`：定义持久化配置和常用应用条目。
- `src/macos/`：负责 macOS 应用扫描、代理启动和剪贴板操作。
- `src/ui/`：存放布局、主题和可复用 UI 组件。
- `openspec/`：存放变更提案、设计和规格文档。
- `.local/bin/zr-proxy`：全局启动入口；`run.sh` 会在缺少 release 二进制时自动构建。

当前没有单独的测试或资源目录。

## 构建、测试与开发命令

请在仓库根目录执行：

```bash
cargo fmt -- --check  # 检查 rustfmt 格式
cargo check           # 执行类型检查
cargo test            # 运行 Rust 测试
cargo build --release # 构建 target/release/zr-proxy
./run.sh              # 必要时构建并启动 TUI
```

如需从任意目录使用全局入口，请将 `.local/bin` 加入 `PATH`，或链接到 `~/.local/bin`。

## 编码风格与命名约定

遵循 `rustfmt` 默认格式，使用四个空格缩进。函数、变量和模块使用 `snake_case`；类型和枚举成员使用 `PascalCase`；常量使用 `SCREAMING_SNAKE_CASE`。UI 渲染放在 `src/main.rs` 或 `src/ui/`，平台相关进程逻辑放在 `src/macos/`。文件和进程操作边界应明确处理 `Result`。

## 测试指南

使用 Rust 内置测试框架。单元测试应靠近被测代码；新增集成测试放在 `tests/`。测试名称应描述行为，例如 `loads_saved_config`、`builds_launch_command`。提交前运行 `cargo test`；当前没有覆盖率门槛。涉及 TUI 或 macOS 行为时，还应通过 `./run.sh` 手动验证。

## 提交与合并请求规范

提交历史采用简短的 Conventional Commits 前缀，例如 `feat:`、`fix:`、`chore:`，后跟明确描述。每个提交应保持单一目的。合并请求需说明行为变化、列出验证命令，并在适用时关联 issue 或 OpenSpec 变更文档。涉及 TUI 界面时，应附终端截图或简短录屏；涉及持久化配置或代理环境处理时，应明确说明。

## 安全与配置注意事项

用户配置位于 `~/Library/Application Support/zr-proxy/config.toml`。不要提交个人应用路径、代理凭据或机器相关环境值。除非需求明确要求注入代理环境，否则应保持 `env_commands` 为空。
