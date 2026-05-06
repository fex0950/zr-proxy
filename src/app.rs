use crate::config::{App, Config};
use crate::macos::{copy_to_clipboard, get_launch_command, launch_with_proxy, scan_applications};
use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Mode {
    Normal,
    AppSelector,
    ProxyEditor,
    EnvEditor,
}

pub struct AppState {
    pub config: Config,
    pub mode: Mode,
    pub selected_index: usize,
    pub all_apps: Vec<App>,
    pub app_selector_index: usize,
    pub proxy_input: String,
    pub env_input: String,
    pub should_quit: Arc<AtomicBool>,
    pub pending_quit: bool,
    pub pending_quit_message: String,
    pub search_query: String,
    pub filtered_apps: Vec<App>,
}

impl AppState {
    fn try_quit(&mut self) {
        if self.pending_quit {
            self.should_quit.store(true, Ordering::SeqCst);
        } else {
            self.pending_quit = true;
            self.pending_quit_message = "再按一次 Ctrl+C 或 Q 退出".to_string();
        }
    }

    fn cancel_pending_quit(&mut self) {
        self.pending_quit = false;
        self.pending_quit_message.clear();
    }

    pub fn new() -> Self {
        let config = Config::load();
        let all_apps = scan_applications();
        Self {
            config,
            mode: Mode::Normal,
            selected_index: 0,
            all_apps: all_apps.clone(),
            app_selector_index: 0,
            proxy_input: String::new(),
            env_input: String::new(),
            should_quit: Arc::new(AtomicBool::new(false)),
            pending_quit: false,
            pending_quit_message: String::new(),
            search_query: String::new(),
            filtered_apps: all_apps,
        }
    }

    pub fn handle_key(&mut self, key: KeyEvent) {
        match self.mode {
            Mode::Normal => self.handle_normal_key(key),
            Mode::AppSelector => self.handle_app_selector_key(key),
            Mode::ProxyEditor => self.handle_proxy_editor_key(key),
            Mode::EnvEditor => self.handle_env_editor_key(key),
        }
    }

    fn handle_normal_key(&mut self, key: KeyEvent) {
        match key.code {
            KeyCode::Char('c') if key.modifiers.contains(KeyModifiers::CONTROL) => {
                self.try_quit();
            }
            KeyCode::Char('q') => {
                self.try_quit();
            }
            KeyCode::Up | KeyCode::Char('k') => {
                self.cancel_pending_quit();
                if self.selected_index > 0 {
                    self.selected_index -= 1;
                }
            }
            KeyCode::Down | KeyCode::Char('j') => {
                self.cancel_pending_quit();
                if self.selected_index < self.config.apps.len().saturating_sub(1) {
                    self.selected_index += 1;
                }
            }
            KeyCode::Enter => {
                self.cancel_pending_quit();
                if let Some(app) = self.config.apps.get(self.selected_index) {
                    let _ = launch_with_proxy(app, &self.config.proxy_url, &self.config.env_commands);
                }
            }
            KeyCode::Char('c') if !key.modifiers.contains(KeyModifiers::CONTROL) => {
                self.cancel_pending_quit();
                if let Some(app) = self.config.apps.get(self.selected_index) {
                    let cmd = get_launch_command(app, &self.config.proxy_url, &self.config.env_commands);
                    let _ = copy_to_clipboard(&cmd);
                }
            }
            KeyCode::Char('a') => {
                self.cancel_pending_quit();
                self.mode = Mode::AppSelector;
                self.app_selector_index = 0;
                self.search_query.clear();
                self.filtered_apps = self.all_apps.clone();
            }
            KeyCode::Char('e') => {
                self.cancel_pending_quit();
                self.mode = Mode::ProxyEditor;
                self.proxy_input = self.config.proxy_url.clone();
            }
            KeyCode::Char('v') => {
                self.cancel_pending_quit();
                self.mode = Mode::EnvEditor;
                self.env_input = self.config.env_commands.join("\n");
            }
            KeyCode::Char('d') => {
                self.cancel_pending_quit();
                if !self.config.apps.is_empty() && self.selected_index < self.config.apps.len() {
                    self.config.apps.remove(self.selected_index);
                    if self.selected_index >= self.config.apps.len() && !self.config.apps.is_empty()
                    {
                        self.selected_index = self.config.apps.len() - 1;
                    }
                    let _ = self.config.save();
                }
            }
            _ => {}
        }
    }

    fn handle_app_selector_key(&mut self, key: KeyEvent) {
        match key.code {
            KeyCode::Char('c') if key.modifiers.contains(KeyModifiers::CONTROL) => {
                self.try_quit();
            }
            KeyCode::Esc => {
                self.cancel_pending_quit();
                self.mode = Mode::Normal;
            }
            KeyCode::Up | KeyCode::Char('k') => {
                self.cancel_pending_quit();
                if self.app_selector_index > 0 {
                    self.app_selector_index -= 1;
                }
            }
            KeyCode::Down | KeyCode::Char('j') => {
                self.cancel_pending_quit();
                if self.app_selector_index < self.filtered_apps.len().saturating_sub(1) {
                    self.app_selector_index += 1;
                }
            }
            KeyCode::Enter => {
                self.cancel_pending_quit();
                if let Some(app) = self.filtered_apps.get(self.app_selector_index) {
                    if !self.config.apps.iter().any(|a| a.path == app.path) {
                        self.config.apps.push(app.clone());
                        let _ = self.config.save();
                    }
                }
                self.mode = Mode::Normal;
            }
            KeyCode::Backspace => {
                self.cancel_pending_quit();
                self.search_query.pop();
                self.update_filtered_apps();
            }
            KeyCode::Char(c) => {
                self.cancel_pending_quit();
                self.search_query.push(c);
                self.update_filtered_apps();
            }
            _ => {}
        }
    }

    fn update_filtered_apps(&mut self) {
        if self.search_query.is_empty() {
            self.filtered_apps = self.all_apps.clone();
        } else {
            let query = self.search_query.to_lowercase();
            self.filtered_apps = self
                .all_apps
                .iter()
                .filter(|app| app.name.to_lowercase().contains(&query))
                .cloned()
                .collect();
        }
        self.app_selector_index = 0;
    }

    fn handle_proxy_editor_key(&mut self, key: KeyEvent) {
        match key.code {
            KeyCode::Char('c') if key.modifiers.contains(KeyModifiers::CONTROL) => {
                self.try_quit();
            }
            KeyCode::Esc => {
                self.cancel_pending_quit();
                self.mode = Mode::Normal;
            }
            KeyCode::Enter => {
                self.cancel_pending_quit();
                self.config.proxy_url = self.proxy_input.clone();
                let _ = self.config.save();
                self.mode = Mode::Normal;
            }
            KeyCode::Backspace => {
                self.cancel_pending_quit();
                self.proxy_input.pop();
            }
            KeyCode::Char(c) => {
                self.cancel_pending_quit();
                self.proxy_input.push(c);
            }
            _ => {}
        }
    }

    fn handle_env_editor_key(&mut self, key: KeyEvent) {
        match key.code {
            KeyCode::Char('c') if key.modifiers.contains(KeyModifiers::CONTROL) => {
                self.try_quit();
            }
            KeyCode::Esc => {
                self.cancel_pending_quit();
                self.mode = Mode::Normal;
            }
            KeyCode::Enter => {
                self.cancel_pending_quit();
                self.config.env_commands = self
                    .env_input
                    .lines()
                    .map(|s| s.trim().to_string())
                    .filter(|s| !s.is_empty())
                    .collect();
                let _ = self.config.save();
                self.mode = Mode::Normal;
            }
            KeyCode::Backspace => {
                self.cancel_pending_quit();
                self.env_input.pop();
            }
            KeyCode::Char(c) => {
                self.cancel_pending_quit();
                self.env_input.push(c);
            }
            _ => {}
        }
    }
}
