use log::*;
pub fn create_app<R: tauri::Runtime>(builder: tauri::Builder<R>) -> tauri::App<R> {
    builder
        .invoke_handler(tauri::generate_handler![
            crate::ssh::ssh_api::add_ssh_connect,
            crate::ssh::ssh_api::exec_ssh_command,
            crate::ssh::ssh_api::disconnect_ssh,
            crate::api::local_sysinfo::cpu_info::get_cpu_info_local,
            crate::api::get_sysinfo,
            crate::api::tools_interface::check_stream::check_stream,
        ])
        .build(tauri::generate_context!())
        .expect("error while running tauri application")
}

pub fn cleanup_on_exit() {
    match crate::ssh::ssh_api::disconnect_all() {
        Ok(result) => info!("{}", result),
        Err(e) => error!("Error: {}", e),
    }
}
