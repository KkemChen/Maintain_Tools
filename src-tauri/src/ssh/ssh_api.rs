use super::ssh_manager::*;
use crate::api::{start_fetch_sysinfo, stop_fetch_sysinfo};
use lazy_static::lazy_static;
use log::*;
use serde_json::json;
use std::collections::HashMap;
use std::sync::Mutex;

lazy_static! {
    static ref SSHMAP: Mutex<HashMap<String, SshConnectionManager>> = Mutex::new(HashMap::new());
}

#[tauri::command]
pub fn add_ssh_connect(host: &str, user: &str, password: &str) -> String {
    let mut manager = SshConnectionManager::new();

    let result = manager.connect(host, user, password);
    // .and_then(|_| manager.exec_command("pkill -9 sysinfo-http"))
    // .and_then(|_| manager.send_file(&config.local_path, &config.remote_path))
    // .and_then(|_| manager.exec_command("chmod +x /tmp/sysinfo-http"))
    // .and_then(|_| manager.exec_command("nohup /tmp/sysinfo-http > /dev/null 2>&1 &"));

    match result {
        Ok(_) => {
            let mut map = SSHMAP.lock().unwrap();
            map.insert(host.to_string(), manager);
            info!("Add ssh connect success. {}", host);
            start_fetch_sysinfo(host);
            json!({
                "code": 0,
                "message": "Add ssh connect success.".to_string(),
            })
            .to_string()
        }
        Err(e) => json!({
            "code": -1,
            "message": format!("Failed: {}", e),
        })
        .to_string(),
    }
}

#[tauri::command]
pub fn exec_ssh_command(host: &str, command: &str) -> Result<String, String> {
    let map = SSHMAP.lock().unwrap();
    if let Some(manager) = map.get(host) {
        match manager.exec_command(command) {
            Ok(output) => Ok(output),
            Err(err) => Err(format!("Failed to execute command: {:?}", err)),
        }
    } else {
        Err(format!("Manager for specified host not found, {}", host))
    }
}

#[tauri::command]
pub fn exec_ssh_command_on_shell(host: &str, command: &str) -> Result<String, String> {
    let map = SSHMAP.lock().unwrap();
    if let Some(manager) = map.get(host) {
        match manager.exec_command_on_shell(command) {
            Ok(output) => Ok(output),
            Err(err) => Err(format!("Failed to execute command: {:?}", err)),
        }
    } else {
        Err(format!("Manager for specified host not found, {}", host))
    }
}

#[tauri::command]
pub fn disconnect_ssh(host: &str) -> String {
    stop_fetch_sysinfo(host);
    let mut map = SSHMAP.lock().unwrap();

    if map.contains_key(host) {
        map.remove(host);

        let res = json!({
            "code": 0,
            "message": format!("Disconnected SSH for host {}", host),
        })
        .to_string();
        res
    } else {
        let res = json!({
            "code": -1,
            "message": format!("Manager for specified host not found, {}", host),
        })
        .to_string();
        res
    }
}

pub fn disconnect_all() -> Result<String, String> {
    let mut map = SSHMAP.lock().unwrap();
    map.clear(); // 清空 HashMap
    Ok("Disconnectd all SSH".to_string())
}

#[cfg(test)]
mod test {
    use super::*;
    use dotenv::dotenv;
    use std::env;
    use std::{thread, time::Duration};

    #[test]
    fn test_add_ssh_connect() {
        dotenv::from_path("../.env").ok();

        let host = env::var("VITE_HOST").unwrap();
        let port = env::var("VITE_PORT").unwrap();
        let user = env::var("VITE_USER").unwrap();
        let passwd = env::var("VITE_PASSWORD").unwrap();

        add_ssh_connect(
            format!("{}:{}", host, port).as_str(),
            user.as_str(),
            passwd.as_str(),
        );
    }

    #[tokio::test]
    async fn test_disconnect() {
        dotenv::from_path("../.env").ok();

        let host = env::var("VITE_HOST").unwrap();
        let port = env::var("VITE_PORT").unwrap();
        let user = env::var("VITE_USER").unwrap();
        let passwd = env::var("VITE_PASSWORD").unwrap();

        add_ssh_connect(
            format!("{}:{}", host, port).as_str(),
            user.as_str(),
            passwd.as_str(),
        );
        thread::sleep(Duration::from_secs(5));
        let ret = disconnect_ssh(format!("{}:{}", host, port).as_str());
        println!("{}", ret);
    }

    #[test]
    fn test_exec_command() {
        dotenv::from_path("../.env").ok();

        let host = env::var("VITE_HOST").unwrap();
        let port = env::var("VITE_PORT").unwrap();
        let user = env::var("VITE_USER").unwrap();
        let passwd = env::var("VITE_PASSWORD").unwrap();

        add_ssh_connect(
            format!("{}:{}", host, port).as_str(),
            user.as_str(),
            passwd.as_str(),
        );
        let out = exec_ssh_command_on_shell(
            format!("{}:{}", host, port).as_str(),
            // "timeout 10 ffprobe -v quiet -print_format json -show_format -show_streams rtsp://192.168.1.82/live/66",
            "timeout 10 top -b -n 1",
        )
        .unwrap();

        println!("xxx: {}", out);

        disconnect_ssh(format!("{}:{}", host, port).as_str());
    }
}
