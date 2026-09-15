## ADDED Requirements

### Requirement: Record watched app on proxied launch
系统 SHALL 在通过 TUI 带代理启动 App 成功后，将该 App 记录到配置文件的 `watched_apps` 中。

#### Scenario: First proxied launch
- **WHEN** 用户通过 TUI 带代理启动某 App 且该 App 不在 watched_apps 中
- **THEN** 系统记录 App 名称、bundle 路径、主可执行名及当前 proxy_url、env_commands 快照并保存配置

#### Scenario: Repeated proxied launch
- **WHEN** 用户再次带代理启动已在 watched_apps 中的 App
- **THEN** 系统按 bundle 路径去重，更新 proxy_url 和 env_commands 快照

#### Scenario: Normal launch without proxy
- **WHEN** 用户选择不带代理启动
- **THEN** 系统不修改 watched_apps

### Requirement: Resolve executable name
系统 SHALL 在记录 watched app 时解析其主可执行文件名，用于进程匹配。

#### Scenario: Read from Info.plist
- **WHEN** App bundle 的 Info.plist 存在且包含 CFBundleExecutable
- **THEN** 使用该值作为可执行名

#### Scenario: Fallback to bundle name
- **WHEN** Info.plist 缺失或解析失败
- **THEN** 回退使用 bundle 文件名（去除 .app 后缀）作为可执行名
