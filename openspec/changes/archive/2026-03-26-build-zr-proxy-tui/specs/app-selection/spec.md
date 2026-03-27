## ADDED Requirements

### Requirement: Select applications for proxy
系统 SHALL 允许用户选择要通过代理启动的应用。

#### Scenario: Add application to list
- **WHEN** 用户在应用选择器中选择应用并确认
- **THEN** 该应用被添加到常用应用列表
- **AND** 该应用显示在主界面

#### Scenario: Remove application from list
- **WHEN** 用户在主界面选中应用并按删除键
- **THEN** 该应用从常用列表中移除
- **AND** 确认删除前显示提示

### Requirement: Display application list
系统 SHALL 在 TUI 中美观地展示应用列表。

#### Scenario: Default view shows常用 apps
- **WHEN** 用户启动工具
- **THEN** 主界面默认显示已配置的常用应用列表
- **AND** 不显示所有可用应用（需通过命令打开）

#### Scenario: Navigate list with keyboard
- **WHEN** 用户使用方向键
- **THEN** 选中项在列表中上下移动
- **AND** 支持 j/k 键作为备选
