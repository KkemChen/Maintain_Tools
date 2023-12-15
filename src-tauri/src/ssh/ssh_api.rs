use super::ssh_manager::*;
use lazy_static::lazy_static;
use serde::Deserialize;
use std::collections::HashMap;
use std::fs;
use std::sync::{Arc, Mutex};
use toml;

lazy_static! {
    static ref SSHMAP: Mutex<HashMap<String, SshConnectionManager>> = Mutex::new(HashMap::new());
}

#[derive(Deserialize)]
struct Config {
    local_path: String,
    remote_path: String,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            local_path: "plugins/sysinfo-http".to_string(),
            remote_path: "/tmp/sysinfo-http".to_string(),
        }
    }
}

fn read_config() -> Config {
    let config_str = match fs::read_to_string("config.toml") {
        Ok(contents) => contents,
        Err(_) => return Config::default(),
    };

    match toml::from_str(&config_str) {
        Ok(config) => config,
        Err(_) => Config::default(),
    }
}

#[tauri::command]
pub fn add_ssh_connect(host: &str, user: &str, password: &str) -> Result<(), String> {
    let config = read_config();

    let mut manager = SshConnectionManager::new();
    manager
        .connect(host, user, password)
        .map_err(|e| format!("Failed to connect: {}", e))?;
    manager
        .exec_command("pkill -9 sysinfo-http")
        .map_err(|e| e.to_string())?;

    manager
        .send_file(&config.local_path, &config.remote_path)
        .map_err(|e| format!("Failed to send file: {}", e))?;

    manager
        .exec_command("chmod +x /tmp/sysinfo-http")
        .map_err(|e| e.to_string())?;

    manager
        .exec_command("nohup /tmp/sysinfo-http > /dev/null 2>&1 &")
        .map_err(|e| format!("Failed to execute sysinfo-http command: {}", e))?;

    let mut map = SSHMAP.lock().unwrap();
    map.insert(host.to_string(), manager);
    println!("Add ssh connect success. {}", host);
    Ok(())
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
pub fn disconnect_ssh(host: &str) -> Result<String, String> {
    let map = SSHMAP.lock().unwrap();
    if let Some(manager) = map.get(host) {
        match manager.exec_command("pkill -9 sysinfo-http") {
            Ok(output) => Ok(output),
            Err(err) => Err(format!("Failed to execute command: {:?}", err)),
        }
    } else {
        Err(format!("Manager for specified host not found, {}", host))
    }
}

pub fn disconnect_all() -> Result<String, String> {
    let map = SSHMAP.lock().unwrap();
    let mut result = String::new();

    for (_host, manager) in map.iter() {
        match manager.exec_command("pkill -9 sysinfo-http") {
            Ok(output) => result.push_str(&format!("{}: Disconnect success\n", _host)),
            Err(err) => result.push_str(&format!(
                "{}: Failed to execute command: {:?}\n",
                _host, err
            )),
        }
    }
    Ok(result)
}

#[cfg(test)]
mod test {
    use std::{thread, time::Duration};

    use super::*;

    #[test]
    fn test_add_ssh_connect() {
        add_ssh_connect("192.168.1.172:6622", "root", "ivauto@123");
    }

    #[test]
    fn test_disconnect() {
        add_ssh_connect("192.168.1.172:6622", "root", "ivauto@123");
        thread::sleep(Duration::from_secs(10));
        disconnect_ssh("192.168.1.172:6622");
    }

    #[test]
    fn test_exec_command() {
        add_ssh_connect("192.168.1.172:5523", "root", "ivauto@123");
        let out = exec_ssh_command_on_shell(
            "192.168.1.172:5523",
            "timeout 10 ffprobe -v quiet -print_format json -show_format -show_streams rtsp://192.168.1.82/live/66",
        )
        .unwrap();

        println!("xxx: {}", out);
        disconnect_ssh("192.168.1.172:5523");
    }
}
