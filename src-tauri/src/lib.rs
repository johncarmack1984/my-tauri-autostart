// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .setup(|app| {
            #[cfg(desktop)]
            use tauri_plugin_autostart::MacosLauncher;
            use tauri_plugin_autostart::ManagerExt;

            match app.handle().plugin(tauri_plugin_autostart::init(
                MacosLauncher::LaunchAgent,
                Some(vec!["--flag1", "--flag2"]),
            )) {
                Ok(()) => (),
                Err(e) => println!("Error initializing autostart plugin: {}", e),
            };

            let autostart_manager = app.autolaunch();

            match autostart_manager.enable() {
                Ok(()) => (),
                Err(e) => println!("Error enabling autostart: {}", e),
            };

            match autostart_manager.is_enabled() {
                Ok(enabled) => println!("Autostart is enabled: {}", enabled),
                Err(e) => println!("Error checking autostart status: {}", e),
            };

            Ok(())
        })
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![greet])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
