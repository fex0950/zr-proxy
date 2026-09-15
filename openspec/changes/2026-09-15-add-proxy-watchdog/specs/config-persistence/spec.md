## ADDED Requirements

### Requirement: Persist watched apps
系统 SHALL 将 `watched_apps` 列表随配置文件一同持久化，并向后兼容旧版配置。

#### Scenario: Load legacy config without watched_apps
- **WHEN** 现有 config.toml 不含 `watched_apps` 字段
- **THEN** 加载时视为空列表，不报错

#### Scenario: Save watched apps
- **WHEN** watched_apps 发生变更
- **THEN** 随下次配置保存写入 `~/Library/Application Support/zr-proxy/config.toml`
