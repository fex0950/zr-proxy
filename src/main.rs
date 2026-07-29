use crossterm::{
    event::{self, Event},
    execute,
    terminal::{EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode},
};
use ratatui::{
    Terminal,
    backend::CrosstermBackend,
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    widgets::{Block, BorderType, Borders, List, ListItem, ListState, Paragraph},
};
use std::io;
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use std::time::Duration;

mod app;
mod config;
mod macos;
mod ui;

struct TerminalGuard {
    cleanup_done: Arc<AtomicBool>,
}

impl TerminalGuard {
    fn new() -> Self {
        Self {
            cleanup_done: Arc::new(AtomicBool::new(false)),
        }
    }

    fn mark_cleaned_up(&self) {
        self.cleanup_done.store(true, Ordering::SeqCst);
    }
}

impl Drop for TerminalGuard {
    fn drop(&mut self) {
        if !self.cleanup_done.load(Ordering::SeqCst) {
            let _ = disable_raw_mode();
            let _ = execute!(io::stdout(), LeaveAlternateScreen);
        }
    }
}

use app::{AppState, Mode};
use ui::components::titled_block;
use ui::layout::{content_layout, main_layout};
use ui::theme::Theme;

fn main() -> Result<(), Box<dyn std::error::Error>> {
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

fn ui(f: &mut ratatui::Frame, app: &mut AppState) {
    let area = f.area();
    let chunks = main_layout(area);

    render_header(f, chunks[0], app);
    render_content(f, chunks[1], app);
    render_footer(f, chunks[2], app);

    if app.mode == Mode::AppSelector {
        render_app_selector(f, area, app);
    } else if app.mode == Mode::ProxyEditor {
        render_proxy_editor(f, area, app);
    } else if app.mode == Mode::EnvEditor {
        render_env_editor(f, area, app);
    } else if app.mode == Mode::GlobalCommandRunner {
        render_global_command_runner(f, area, app);
    } else if app.mode == Mode::GlobalCommandEditor {
        render_global_command_editor(f, area, app);
    }

    if app.pending_quit {
        render_pending_quit_notice(f, area, app);
    }
}

fn render_header(f: &mut ratatui::Frame, area: Rect, app: &AppState) {
    let title = Paragraph::new("🚀 zr-proxy  v0.1.0")
        .style(Theme::purple())
        .alignment(Alignment::Left);

    let proxy_info = Paragraph::new(format!("[代理: {}]", app.config.proxy_url))
        .style(Theme::dim())
        .alignment(Alignment::Right);

    let block = Block::default()
        .borders(Borders::BOTTOM)
        .border_type(BorderType::Rounded)
        .border_style(Theme::border())
        .style(Theme::bg());

    f.render_widget(block, area);
    f.render_widget(title, area);
    f.render_widget(proxy_info, area);
}

fn render_content(f: &mut ratatui::Frame, area: Rect, app: &AppState) {
    let chunks = content_layout(area);

    let items: Vec<ListItem> = app
        .config
        .apps
        .iter()
        .map(|app_item| ListItem::new(format!("●  {}", app_item.name)).style(Theme::text()))
        .collect();

    let list = List::new(items)
        .block(titled_block("常用应用"))
        .highlight_style(Theme::selected())
        .highlight_symbol("> ");

    let mut list_state = ListState::default();
    list_state.select(Some(app.selected_index));

    f.render_stateful_widget(list, chunks[0], &mut list_state);

    let details = if let Some(selected_app) = app.config.apps.get(app.selected_index) {
        let env_cmds = if app.config.env_commands.is_empty() {
            "无".to_string()
        } else {
            app.config.env_commands.join(", ")
        };
        let global_cmds_info = if app.config.global_commands.is_empty() {
            String::new()
        } else {
            format!("\n全局命令: {} 条", app.config.global_commands.len())
        };
        format!(
            "名称: {}\n路径: {}\n环境命令: {}{}\n\n[Enter] 通过代理启动\n[C]     复制启动命令",
            selected_app.name,
            selected_app.path.to_string_lossy(),
            env_cmds,
            global_cmds_info,
        )
    } else {
        "暂无应用\n按 [A] 添加应用".to_string()
    };

    let details_widget = Paragraph::new(details)
        .block(titled_block("应用信息"))
        .style(Theme::text());

    f.render_widget(details_widget, chunks[1]);
}

fn render_footer(f: &mut ratatui::Frame, area: Rect, _app: &AppState) {
    let help = Paragraph::new(
        "[A] 添加应用  [E] 编辑代理  [V] 环境命令  [G] 全局命令  [D] 删除应用  [Q] 退出",
    )
    .style(Theme::dim())
    .alignment(Alignment::Center);

    let block = Block::default()
        .borders(Borders::TOP)
        .border_type(BorderType::Rounded)
        .border_style(Theme::border())
        .style(Theme::bg());

    f.render_widget(block, area);
    f.render_widget(help, area);
}

fn render_app_selector(f: &mut ratatui::Frame, area: Rect, app: &AppState) {
    let popup_area = centered_rect(area, 70, 80);

    // Split popup into search and list areas
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(3), Constraint::Min(5)])
        .split(popup_area);

    // Render search input
    let search_input = Paragraph::new(app.search_query.as_str())
        .style(Theme::text())
        .block(
            Block::default()
                .title(format!(
                    " 搜索 (找到 {} 个应用) [Esc 取消]",
                    app.filtered_apps.len()
                ))
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded)
                .border_style(Theme::border())
                .style(Theme::bg_dark()),
        );

    f.render_widget(search_input, chunks[0]);

    // Render app list from filtered_apps
    let items: Vec<ListItem> = app
        .filtered_apps
        .iter()
        .map(|app_item| {
            let is_selected = app.config.apps.iter().any(|a| a.path == app_item.path);
            let marker = if is_selected { "[x]" } else { "[ ]" };
            ListItem::new(format!("{}  {}", marker, app_item.name)).style(Theme::text())
        })
        .collect();

    let list = List::new(items)
        .block(titled_block("选择应用"))
        .highlight_style(Theme::selected())
        .highlight_symbol("> ");

    let mut list_state = ListState::default();
    list_state.select(Some(app.app_selector_index));

    f.render_widget(Block::default().style(Theme::bg()), chunks[1]);
    f.render_stateful_widget(list, chunks[1], &mut list_state);
}

fn render_proxy_editor(f: &mut ratatui::Frame, area: Rect, app: &AppState) {
    let popup_area = centered_rect(area, 50, 20);

    let input = Paragraph::new(app.proxy_input.as_str())
        .style(Theme::text())
        .block(titled_block("编辑代理地址 [Enter 确认] [Esc 取消]"));

    f.render_widget(Block::default().style(Theme::bg()), popup_area);
    f.render_widget(input, popup_area);

    let cursor_x = popup_area.x + 1 + app.proxy_input.len() as u16;
    let cursor_y = popup_area.y + 2;
    f.set_cursor_position((cursor_x, cursor_y));
}

fn render_env_editor(f: &mut ratatui::Frame, area: Rect, app: &AppState) {
    let popup_area = centered_rect(area, 60, 40);

    let input = Paragraph::new(app.env_input.as_str())
        .style(Theme::text())
        .block(
            Block::default()
                .title(" 编辑全局环境命令 [Enter 确认] [Esc 取消] ")
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded)
                .border_style(Theme::border())
                .style(Theme::bg_dark()),
        );

    f.render_widget(Block::default().style(Theme::bg()), popup_area);
    f.render_widget(input, popup_area);

    let lines: Vec<&str> = app.env_input.lines().collect();
    let cursor_y = popup_area.y + 1 + lines.len().saturating_sub(1) as u16;
    let last_line_len = lines.last().map(|l| l.len()).unwrap_or(0) as u16;
    let cursor_x = popup_area.x + 1 + last_line_len;
    f.set_cursor_position((cursor_x, cursor_y));
}

fn render_global_command_runner(f: &mut ratatui::Frame, area: Rect, app: &AppState) {
    let popup_area = centered_rect(area, 50, 50);

    let items: Vec<ListItem> = app
        .config
        .global_commands
        .iter()
        .map(|cmd| ListItem::new(cmd.as_str()).style(Theme::text()))
        .collect();

    let list = List::new(items)
        .block(
            Block::default()
                .title(" 全局命令 [Enter 执行] [E 编辑] [A 添加] [D 删除] [Esc 返回] ")
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded)
                .border_style(Theme::border())
                .style(Theme::bg_dark()),
        )
        .highlight_style(Theme::selected())
        .highlight_symbol("> ");

    let mut list_state = ListState::default();
    list_state.select(Some(app.global_cmd_index));

    f.render_widget(Block::default().style(Theme::bg()), popup_area);
    f.render_stateful_widget(list, popup_area, &mut list_state);
}

fn render_global_command_editor(f: &mut ratatui::Frame, area: Rect, app: &AppState) {
    let popup_area = centered_rect(area, 50, 20);

    let title = if app.global_cmd_editing_index.is_some() {
        " 编辑全局命令 [Enter 确认] [Esc 取消] "
    } else {
        " 添加全局命令 [Enter 确认] [Esc 取消] "
    };

    let input = Paragraph::new(app.global_cmd_input.as_str())
        .style(Theme::text())
        .block(
            Block::default()
                .title(title)
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded)
                .border_style(Theme::border())
                .style(Theme::bg_dark()),
        );

    f.render_widget(Block::default().style(Theme::bg()), popup_area);
    f.render_widget(input, popup_area);

    let cursor_x = popup_area.x + 1 + app.global_cmd_input.len() as u16;
    let cursor_y = popup_area.y + 2;
    f.set_cursor_position((cursor_x, cursor_y));
}

fn render_pending_quit_notice(f: &mut ratatui::Frame, area: Rect, app: &AppState) {
    let notice_area = centered_rect(area, 60, 15);

    let notice = Paragraph::new(app.pending_quit_message.as_str())
        .style(Theme::pink())
        .alignment(Alignment::Center)
        .block(
            Block::default()
                .title(" ⚠️  退出确认 ")
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded)
                .border_style(Theme::pink())
                .style(Theme::bg_dark()),
        );

    f.render_widget(Block::default().style(Theme::bg()), notice_area);
    f.render_widget(notice, notice_area);
}

fn centered_rect(area: Rect, percent_x: u16, percent_y: u16) -> Rect {
    let popup_layout = ratatui::layout::Layout::default()
        .direction(ratatui::layout::Direction::Vertical)
        .constraints([
            ratatui::layout::Constraint::Percentage((100 - percent_y) / 2),
            ratatui::layout::Constraint::Percentage(percent_y),
            ratatui::layout::Constraint::Percentage((100 - percent_y) / 2),
        ])
        .split(area);

    ratatui::layout::Layout::default()
        .direction(ratatui::layout::Direction::Horizontal)
        .constraints([
            ratatui::layout::Constraint::Percentage((100 - percent_x) / 2),
            ratatui::layout::Constraint::Percentage(percent_x),
            ratatui::layout::Constraint::Percentage((100 - percent_x) / 2),
        ])
        .split(popup_layout[1])[1]
}
