use super::ssh_manager::*;
use lazy_static::lazy_static;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

lazy_static! {
    static ref SSHMAP: Mutex<HashMap<String, SshConnectionManager>> = Mutex::new(HashMap::new());
}

#[tauri::command]
pub fn add_ssh_connect(host: &str, user: &str, password: &str) {
    let mut manager = SshConnectionManager::new();
    let _ = manager.connect(host, user, password);
    let _ = manager.send_file(
        "C:\\Users\\Administrator\\Desktop\\sysinfo-http",
        "/tmp/sysinfo-http",
    );
    manager.exec_command("chmod +x /tmp/sysinfo-http").unwrap();
    manager
        .exec_command("nohup /tmp/sysinfo-http > /dev/null 2>&1 &")
        .unwrap();
    //todo:添加到map
    let mut map = SSHMAP.lock().unwrap();
    map.insert(host.to_string(), manager);
    println!("Add ssh connect success.");
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
        Err("Manager for specified host not found".to_string())
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
        Err("Manager for specified host not found".to_string())
    }
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
}
