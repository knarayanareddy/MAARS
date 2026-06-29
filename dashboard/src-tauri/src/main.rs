mod commands;

use tauri::Manager;

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            commands::run_phase0_command,
            commands::run_project_command,
            commands::inspect_routes_command,
            commands::audit_dashboard_command,
            commands::read_config
        ])
        .setup(|app| {
            let window = app.get_webview_window("main").expect("main window");
            window.show()?;
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
