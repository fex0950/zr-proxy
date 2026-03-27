## ADDED Requirements

### Requirement: Launch application with proxy
系统 SHALL 能够通过代理启动选中的应用。

#### Scenario: Direct launch from TUI
- **WHEN** 用户选中应用并按回车
- **THEN** 系统使用 `open -na "App" --args --proxy-server="..."` 命令启动应用
- **AND** TUI 可选择保持打开或退出

#### Scenario: Generate launch command
- **WHEN** 用户选择复制命令
- **THEN** 系统生成完整的代理启动命令
- **AND** 命令复制到剪贴板

#### Scenario: Launch without proxy (optional)
- **WHEN** 用户选择正常启动
- **THEN** 系统不添加代理参数直接启动应用

### Requirement: Handle launch errors
系统 SHALL 优雅处理启动失败的情况。

#### Scenario: Application not found
- **WHEN** 应用路径无效或应用已被删除
- **THEN** 显示错误信息
- **AND** 提供从列表中移除该应用的选项
