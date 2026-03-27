## ADDED Requirements

### Requirement: Configure proxy server
系统 SHALL 允许用户配置代理服务器地址。

#### Scenario: Default proxy address
- **WHEN** 用户首次使用工具
- **THEN** 默认代理地址为 `http://127.0.0.1:7890`

#### Scenario: Edit proxy address
- **WHEN** 用户触发编辑代理配置
- **THEN** 显示输入框让用户修改代理地址
- **AND** 支持 HTTP 和 SOCKS 代理格式

#### Scenario: Validate proxy address
- **WHEN** 用户输入代理地址并确认
- **THEN** 系统验证地址格式有效性
- **AND** 无效时显示错误提示

### Requirement: Display current proxy
系统 SHALL 在界面显示当前使用的代理地址。

#### Scenario: Show proxy in status bar
- **WHEN** 工具运行中
- **THEN** 状态栏或配置区域显示当前代理地址
