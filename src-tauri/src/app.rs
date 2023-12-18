pub fn create_app<R: tauri::Runtime>(builder: tauri::Builder<R>) -> tauri::App<R> {
    builder
        .invoke_handler(tauri::generate_handler![
            crate::ssh::ssh_api::add_ssh_connect,
            crate::ssh::ssh_api::exec_ssh_command,
            crate::ssh::ssh_api::disconnect_ssh,
            crate::api::local_sysinfo::cpu_info::get_cpu_info,
            crate::api::remote_sysinfo::process_info::get_process_info,
            crate::api::remote_sysinfo::disk_info::get_disk_info,
            crate::api::remote_sysinfo::net_info::get_net_info,
            crate::api::remote_sysinfo::total_info::get_total_info,
            crate::api::remote_sysinfo::cpu_detail::get_cpu_detail
        ])
        .build(tauri::generate_context!())
        .expect("error while running tauri application")
}

pub fn cleanup_on_exit() {
    match crate::ssh::ssh_api::disconnect_all() {
        Ok(result) => println!("{}", result),
        Err(e) => println!("Error: {}", e),
    }
}
