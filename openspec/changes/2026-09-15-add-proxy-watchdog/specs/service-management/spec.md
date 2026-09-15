## ADDED Requirements

### Requirement: Install watchdog service
系统 SHALL 提供 `zr-proxy install` 命令将监控服务安装为当前用户的 LaunchAgent。

#### Scenario: Fresh install
- **WHEN** 用户执行 `zr-proxy install` 且 plist 不存在
- **THEN** 系统生成 `~/Library/LaunchAgents/com.zrproxy.watch.plist`（RunAtLoad + KeepAlive，指向当前可执行文件的 `watch` 子命令）并 `launchctl load`
- **AND** 输出服务状态提示

#### Scenario: Re-install over existing
- **WHEN** 用户执行 `zr-proxy install` 且 plist 已存在
- **THEN** 系统覆盖重写 plist 并重新加载，保证可执行文件路径为最新

### Requirement: Uninstall watchdog service
系统 SHALL 提供 `zr-proxy uninstall` 命令移除监控服务。

#### Scenario: Uninstall
- **WHEN** 用户执行 `zr-proxy uninstall`
- **THEN** 系统 `launchctl unload` 并删除 plist 文件
- **AND** 已运行的 watch 进程被终止，不再自动拉起

### Requirement: Watch command runs monitoring loop
系统 SHALL 提供 `zr-proxy watch` 前台子命令执行监控循环，供 launchd 托管。

#### Scenario: Watch started by launchd
- **WHEN** launchd 启动 `zr-proxy watch`
- **THEN** 系统加载配置中的 watched_apps 并进入 5 秒轮询循环

#### Scenario: Watch process crashes
- **WHEN** watch 进程异常退出
- **THEN** launchd 依据 KeepAlive 自动重启该进程
