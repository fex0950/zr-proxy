## ADDED Requirements

### Requirement: CTRL+C 优雅退出
系统 SHALL 支持通过 CTRL+C (SIGINT) 信号优雅退出应用。

#### Scenario: 用户按下 CTRL+C 退出
- **WHEN** 用户在应用运行时按下 CTRL+C
- **THEN** 应用设置退出标志
- **AND** 应用在下一次事件循环迭代中检测到退出标志
- **AND** 应用正确清理终端状态（disable raw mode、leave alternate screen）
- **AND** 应用正常退出，返回 0 状态码

#### Scenario: 避免按两次 CTRL+C
- **WHEN** 用户按下 CTRL+C
- **THEN** 应用在最多 100ms 内检测到退出标志并开始退出流程
- **AND** 用户不需要按第二次 CTRL+C

### Requirement: 终端状态清理保证
无论通过何种方式退出（Q 键、CTRL+C、panic），系统 SHALL 确保终端状态被正确清理。

#### Scenario: 通过 Q 键退出
- **WHEN** 用户按 Q 键退出
- **THEN** 终端被正确清理

#### Scenario: 通过 CTRL+C 退出
- **WHEN** 用户按 CTRL+C 退出
- **THEN** 终端被正确清理

#### Scenario: 发生 panic 时
- **WHEN** 应用发生 panic
- **THEN** 终端仍然被正确清理（尽力而为）
