## ADDED Requirements

### Requirement: Save configuration
系统 SHALL 自动保存用户配置。

#### Scenario: Auto-save on change
- **WHEN** 用户修改配置（添加/删除应用、修改代理）
- **THEN** 配置自动保存到磁盘

#### Scenario: Config file location
- **WHEN** 保存配置
- **THEN** 配置文件存储在 `~/Library/Application Support/zr-proxy/config.toml`
- **AND** 目录不存在时自动创建

### Requirement: Load configuration
系统 SHALL 在启动时加载保存的配置。

#### Scenario: Load on startup
- **WHEN** 用户启动工具
- **THEN** 系统从配置文件加载常用应用列表和代理设置
- **AND** 配置文件不存在时使用默认值

#### Scenario: Handle corrupted config
- **WHEN** 配置文件损坏或格式错误
- **THEN** 显示警告信息
- **AND** 使用默认配置启动
- **AND** 备份损坏的配置文件
