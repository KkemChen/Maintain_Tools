use log::*;
pub fn create_app<R: tauri::Runtime>(builder: tauri::Builder<R>) -> tauri::App<R> {
    builder
        .on_window_event(|event| {
            match event.event() {
                tauri::WindowEvent::CloseRequested { api, .. } => {
                    //阻止默认关闭
                    api.prevent_close();

                    let window = event.window().clone();
                    tauri::api::dialog::confirm(
                        Some(&event.window()),
                        "关闭应用",
                        "确定关闭当前应用?",
                        move |answer| {
                            if answer {
                                cleanup_on_exit();
                                window.close();
                            }
                        },
                    )
                }
                _ => {} //todo
            }
        })
        .invoke_handler(tauri::generate_handler![
            crate::ssh::ssh_api::add_ssh_connect,
            crate::ssh::ssh_api::exec_ssh_command,
            crate::ssh::ssh_api::disconnect_ssh,
            crate::api::local_sysinfo::cpu_info::get_cpu_info_local,
            crate::api::remote_sysinfo::get_sysinfo,
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
