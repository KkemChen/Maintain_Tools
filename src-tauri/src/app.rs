use crate::local_sysinfo::*;
use crate::remote_sysinfo::*;
use crate::ssh::*;

pub fn create_app<R: tauri::Runtime>(builder: tauri::Builder<R>) -> tauri::App<R> {
    builder
        .invoke_handler(tauri::generate_handler![
            crate::ssh::ssh_api::add_ssh_connect,
            crate::ssh::ssh_api::exec_ssh_command,
            crate::ssh::ssh_api::disconnect_ssh,
            crate::local_sysinfo::cpu_info::get_cpu_info,
            crate::remote_sysinfo::process_info::get_process_info
        ])
        .build(tauri::generate_context!())
        .expect("error while running tauri application")
}
