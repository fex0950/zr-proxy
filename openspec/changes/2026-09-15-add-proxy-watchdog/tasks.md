# Proxy Watchdog 实现计划

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** 新增常驻监控:检测到被 zr-proxy 带代理启动过的 App 以无代理方式运行时,弹窗提醒并支持一键修复(重启并恢复代理注入)。

**Architecture:** `zr-proxy watch` 监控循环由 launchd LaunchAgent 托管(`zr-proxy install` 安装);每 5 秒用 `ps` 轮询进程表,发现无代理进程时用 osascript 弹原生对话框;用户确认后优雅退出(超时强杀)并复用 `launch_with_proxy` 重新拉起。TUI 每次带代理启动成功后将 App 快照记录到 `config.toml` 的 `watched_apps`。

**Tech Stack:** Rust 2024,macOS 系统命令(ps / osascript / kill / launchctl / PlistBuddy),无新增第三方依赖。

**测试策略:** 项目无测试基建(设计文档已确认不新增),每个任务以 `cargo build` + `cargo clippy` 验证编译,最终以手动场景清单验收(Task 7)。

---

### Task 1: 配置扩展 — WatchedApp

**Files:**
- Modify: `src/config.rs`

- [ ] **Step 1: 新增 `WatchedApp` 结构体和 `watched_apps` 字段**

在 `src/config.rs` 中,`App` 结构体之后新增:

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WatchedApp {
    pub name: String,
    pub path: PathBuf,
    pub executable: String,
    pub proxy_url: String,
    #[serde(default)]
    pub env_commands: Vec<String>,
}
```

`Config` 结构体新增字段(放在 `apps` 之后):

```rust
    #[serde(default)]
    pub watched_apps: Vec<WatchedApp>,
```

`Config::default()` 的返回体中新增:

```rust
            watched_apps: Vec::new(),
```

- [ ] **Step 2: 新增 upsert 方法**

在 `impl Config` 中(`save` 方法之后)新增:

```rust
    pub fn upsert_watched_app(&mut self, app: &App, executable: String) {
        let entry = WatchedApp {
            name: app.name.clone(),
            path: app.path.clone(),
            executable,
            proxy_url: self.proxy_url.clone(),
            env_commands: self.env_commands.clone(),
        };
        if let Some(existing) = self.watched_apps.iter_mut().find(|a| a.path == entry.path) {
            *existing = entry;
        } else {
            self.watched_apps.push(entry);
        }
    }
```

- [ ] **Step 3: 验证编译**

Run: `cargo build 2>&1 | tail -5`
Expected: `Finished` 无错误

- [ ] **Step 4: Commit**

```bash
git add src/config.rs
git commit -m "feat(config): add WatchedApp and watched_apps persistence"
```

---

### Task 2: 解析 App 可执行名和 bundle id

**Files:**
- Modify: `src/macos/apps.rs`

- [ ] **Step 1: 新增 PlistBuddy 读取辅助函数**

在 `src/macos/apps.rs` 末尾新增:

```rust
fn read_plist_string(bundle_path: &Path, key: &str) -> Option<String> {
    let plist = bundle_path.join("Contents/Info.plist");
    let output = std::process::Command::new("/usr/libexec/PlistBuddy")
        .arg("-c")
        .arg(format!("Print :{}", key))
        .arg(&plist)
        .output()
        .ok()?;
    if !output.status.success() {
        return None;
    }
    let value = String::from_utf8_lossy(&output.stdout).trim().to_string();
    if value.is_empty() { None } else { Some(value) }
}

pub fn resolve_executable(bundle_path: &Path) -> String {
    read_plist_string(bundle_path, "CFBundleExecutable").unwrap_or_else(|| {
        bundle_path
            .file_stem()
            .and_then(|s| s.to_str())
            .unwrap_or_default()
            .to_string()
    })
}

pub fn bundle_identifier(bundle_path: &Path) -> Option<String> {
    read_plist_string(bundle_path, "CFBundleIdentifier")
}
```

- [ ] **Step 2: 验证编译**

Run: `cargo build 2>&1 | tail -5`
Expected: 编译通过(允许 `bundle_identifier` 暂时 dead_code 警告)

- [ ] **Step 3: Commit**

```bash
git add src/macos/apps.rs
git commit -m "feat(macos): resolve bundle executable name and identifier via PlistBuddy"
```

---

### Task 3: TUI 启动成功后记录 watched app

**Files:**
- Modify: `src/app.rs:103-109`(Enter 键分支)
- Modify: `src/app.rs:2`(import)

- [ ] **Step 1: 更新 import**

`src/app.rs` 第 2 行改为:

```rust
use crate::macos::{
    copy_to_clipboard, get_launch_command, launch_with_proxy, resolve_executable, scan_applications,
};
```

- [ ] **Step 2: 修改 Enter 分支,启动成功后写入监控列表**

将 `handle_normal_key` 中的 `KeyCode::Enter` 分支替换为:

```rust
            KeyCode::Enter => {
                self.cancel_pending_quit();
                if let Some(app) = self.config.apps.get(self.selected_index).cloned() {
                    if launch_with_proxy(&app, &self.config.proxy_url, &self.config.env_commands)
                        .is_ok()
                    {
                        let executable = resolve_executable(&app.path);
                        self.config.upsert_watched_app(&app, executable);
                        let _ = self.config.save();
                    }
                }
            }
```

- [ ] **Step 3: 验证编译**

Run: `cargo build 2>&1 | tail -5`
Expected: 编译通过

- [ ] **Step 4: Commit**

```bash
git add src/app.rs
git commit -m "feat(app): record watched app after successful proxied launch"
```

---

### Task 4: 监控核心 — watchdog.rs

**Files:**
- Create: `src/macos/watchdog.rs`
- Modify: `src/macos/mod.rs`

- [ ] **Step 1: 创建 `src/macos/watchdog.rs`(完整文件)**

```rust
use crate::config::{App, Config, WatchedApp};
use crate::macos::launch_with_proxy;
use std::collections::HashSet;
use std::io::Write;
use std::path::PathBuf;
use std::process::Command;
use std::time::Duration;

const POLL_INTERVAL: Duration = Duration::from_secs(5);
const QUIT_TIMEOUT: Duration = Duration::from_secs(10);
const DIALOG_TIMEOUT_SECS: u32 = 120;

#[derive(Debug)]
struct ProcessInfo {
    pid: i32,
    comm: String,
    args: String,
}

fn parse_ps_line(line: &str) -> Option<ProcessInfo> {
    let trimmed = line.trim_start();
    let mut head = trimmed.splitn(2, char::is_whitespace);
    let pid: i32 = head.next()?.trim().parse().ok()?;
    let rest = head.next().unwrap_or("").trim_start();
    let mut parts = rest.splitn(2, char::is_whitespace);
    let comm = parts.next()?.to_string();
    let args = parts.next().unwrap_or("").trim_start().to_string();
    Some(ProcessInfo { pid, comm, args })
}

fn snapshot_processes() -> Vec<ProcessInfo> {
    let output = Command::new("ps")
        .args(["axo", "pid=,comm=,args="])
        .output();
    match output {
        Ok(o) => String::from_utf8_lossy(&o.stdout)
            .lines()
            .filter_map(parse_ps_line)
            .collect(),
        Err(_) => Vec::new(),
    }
}

fn matches_watched(proc: &ProcessInfo, watched: &WatchedApp) -> bool {
    let prefix = format!("{}/Contents/MacOS/", watched.path.to_string_lossy());
    proc.comm.starts_with(&prefix)
        && proc.comm.rsplit('/').next() == Some(watched.executable.as_str())
}

fn process_env_has_any(pid: i32, env_commands: &[String]) -> bool {
    let output = Command::new("ps")
        .args(["eww", "-p", &pid.to_string()])
        .output();
    match output {
        Ok(o) => {
            let text = String::from_utf8_lossy(&o.stdout);
            env_commands.iter().any(|ec| {
                let name = ec.split('=').next().unwrap_or("");
                !name.is_empty() && text.contains(&format!("{}=", name))
            })
        }
        Err(_) => false,
    }
}

fn has_proxy(proc: &ProcessInfo, watched: &WatchedApp) -> bool {
    let flag = format!("--proxy-server={}", watched.proxy_url);
    if proc.args.contains(&flag) {
        return true;
    }
    if !watched.env_commands.is_empty() {
        return process_env_has_any(proc.pid, &watched.env_commands);
    }
    false
}

enum DialogChoice {
    Fix,
    Ignore,
}

fn ask_fix(app_name: &str) -> DialogChoice {
    let script = format!(
        "display dialog \"「{}」正在无代理运行(可能刚更新或重启)。是否重启并恢复代理?\" \
         with title \"zr-proxy\" buttons {{\"忽略\", \"立即修复\"}} \
         default button \"立即修复\" giving up after {}",
        app_name.replace('"', "'"),
        DIALOG_TIMEOUT_SECS
    );
    let output = Command::new("osascript").args(["-e", &script]).output();
    match output {
        Ok(o) if o.status.success() => {
            let text = String::from_utf8_lossy(&o.stdout);
            if text.contains("立即修复") {
                DialogChoice::Fix
            } else {
                DialogChoice::Ignore
            }
        }
        // 用户关闭对话框或 120s 超时(giving up)都会以非零退出,视为忽略
        _ => DialogChoice::Ignore,
    }
}

pub fn log_path() -> PathBuf {
    directories::BaseDirs::new()
        .map(|b| b.home_dir().join("Library/Logs/zr-proxy-watch.log"))
        .unwrap_or_else(|| PathBuf::from("zr-proxy-watch.log"))
}

fn log(message: &str) {
    let timestamp = Command::new("date")
        .arg("+%Y-%m-%d %H:%M:%S")
        .output()
        .ok()
        .map(|o| String::from_utf8_lossy(&o.stdout).trim().to_string())
        .unwrap_or_default();
    if let Ok(mut file) = std::fs::OpenOptions::new()
        .create(true)
        .append(true)
        .open(log_path())
    {
        let _ = writeln!(file, "[{}] {}", timestamp, message);
    }
}

fn pid_alive(pid: i32) -> bool {
    Command::new("kill")
        .args(["-0", &pid.to_string()])
        .status()
        .map(|s| s.success())
        .unwrap_or(false)
}

fn graceful_quit(app: &WatchedApp) {
    let script = match crate::macos::bundle_identifier(&app.path) {
        Some(id) => format!("tell application id \"{}\" to quit", id),
        None => format!(
            "tell application \"{}\" to quit",
            app.name.replace('"', "'")
        ),
    };
    let _ = Command::new("osascript").args(["-e", &script]).status();
}

fn force_kill(pid: i32) {
    let _ = Command::new("kill").arg(pid.to_string()).status();
    std::thread::sleep(Duration::from_secs(2));
    if pid_alive(pid) {
        let _ = Command::new("kill").args(["-9", &pid.to_string()]).status();
    }
}

fn relaunch(app: &WatchedApp) -> Result<(), Box<dyn std::error::Error>> {
    let app_ref = App {
        name: app.name.clone(),
        path: app.path.clone(),
    };
    launch_with_proxy(&app_ref, &app.proxy_url, &app.env_commands)
}

fn fix(app: &WatchedApp, pid: i32) {
    log(&format!("fixing {} (pid {})", app.name, pid));
    graceful_quit(app);
    let deadline = std::time::Instant::now() + QUIT_TIMEOUT;
    while pid_alive(pid) && std::time::Instant::now() < deadline {
        std::thread::sleep(Duration::from_millis(300));
    }
    if pid_alive(pid) {
        log(&format!("graceful quit timed out for {}, force killing", app.name));
        force_kill(pid);
    }
    match relaunch(app) {
        Ok(()) => log(&format!("relaunched {} with proxy", app.name)),
        Err(e) => log(&format!("failed to relaunch {}: {}", app.name, e)),
    }
}

pub fn run_watch_loop() -> ! {
    let mut cooldown: HashSet<i32> = HashSet::new();
    log("watch loop started");
    loop {
        let config = Config::load();
        let processes = snapshot_processes();
        let running: HashSet<i32> = processes.iter().map(|p| p.pid).collect();
        cooldown.retain(|pid| running.contains(pid));

        for watched in &config.watched_apps {
            let Some(proc) = processes.iter().find(|p| matches_watched(p, watched)) else {
                continue;
            };
            if cooldown.contains(&proc.pid) || has_proxy(proc, watched) {
                continue;
            }
            log(&format!(
                "detected unproxied process: {} (pid {})",
                watched.name, proc.pid
            ));
            match ask_fix(&watched.name) {
                DialogChoice::Fix => {
                    fix(watched, proc.pid);
                    // 修复失败时进程可能仍无代理运行,冷却避免连环弹窗
                    cooldown.insert(proc.pid);
                }
                DialogChoice::Ignore => {
                    log(&format!("user ignored {} (pid {})", watched.name, proc.pid));
                    cooldown.insert(proc.pid);
                }
            }
        }
        std::thread::sleep(POLL_INTERVAL);
    }
}
```

- [ ] **Step 2: 声明模块**

`src/macos/mod.rs` 改为:

```rust
pub mod apps;
pub mod launch;
pub mod watchdog;

pub use apps::*;
pub use launch::*;
```

(注意:此处先不导出 `watchdog` 的内容,`run_watch_loop` 和 `log_path` 通过 `crate::macos::watchdog::` 路径使用。)

- [ ] **Step 3: 验证编译**

Run: `cargo build 2>&1 | tail -5`
Expected: 编译通过(允许 `run_watch_loop`/`log_path` 暂时 dead_code 警告)

- [ ] **Step 4: Commit**

```bash
git add src/macos/watchdog.rs src/macos/mod.rs
git commit -m "feat(watchdog): detect unproxied processes, dialog prompt, one-click fix"
```

---

### Task 5: launchd 服务管理 — service.rs

**Files:**
- Create: `src/macos/service.rs`
- Modify: `src/macos/mod.rs`

- [ ] **Step 1: 创建 `src/macos/service.rs`(完整文件)**

```rust
use crate::macos::watchdog::log_path;
use std::path::{Path, PathBuf};
use std::process::Command;

const LABEL: &str = "com.zrproxy.watch";

fn plist_path() -> Result<PathBuf, Box<dyn std::error::Error>> {
    let home = directories::BaseDirs::new()
        .ok_or("cannot resolve home directory")?
        .home_dir()
        .to_path_buf();
    Ok(home
        .join("Library/LaunchAgents")
        .join(format!("{}.plist", LABEL)))
}

fn gui_domain() -> String {
    let uid = Command::new("id")
        .arg("-u")
        .output()
        .ok()
        .map(|o| String::from_utf8_lossy(&o.stdout).trim().to_string())
        .unwrap_or_else(|| "501".to_string());
    format!("gui/{}", uid)
}

fn xml_escape(s: &str) -> String {
    s.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
}

fn render_plist(exe: &Path) -> String {
    let log = log_path();
    format!(
        r#"<?xml version="1.0" encoding="UTF-8"?>
<!DOCTYPE plist PUBLIC "-//Apple//DTD PLIST 1.0//EN" "http://www.apple.com/DTDs/PropertyList-1.0.dtd">
<plist version="1.0">
<dict>
    <key>Label</key>
    <string>{LABEL}</string>
    <key>ProgramArguments</key>
    <array>
        <string>{}</string>
        <string>watch</string>
    </array>
    <key>RunAtLoad</key>
    <true/>
    <key>KeepAlive</key>
    <true/>
    <key>StandardOutPath</key>
    <string>{}</string>
    <key>StandardErrorPath</key>
    <string>{}</string>
</dict>
</plist>
"#,
        xml_escape(&exe.to_string_lossy()),
        xml_escape(&log.to_string_lossy()),
        xml_escape(&log.to_string_lossy()),
    )
}

pub fn install() -> Result<(), Box<dyn std::error::Error>> {
    let exe = std::env::current_exe()?;
    let plist = plist_path()?;
    let domain = gui_domain();

    // 已安装则先卸载,保证可执行路径为最新(幂等)
    let _ = Command::new("launchctl")
        .args(["bootout", &format!("{}/{}", domain, LABEL)])
        .status();

    if let Some(parent) = plist.parent() {
        std::fs::create_dir_all(parent)?;
    }
    std::fs::write(&plist, render_plist(&exe))?;

    let status = Command::new("launchctl")
        .args(["bootstrap", &domain, &plist.to_string_lossy()])
        .status()?;
    if !status.success() {
        return Err(format!("launchctl bootstrap failed: {}", status).into());
    }

    println!("监控服务已安装并启动");
    println!("  plist: {}", plist.display());
    println!("  可执行: {}", exe.display());
    println!("  日志: {}", log_path().display());
    Ok(())
}

pub fn uninstall() -> Result<(), Box<dyn std::error::Error>> {
    let plist = plist_path()?;
    let domain = gui_domain();

    let status = Command::new("launchctl")
        .args(["bootout", &format!("{}/{}", domain, LABEL)])
        .status()?;
    if !status.success() {
        eprintln!("警告: launchctl bootout 返回 {},服务可能未在运行", status);
    }

    if plist.exists() {
        std::fs::remove_file(&plist)?;
    }
    println!("监控服务已卸载: {}", plist.display());
    Ok(())
}
```

- [ ] **Step 2: 声明模块**

`src/macos/mod.rs` 改为:

```rust
pub mod apps;
pub mod launch;
pub mod service;
pub mod watchdog;

pub use apps::*;
pub use launch::*;
```

- [ ] **Step 3: 验证编译**

Run: `cargo build 2>&1 | tail -5`
Expected: 编译通过(允许 service 函数暂时 dead_code 警告)

- [ ] **Step 4: Commit**

```bash
git add src/macos/service.rs src/macos/mod.rs
git commit -m "feat(service): install/uninstall launchd watchdog agent"
```

---

### Task 6: CLI 子命令分发

**Files:**
- Modify: `src/main.rs:52-83`(main 函数)

- [ ] **Step 1: 将现有 TUI 逻辑抽为 `run_tui`,main 增加子命令分发**

把 `src/main.rs` 中现有的 `fn main() { ... }`(第 52-83 行)整体改名为:

```rust
fn run_tui() -> Result<(), Box<dyn std::error::Error>> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let terminal_guard = TerminalGuard::new();

    let mut app = AppState::new();
    let should_quit = app.should_quit.clone();

    ctrlc::set_handler(move || {
        should_quit.store(true, Ordering::SeqCst);
    })?;

    while !app.should_quit.load(Ordering::SeqCst) {
        terminal.draw(|f| ui(f, &mut app))?;

        if event::poll(Duration::from_millis(100))? {
            if let Event::Key(key) = event::read()? {
                app.handle_key(key);
            }
        }
    }

    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    terminal_guard.mark_cleaned_up();

    Ok(())
}
```

在其后新增:

```rust
fn print_usage() {
    eprintln!("用法:");
    eprintln!("  zr-proxy              打开 TUI");
    eprintln!("  zr-proxy watch        运行监控循环(供 launchd 调用)");
    eprintln!("  zr-proxy install      安装并启动后台监控服务");
    eprintln!("  zr-proxy uninstall    停止并移除后台监控服务");
}

fn main() {
    let result: Result<(), Box<dyn std::error::Error>> =
        match std::env::args().nth(1).as_deref() {
            None => run_tui(),
            Some("watch") => {
                macos::watchdog::run_watch_loop();
            }
            Some("install") => macos::service::install(),
            Some("uninstall") => macos::service::uninstall(),
            Some("-h") | Some("--help") => {
                print_usage();
                Ok(())
            }
            Some(other) => {
                eprintln!("未知命令: {}", other);
                print_usage();
                std::process::exit(2);
            }
        };
    if let Err(e) = result {
        eprintln!("错误: {}", e);
        std::process::exit(1);
    }
}
```

- [ ] **Step 2: 验证编译和 lint**

Run: `cargo build 2>&1 | tail -5 && cargo clippy 2>&1 | tail -10`
Expected: 编译通过,clippy 无 warning(dead_code 应已全部消失)

- [ ] **Step 3: 验证 CLI 分发**

Run: `./target/debug/zr-proxy --help`
Expected: 打印 4 行用法说明

- [ ] **Step 4: Commit**

```bash
git add src/main.rs
git commit -m "feat(cli): add watch/install/uninstall subcommands"
```

---

### Task 7: 手动验证清单

**Files:** 无代码改动

前置:构建 release 或直接debug 二进制均可;本机需有一个代理端口(默认 7890)或仅为验证参数注入(代理不通不影响进程检测)。

- [ ] **Step 1: TUI 记录验证**

Run: `./target/debug/zr-proxy`,选一个 App(如 Chrome)按 Enter 带代理启动,然后退出
Expected:
- `ps axo args | grep -i chrome | grep proxy-server` 能看到 `--proxy-server=http://127.0.0.1:7890`
- `cat ~/Library/Application\ Support/zr-proxy/config.toml` 中 `[[watched_apps]]` 含该 App 及 executable、proxy_url、env_commands

- [ ] **Step 2: 有代理不报警**

Run: `./target/debug/zr-proxy watch`(前台另开终端运行),保持 Step 1 启动的 App 开着,观察 30 秒
Expected: 不弹对话框;`~/Library/Logs/zr-proxy-watch.log` 只有 `watch loop started`

- [ ] **Step 3: 无代理触发提醒**

操作:退出该 App,从 Dock 或 Spotlight 重新打开(不带代理)
Expected: 5 秒内弹出 zr-proxy 对话框,含 [忽略] [立即修复] 两个按钮

- [ ] **Step 4: 一键修复**

操作:点 [立即修复]
Expected: App 退出后自动重启;`ps` 确认新进程带 `--proxy-server` 参数;日志记录 fixing / relaunched

- [ ] **Step 5: 忽略与冷却**

操作:再次从 Dock 重启 App(无代理),对话框弹出后点 [忽略]
Expected: 该进程运行期间不再弹窗;彻底退出 App 再打开 → 再次弹窗

- [ ] **Step 6: 服务安装与自启**

Run: `./target/debug/zr-proxy install && launchctl list | grep zrproxy`
Expected: 输出 `com.zrproxy.watch`;杀掉 watch 进程(`pkill -f "zr-proxy watch"`)后几秒内被 launchd 自动拉起(`launchctl list | grep zrproxy` 仍存在新 PID)

- [ ] **Step 7: 服务卸载**

Run: `./target/debug/zr-proxy uninstall && launchctl list | grep zrproxy; ls ~/Library/LaunchAgents/com.zrproxy.watch.plist`
Expected: grep 无输出,plist 文件不存在(ls 报 No such file)

- [ ] **Step 8: 全部通过后提交并归档准备**

```bash
git add -A
git commit -m "chore: verify watchdog end-to-end scenarios" --allow-empty
```

---

## Self-Review 记录

- **Spec 覆盖**: proxy-watchdog(检测/对话框/修复/冷却/日志)→ Task 4 + 7;service-management(install/uninstall/watch 循环/KeepAlive)→ Task 5 + 6 + 7;app-launch(启动记录/可执行名解析)→ Task 2 + 3;config-persistence(字段持久化/旧配置兼容,靠 `#[serde(default)]`)→ Task 1。无缺口。
- **占位符**: 无,所有代码完整给出。
- **类型一致性**: `WatchedApp` 字段(name/path/executable/proxy_url/env_commands)在 Task 1/3/4 一致;`run_watch_loop() -> !`、`log_path() -> PathBuf`、`install()/uninstall()` 签名在 Task 4/5/6 间一致;`resolve_executable`/`bundle_identifier` 在 Task 2 定义,Task 3/4 使用,经由 `macos/mod.rs` 的 `pub use apps::*` 导出。
- **偏离说明**: 未采用 TDD —— 项目无测试基建,设计文档(用户已批准)明确手动验证;plist XML 未做完整转义测试,`xml_escape` 覆盖路径含 `&<>"` 的情况。
