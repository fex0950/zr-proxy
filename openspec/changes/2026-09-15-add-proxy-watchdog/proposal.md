## Why

zr-proxy 目前通过 `env VAR=... open -na App --args --proxy-server=...` 注入代理，只对当次启动生效。App 自我更新后重启、或用户从 Dock/Finder 重新打开时，代理参数全部丢失，用户往往无感知，导致流量直连。

macOS 没有持久化命令行参数的官方机制，因此采用"监控 + 提醒 + 一键修复"的兜底方案：常驻后台检测目标 App 是否以无代理方式运行，发现后弹窗提醒，用户确认后自动重启并重新注入代理。

## What Changes

- 新增 `zr-proxy watch` 子命令：前台监控循环，供 launchd 托管运行
- 新增 `zr-proxy install` / `zr-proxy uninstall` 子命令：安装/卸载 LaunchAgent（`~/Library/LaunchAgents/com.zrproxy.watch.plist`，RunAtLoad + KeepAlive）
- 监控循环每 5 秒通过 `ps` 检测监控列表中的 App 主进程是否带代理参数/环境变量运行
- 检测到无代理运行时，通过 `osascript` 弹原生对话框（[立即修复] [忽略]）
- "立即修复"：AppleScript 优雅退出 → 等待进程消失（10s 超时强杀）→ 复用 `launch_with_proxy` 重新拉起
- "忽略"或对话框 120s 超时：对该进程实例冷却，不再重复提醒
- TUI 每次带代理启动 App 时，将其记录到 `config.toml` 的 `watched_apps` 表（含 proxy_url、env_commands 快照）
- watch 日志写入 `~/Library/Logs/zr-proxy-watch.log`

## Capabilities

### New Capabilities
- `proxy-watchdog`: 检测目标 App 无代理运行、通知用户、一键修复重启
- `service-management`: 安装/卸载/查询 launchd 后台监控服务

### Modified Capabilities
- `config-persistence`: 新增 `watched_apps` 持久化（App 路径、可执行名、proxy_url、env_commands 快照）
- `app-launch`: 带代理启动成功后自动将 App 记录到监控列表

## Impact

- `src/main.rs`: 新增子命令分发（无参数时仍进 TUI，保持现有行为）
- `src/config.rs`: `Config` 新增 `watched_apps` 字段
- 新增 `src/macos/watchdog.rs`: 进程检测、通知、修复逻辑
- 新增 `src/macos/service.rs`: LaunchAgent plist 生成与安装/卸载
- 新增文件 `~/Library/LaunchAgents/com.zrproxy.watch.plist`、`~/Library/Logs/zr-proxy-watch.log`
- 无新增第三方依赖（osascript/ps/kill 均为系统自带）
