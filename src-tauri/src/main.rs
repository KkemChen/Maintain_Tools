// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod api;
mod app;
mod ssh;
use log::{self, info};
use log4rs;
fn main() {
    log4rs::init_file("log4rs.yaml", Default::default()).unwrap();
    info!("Start...");
    app::create_app(tauri::Builder::default()).run(|_app_handle, event| match event {
        tauri::RunEvent::ExitRequested { api, .. } => {
            app::cleanup_on_exit();
            // api.prevent_exit();
        }
        _ => {}
    });
}
