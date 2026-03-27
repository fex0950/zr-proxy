## ADDED Requirements

### Requirement: Discover system applications
系统 SHALL 能够发现 macOS 中安装的应用程序。

#### Scenario: Scan Applications directory
- **WHEN** 用户触发应用发现
- **THEN** 系统扫描 `/Applications` 目录
- **AND** 系统扫描 `~/Applications` 目录
- **AND** 系统返回所有找到的 `.app` 应用

#### Scenario: Use LaunchServices API
- **WHEN** 系统可用 LaunchServices API
- **THEN** 系统优先通过 API 获取已注册应用列表
- **AND** 补充扫描目录中未在 API 中列出的应用

### Requirement: Search and filter applications
系统 SHALL 支持应用列表的搜索和过滤。

#### Scenario: Search by name
- **WHEN** 用户输入搜索关键词
- **THEN** 系统实时过滤显示名称匹配的应用
- **AND** 匹配不区分大小写

#### Scenario: Show application details
- **WHEN** 用户选中一个应用
- **THEN** 系统显示应用名称、版本、路径等基本信息
