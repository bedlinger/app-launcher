// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use tauri::{Manager, SystemTray, SystemTrayEvent};
use tauri::{CustomMenuItem, SystemTrayMenu};
use tauri_plugin_autostart::MacosLauncher;

fn main() {
    let quit = CustomMenuItem::new("quit".to_string(), "Quit");
    let tray_menu = SystemTrayMenu::new()
        .add_item(quit);
    let tray = SystemTray::new().with_menu(tray_menu).with_tooltip("App-Launcher");

    tauri::Builder::default()
        .system_tray(tray)
        .on_system_tray_event(|app, event| match event {
            SystemTrayEvent::LeftClick {
                position: _,
                size: _,
                ..
            } => {
                let window = app.get_window("main").unwrap();
                if window.is_visible().unwrap() {
                    window.hide().unwrap();
                } else {
                    window.show().unwrap();
                }
            }
            SystemTrayEvent::MenuItemClick { id, .. } => match id.as_str() {
                "quit" => {
                    std::process::exit(0);
                }
                _ => {}
            },
            _ => {}
        })
        .invoke_handler(tauri::generate_handler![
            get_installed_apps_win,
            launch_app_win,
            open_location_win,
            get_installed_app_linux,
            launch_app_linux
        ])
        .plugin(tauri_plugin_autostart::init(MacosLauncher::LaunchAgent, Some(vec!["--flag1", "--flag2"])))
        .run(tauri::generate_context!())
        .unwrap_or_else(|e| eprintln!("Error while running tauri application: {}", e));
}

#[derive(serde::Deserialize, serde::Serialize)]
struct App {
    name: String,
    path: String,
}

#[tauri::command]
fn get_installed_apps_win() -> Vec<App> {
    let mut apps = vec![];
    let username = std::env::var("USERNAME").unwrap_or_else(|_| "default".to_string());
    let formatted_path = format!("C:\\Users\\{}\\AppData\\Roaming\\Microsoft\\Windows\\Start Menu\\Programs", username);
    let paths = vec![
        "C:\\ProgramData\\Microsoft\\Windows\\Start Menu",
        &formatted_path,
    ];
    for path in paths {
        get_apps_from_dir_win(path, &mut apps);
    }
    apps.sort_by(|a, b| a.name.to_lowercase().cmp(&b.name.to_lowercase()));
    apps
}

fn get_apps_from_dir_win(path: &str, apps: &mut Vec<App>) {
    if let Ok(entries) = std::fs::read_dir(path) {
        for entry in entries {
            if let Ok(entry) = entry {
                if let Ok(metadata) = entry.metadata() {
                    if metadata.is_dir() {
                        get_apps_from_dir_win(&entry.path().to_string_lossy().to_string(), apps);
                    } else if entry.path().extension().and_then(std::ffi::OsStr::to_str)
                        == Some("lnk") || entry.path().extension().and_then(std::ffi::OsStr::to_str) == Some("url")
                    {
                        if let Ok(name) = entry.file_name().into_string() {
                            let name_without_extension = std::path::Path::new(&name)
                                .file_stem()
                                .and_then(std::ffi::OsStr::to_str)
                                .unwrap_or(&name);
                            apps.push(App {
                                name: name_without_extension.to_string(),
                                path: entry.path().to_string_lossy().to_string(),
                            });
                        }
                    }
                }
            }
        }
    }
}

#[tauri::command]
fn launch_app_win(app: App) {
    if let Ok(_) = std::process::Command::new("explorer").arg(app.path).spawn() {
        return;
    }
}

#[tauri::command]
fn open_location_win(app: App) {
    if let Ok(_) = std::process::Command::new("explorer")
        .arg("/select,")
        .arg(app.path)
        .spawn()
    {
        return;
    }
}

#[tauri::command]
fn get_installed_app_linux(executable_dir_path: String) -> Vec<App> {
    let paths = std::fs::read_dir(executable_dir_path).unwrap();
    let mut apps = vec![];
    for path in paths {
        let path = path.unwrap().path();
        let path = path.to_str().unwrap();
        let app = App {
            name: path.to_string(),
            path: path.to_string()
        };
        apps.push(app);
    }
    apps
}

#[tauri::command]
fn launch_app_linux(app: App) {
    let _ = std::process::Command::new("xdg-open").arg(app.path).spawn();
}
