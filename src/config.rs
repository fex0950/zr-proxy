use directories::ProjectDirs;
use serde::{Deserialize, Serialize};
use std::fs::{create_dir_all, read_to_string, write};
use std::path::PathBuf;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct App {
    pub name: String,
    pub path: PathBuf,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    pub proxy_url: String,
    #[serde(default)]
    pub env_commands: Vec<String>,
    #[serde(default)]
    pub global_commands: Vec<String>,
    pub apps: Vec<App>,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            proxy_url: "http://127.0.0.1:7890".to_string(),
            env_commands: vec![
                "HTTP_PROXY=http://127.0.0.1:7890".to_string(),
                "HTTPS_PROXY=http://127.0.0.1:7890".to_string(),
                "ALL_PROXY=http://127.0.0.1:7890".to_string(),
            ],
            global_commands: Vec::new(),
            apps: Vec::new(),
        }
    }
}

impl Config {
    pub fn load() -> Self {
        if let Some(proj_dirs) = ProjectDirs::from("", "", "zr-proxy") {
            let config_path = proj_dirs.config_dir().join("config.toml");
            if config_path.exists() {
                if let Ok(content) = read_to_string(&config_path) {
                    if let Ok(config) = toml::from_str(&content) {
                        return config;
                    }
                }
            }
        }
        Self::default()
    }

    pub fn save(&self) -> Result<(), Box<dyn std::error::Error>> {
        if let Some(proj_dirs) = ProjectDirs::from("", "", "zr-proxy") {
            let config_dir = proj_dirs.config_dir();
            create_dir_all(config_dir)?;
            let config_path = config_dir.join("config.toml");
            let content = toml::to_string_pretty(self)?;
            write(config_path, content)?;
        }
        Ok(())
    }
}
