## ADDED Requirements

### Requirement: Detect unproxied app processes
系统 SHALL 周期检测监控列表中 App 的运行状态，识别以无代理方式运行的进程。

#### Scenario: App running with proxy args
- **WHEN** 目标 App 主进程存在且 args 包含 `--proxy-server=<记录的 proxy_url>`
- **THEN** 判定为有代理运行，不触发提醒

#### Scenario: App running with proxy env
- **WHEN** 目标 App 记录了 env_commands 且进程环境中存在这些变量
- **THEN** 判定为有代理运行，不触发提醒

#### Scenario: App running without proxy
- **WHEN** 目标 App 主进程存在但既无代理参数也无代理环境变量，且该 PID 不在冷却集合中
- **THEN** 弹出提醒对话框

#### Scenario: App not running
- **WHEN** 目标 App 主进程不存在
- **THEN** 跳过检测，不触发提醒

### Requirement: Notify user with actionable dialog
系统 SHALL 通过原生对话框提醒用户，提供"立即修复"和"忽略"两个选项，并在 120 秒无响应时视为忽略。

#### Scenario: User chooses fix
- **WHEN** 用户点击"立即修复"
- **THEN** 系统优雅退出该 App，等待最长 10 秒，超时后强杀，随后以记录的代理配置重新启动

#### Scenario: User chooses ignore
- **WHEN** 用户点击"忽略"或对话框 120 秒超时
- **THEN** 该 PID 加入冷却集合，本次运行期间不再提醒

#### Scenario: New process after ignore
- **WHEN** 被忽略的 App 退出后出现新 PID 且仍无代理
- **THEN** 重新触发提醒

### Requirement: Watchdog logging
系统 SHALL 将检测触发、用户选择、修复结果和错误追加写入 `~/Library/Logs/zr-proxy-watch.log`。

#### Scenario: Fix succeeds
- **WHEN** 修复流程完成
- **THEN** 日志记录 App 名称、原 PID、新启动结果

#### Scenario: Fix fails
- **WHEN** 优雅退出超时后强杀仍失败，或重新启动失败
- **THEN** 日志记录错误详情，监控循环继续运行不退出
