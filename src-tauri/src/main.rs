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
            api.prevent_exit();
        }
        _ => {}
    });

    /*  tauri::Builder::default()
    .invoke_handler(tauri::generate_handler![
        crate::ssh::ssh_api::add_ssh_connect,
        crate::ssh::ssh_api::exec_ssh_command,
        crate::ssh::ssh_api::disconnect_ssh,
        crate::api::local_sysinfo::cpu_info::get_cpu_info_local,
        crate::api::remote_sysinfo::process_info::get_process_info,
        crate::api::remote_sysinfo::disk_detail::get_disk_detail,
        crate::api::remote_sysinfo::disk_info::get_disk_info,
        crate::api::remote_sysinfo::net_info::get_net_info,
        crate::api::remote_sysinfo::cpu_info::get_cpu_info,
        crate::api::remote_sysinfo::cpu_detail::get_cpu_detail,
        crate::api::remote_sysinfo::mem_info::get_mem_info,
        crate::api::remote_sysinfo::load_info::get_load_info,
        crate::api::tools_interface::check_stream::check_stream,
    ])
    .run(tauri::generate_context!())
    .expect("error while running tauri application"); */
}
