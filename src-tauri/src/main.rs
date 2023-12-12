// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod sysinfo;
use crate::sysinfo::cpu_info::get_cpu_info;


fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![sysinfo::cpu_info::get_cpu_info])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
