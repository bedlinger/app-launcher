// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![get_installed_apps, launch_app])
        .run(tauri::generate_context!())
        .unwrap_or_else(|e| eprintln!("Error while running tauri application: {}", e));
}

#[derive(serde::Deserialize, serde::Serialize)]
struct App {
    name: String,
    path: String
}

#[tauri::command]
fn get_installed_apps() -> Vec<App> {
    let mut apps = vec![];
    let paths = vec![
        "C:\\ProgramData\\Microsoft\\Windows\\Start Menu"
    ];
    for path in paths {
        get_apps_from_dir(&path, &mut apps);
    }
    apps.sort_by(|a, b| a.name.to_lowercase().cmp(&b.name.to_lowercase()));
    apps
}

fn get_apps_from_dir(path: &str, apps: &mut Vec<App>) {
    if let Ok(entries) = std::fs::read_dir(path) {
        for entry in entries {
            if let Ok(entry) = entry {
                if let Ok(metadata) = entry.metadata() {
                    if metadata.is_dir() {
                        get_apps_from_dir(&entry.path().to_string_lossy().to_string(), apps);
                    } else if entry.path().extension().and_then(std::ffi::OsStr::to_str) == Some("lnk") {
                        if let Ok(name) = entry.file_name().into_string() {
                            let name_without_extension = std::path::Path::new(&name)
                                .file_stem()
                                .and_then(std::ffi::OsStr::to_str)
                                .unwrap_or(&name);
                            apps.push(App {
                                name: name_without_extension.to_string(),
                                path: entry.path().to_string_lossy().to_string()
                            });
                        }
                    }
                }
            }
        }
    }
}

#[tauri::command]
fn launch_app(app: App) {
    println!("Launching app: {}", app.path);
    // Verknüpfung öffnen
    if let Ok(_) = std::process::Command::new("explorer")
        .arg(app.path)
        .spawn() {
        return;
    }
}