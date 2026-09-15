## Context

zr-proxy 现有启动方式为一次性注入（`src/macos/launch.rs`）：环境变量和 `--proxy-server` 参数只对当次 `open` 生效。App 自我更新重启或用户从 Dock 重新打开后代理丢失。

已确认的决策（来自需求讨论）：
- 方案选型：**通知 + 一键修复**（而非 Info.plist / Wrapper / 系统代理等持久化方案）
- 常驻方式：**launchd LaunchAgent**
- 通知实现：**osascript 原生对话框**（零依赖，支持按钮回调）

## Goals / Non-Goals

**Goals:**
- 目标 App 以无代理方式运行时被及时发现并提醒用户
- 用户一键确认后自动重启 App 并恢复代理注入
- 监控服务开机自启、崩溃自愈，无需用户值守
- 不重复骚扰：同一进程实例只提醒一次

**Non-Goals:**
- 不追求代理永不丢失（那是持久化方案的事，后续可另行评估）
- 不监控未通过 zr-proxy 启动过的 App
- 不支持自动修复无需确认（避免打断用户工作，修复必须用户点击确认）
- 不处理 App 内部自己的代理设置

## Decisions

### 1. 监控列表来源：TUI 启动时自动记录
- **选择**: TUI 每次带代理启动成功后，将 App 写入 `config.toml` 的 `watched_apps`
- **结构**:
  ```toml
  [[watched_apps]]
  name = "Chrome"
  path = "/Applications/Google Chrome.app"
  executable = "Google Chrome"        # 主二进制名，用于 ps 匹配
  proxy_url = "http://127.0.0.1:7890"
  env_commands = ["HTTP_PROXY=...", ...]
  ```
- **原因**: 用户意图最明确——"我用代理启动过它，说明我在乎它走代理"；无需额外 UI 管理监控列表
- **备选**: 独立的监控列表管理界面（YAGNI，后续需要再加）

### 2. 常驻方式：launchd LaunchAgent
- **选择**: `zr-proxy install` 生成 `~/Library/LaunchAgents/com.zrproxy.watch.plist`，`RunAtLoad=true` + `KeepAlive=true`，ProgramArguments 指向当前可执行文件的绝对路径 + `watch` 参数
- **原因**: macOS 标准做法，登录自启、崩溃自愈、无需 root（用户域）
- **卸载**: `zr-proxy uninstall` 执行 `launchctl unload` + 删除 plist
- **注意**: 可执行文件路径变化（如重新编译到不同位置）后需重新 install；install 时若已存在则覆盖重写

### 3. 检测方式：轮询 ps（5 秒间隔）
- **选择**: `ps axo pid=,comm=,args=` 匹配目标 App 主进程（comm 等于 executable 名，且路径在 watched path 下）
- **判定"有代理"**（满足其一即可）:
  - 进程 args 包含 `--proxy-server=<记录的 proxy_url>`
  - 记录了 env_commands 时，`ps eww <pid>` 输出中包含其中任一变量名
- **原因**: 纯用户态、无需权限（同用户进程可读 env）；轮询 5s 的 CPU 开销可忽略
- **备选**: FSEvents/kqueue 监听进程事件（macOS 无简单进程启动通知 API，需 Endpoint Security 框架，需 entitlement，过度设计）

### 4. 通知与交互：osascript 原生对话框
- **选择**: `osascript -e 'display dialog "..." buttons {"忽略", "立即修复"} default button "立即修复" giving up after 120'`
- **原因**: 零依赖；按钮结果直接从 osascript stdout 返回，天然支持"一键修复"
- **取舍**: 对话框是模态的，比横幅通知显眼——这是刻意的，因为代理丢失属于需要用户决策的事件；120s 不点自动视为"忽略"并冷却，不会堆积
- **备选**: terminal-notifier（需额外安装且停止维护）、自建 .app + UNUserNotificationCenter（工程量大）

### 5. 修复流程：优雅退出 → 超时强杀 → 重新拉起
1. `osascript -e 'tell application "<name>" to quit'`（按 bundle id 更稳，用 bundle 路径 `tell application id` 或 `using terms from`；实现时按可执行路径取 bundle id）
2. 轮询等待进程消失，最长 10 秒
3. 超时后 `kill <pid>`（仍不消失再 `kill -9`）
4. 复用现有 `launch_with_proxy` 以记录的 proxy_url/env_commands 重新启动
- **取舍**: 优雅退出依赖 App 响应 AppleScript quit，个别 App 可能弹自己的确认框——属 macOS 机制，由强杀兜底

### 6. 冷却机制：按进程实例去重
- **选择**: 提醒过的 PID 记入内存冷却集合；用户点"忽略"或对话框超时后也记入
- App 修复重启后出现新 PID → 新进程带代理，自然不再触发
- 用户点"忽略"后该进程继续无代理运行 → 不再提醒，直到 App 下次重启产生新 PID
- watch 进程自身重启（崩溃/重开机）→ 冷却清空，重新检测一遍（可接受）

### 7. 日志
- **选择**: 追加写 `~/Library/Logs/zr-proxy-watch.log`，记录检测触发、用户选择、修复结果、错误
- **原因**: launchd 托管进程无终端，排障全靠日志；不引入日志框架，简单封装即可

## 项目结构变化

```
src/
├── main.rs              # 新增子命令分发: watch / install / uninstall
├── config.rs            # Config 新增 watched_apps: Vec<WatchedApp>
├── macos/
│   ├── watchdog.rs      # 新增: 检测循环、osascript 对话框、修复流程、冷却
│   └── service.rs       # 新增: plist 生成、launchctl load/unload
```

## Risks / Trade-offs

| 风险 | 缓解措施 |
|------|----------|
| `ps eww` 读其他进程 env 在未来 macOS 版本受限 | 同用户进程目前可读；若失效，降级为仅按 args 判定（env 类 App 会误报，日志中说明） |
| App 更新后可执行名变化导致 ps 匹配不到 | 匹配失败视为"未运行"，不误报；极端情况用户重新从 TUI 启动一次即更新记录 |
| osascript 弹窗在锁屏/全屏时体验差 | 对话框排队显示属系统行为；120s 超时兜底 |
| 修复时用户有未保存内容 | 优雅退出给 App 保存/确认机会；强杀仅作为 10s 超时后的兜底 |
| 可执行文件被移动后 launchd 指向失效 | install 幂等重写 plist；日志记录启动失败原因 |
