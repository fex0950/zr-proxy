## 1. 依赖配置

- [x] 1.1 在 Cargo.toml 中添加 `ctrlc` 依赖

## 2. TerminalGuard 实现

- [x] 2.1 创建 `TerminalGuard` 结构体
- [x] 2.2 实现 `Drop` trait 来清理终端

## 3. AppState 修改

- [x] 3.1 在 `AppState` 中添加 `should_quit` 原子标志
- [x] 3.2 确保线程安全

## 4. 信号处理

- [x] 4.1 在 main 中设置 `ctrlc` 回调
- [x] 4.2 回调中设置 `should_quit` 标志

## 5. 事件循环修改

- [x] 5.1 将 `event::read()` 改为 `event::poll()` + `event::read()`
- [x] 5.2 设置超时为 100ms
- [x] 5.3 每次循环都检查 `should_quit` 标志

## 6. 集成 TerminalGuard

- [x] 6.1 在 main 中创建 `TerminalGuard` 实例
- [x] 6.2 确保无论如何退出都清理终端

## 7. 测试

- [x] 7.1 测试按 Q 键退出是否正常
- [x] 7.2 测试按 CTRL+C 退出是否正常
- [x] 7.3 验证终端状态是否正确清理
