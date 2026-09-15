## 1. 配置扩展 (config-persistence)

- [ ] 1.1 `config.rs` 新增 `WatchedApp` 结构体（name, path, executable, proxy_url, env_commands）
- [ ] 1.2 `Config` 新增 `watched_apps: Vec<WatchedApp>`（`#[serde(default)]` 保证旧配置兼容）
- [ ] 1.3 新增/更新 watched app 的 upsert 方法（按 path 去重，更新 proxy 快照）

## 2. 启动时记录 (app-launch)

- [ ] 2.1 TUI 带代理启动成功后调用 upsert，将 App 及当前 proxy_url/env_commands 写入 watched_apps 并保存配置
- [ ] 2.2 从 App 的 Info.plist 或路径推导主可执行名（CFBundleExecutable，失败时回退 bundle 文件名去 .app）

## 3. 监控循环 (proxy-watchdog)

- [ ] 3.1 `src/macos/watchdog.rs`：`ps axo pid=,comm=,args=` 解析进程表，匹配 watched_apps
- [ ] 3.2 有代理判定：args 含 `--proxy-server=<url>` 或 `ps eww <pid>` 含记录的 env 变量
- [ ] 3.3 5 秒轮询主循环 + 按 PID 的冷却集合（忽略/超时后不再提醒同进程）
- [ ] 3.4 osascript 对话框：标题+说明，按钮 [忽略] [立即修复]，120s 超时视为忽略
- [ ] 3.5 修复流程：AppleScript quit → 等待退出（10s）→ kill / kill -9 兜底 → `launch_with_proxy` 重启
- [ ] 3.6 日志：触发、选择、修复结果、错误追加写入 `~/Library/Logs/zr-proxy-watch.log`

## 4. launchd 服务 (service-management)

- [ ] 4.1 `src/macos/service.rs`：生成 LaunchAgent plist（Label com.zrproxy.watch，RunAtLoad + KeepAlive，标准输出/错误重定向到日志文件）
- [ ] 4.2 `zr-proxy install`：写 plist 到 `~/Library/LaunchAgents/`（已存在则覆盖）+ `launchctl load`
- [ ] 4.3 `zr-proxy uninstall`：`launchctl unload` + 删除 plist
- [ ] 4.4 install 时校验当前可执行文件路径，并在输出中提示用户服务状态

## 5. CLI 入口

- [ ] 5.1 `main.rs` 子命令分发：无参数 → TUI（现状）；`watch` → 监控循环；`install` / `uninstall` → 服务管理

## 6. 验证

- [ ] 6.1 `cargo build` 通过，`cargo clippy` 无警告
- [ ] 6.2 手动验证：TUI 带代理启动 App → watched_apps 已记录且 watch 不报警
- [ ] 6.3 手动验证：从 Dock 重启该 App → 5 秒内弹出对话框
- [ ] 6.4 手动验证：点"立即修复" → App 退出并以代理参数重启（ps 确认）
- [ ] 6.5 手动验证：点"忽略" → 该进程不再提醒；再次重启 App → 重新提醒
- [ ] 6.6 手动验证：install 后 `launchctl list | grep zrproxy` 存在；重启登录会话后服务自启；uninstall 后干净移除
