## Context

当前 zr-proxy TUI 应用使用阻塞式 `event::read()` 来处理键盘输入。当用户按下 CTRL+C 时，程序直接终止，导致终端停留在 raw mode 和 alternate screen，用户需要手动输入 `reset` 恢复。

## Goals / Non-Goals

**Goals:**
- 支持 CTRL+C 作为标准退出方式
- 确保无论如何退出，终端状态都能正确清理
- 避免需要按两次 CTRL+C 才能退出的问题
- 保持现有 Q 键退出功能不变

**Non-Goals:**
- 不处理除 SIGINT 外的其他信号
- 不实现崩溃恢复机制
- 不改写整个事件循环架构

## Decisions

### 1. 信号处理库选择: `ctrlc` crate
- **选择**: `ctrlc` crate
- **原因**: 最简单直接，专门为处理 CTRL+C 设计
- **备选**: `signal-hook`（功能更强大但对于此需求过于复杂）

### 2. 退出标志: `std::sync::atomic::AtomicBool`
- **选择**: 使用 `AtomicBool` 作为线程安全的退出标志
- **原因**: `ctrlc` 回调在另一个线程运行，需要线程安全
- **备选**: `Mutex<bool>`（性能稍差，对于此场景没必要）

### 3. 事件循环: 改用 `event::poll` + 超时
- **选择**: 将 `event::read()` 改为 `event::poll(Duration::from_millis(100))` + `event::read()`
- **原因**: 允许定期检查退出标志，避免阻塞
- **备选**: 使用非阻塞 I/O 和 mio（过于复杂）

### 4. 终端清理: 双重保险机制
- **选择**: 正常清理 + `Drop` trait 兜底
- **原因**: 即使发生 panic，也能确保终端被清理
- **实现**: 创建 `TerminalGuard` 结构体，在 `drop()` 中执行清理

## Risks / Trade-offs

| 风险 | 缓解措施 |
|------|----------|
| 100ms 超时可能导致轻微的输入延迟 | 100ms 对人类用户不可察觉，可接受 |
| `ctrlc` 回调在多线程环境下的安全性 | 使用 `AtomicBool` 保证线程安全 |
| `Drop` 中的错误无法传播 | 使用 `let _ =` 忽略错误，尽力而为 |
