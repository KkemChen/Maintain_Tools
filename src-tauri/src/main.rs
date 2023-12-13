// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod local_sysinfo;
use local_sysinfo::cpu_info::get_cpu_info;
mod ssh;
use ssh::ssh_api;

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            local_sysinfo::cpu_info::get_cpu_info,
            //ssh::ssh_api::init_ssh_connect
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
