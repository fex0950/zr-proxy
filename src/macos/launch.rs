use crate::config::App;
use arboard::Clipboard;
use std::process::Command;

pub fn launch_with_proxy(
    app: &App,
    proxy_url: &str,
    env_commands: &[String],
) -> Result<(), Box<dyn std::error::Error>> {
    let app_name = app
        .path
        .file_name()
        .and_then(|s| s.to_str())
        .unwrap_or("App");
    let mut cmd = Command::new("env");
    for ec in env_commands {
        cmd.arg(ec);
    }
    cmd.arg("open")
        .arg("-na")
        .arg(app_name)
        .arg("--args")
        .arg(format!("--proxy-server={}", proxy_url))
        .spawn()?;
    Ok(())
}

pub fn get_launch_command(app: &App, proxy_url: &str, env_commands: &[String]) -> String {
    let app_name = app
        .path
        .file_name()
        .and_then(|s| s.to_str())
        .unwrap_or("App");
    let env_part = if env_commands.is_empty() {
        String::new()
    } else {
        format!("{} ", env_commands.join(" "))
    };
    format!(
        "{}open -na \"{}\" --args --proxy-server=\"{}\"",
        env_part, app_name, proxy_url
    )
}

pub fn copy_to_clipboard(text: &str) -> Result<(), Box<dyn std::error::Error>> {
    let mut clipboard = Clipboard::new()?;
    clipboard.set_text(text)?;
    Ok(())
}
