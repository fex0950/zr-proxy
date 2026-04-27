## Why

当前按 CTRL+C 会直接终止程序但不清理终端状态，导致用户需要手动输入 `reset` 才能恢复正常使用。这是 TUI 软件的标准退出方式，需要支持。

## What Changes

- 新增 `ctrlc` 依赖来监听 SIGINT 信号
- 将事件循环从阻塞式改为非阻塞式（使用 `event::poll`）
- 确保无论通过 Q 键还是 CTRL+C 退出，都能正确清理终端状态
- 可选：使用 `Drop` trait 做双重保险

## Capabilities

### New Capabilities
- `signal-handling`: 处理系统信号（SIGINT/CTRL+C）以支持优雅退出

### Modified Capabilities
- （无）

## Impact

- 新增依赖：`ctrlc` crate
- 修改 `src/main.rs` 的主事件循环
- 修改 `src/app.rs` 的 `AppState`（添加原子标志或使用内部可变性）
