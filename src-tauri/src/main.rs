// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod app;
pub mod local_sysinfo;
pub mod remote_sysinfo;
pub mod ssh;

fn main() {
    app::create_app(tauri::Builder::default()).run(|_app_handle, event| match event {
        tauri::RunEvent::ExitRequested { api, .. } => {
            api.prevent_exit();
        }
        _ => {}
    });
}
