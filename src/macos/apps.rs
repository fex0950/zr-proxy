use crate::config::App;
use std::fs;
use std::path::Path;

pub fn scan_applications() -> Vec<App> {
    let mut apps = Vec::new();

    if let Ok(home) = std::env::var("HOME") {
        let user_apps = Path::new(&home).join("Applications");
        apps.extend(scan_directory(&user_apps));
    }

    let system_apps = Path::new("/Applications");
    apps.extend(scan_directory(system_apps));

    apps.sort_by(|a, b| a.name.cmp(&b.name));
    apps.dedup_by(|a, b| a.path == b.path);
    apps
}

fn scan_directory(dir: &Path) -> Vec<App> {
    let mut apps = Vec::new();

    if let Ok(entries) = fs::read_dir(dir) {
        for entry in entries.flatten() {
            let path = entry.path();
            if path.extension().and_then(|s| s.to_str()) == Some("app") {
                if let Some(name) = path.file_stem().and_then(|s| s.to_str()) {
                    apps.push(App {
                        name: name.to_string(),
                        path: path.clone(),
                    });
                }
            }
        }
    }

    apps
}
