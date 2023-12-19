use super::Response;
use crate::ssh::ssh_api::*;
use log::error;
use regex::Regex;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct MemInfo {
    used: u32,
    free: u32,
    total: u32,
}

fn get_mem_info_l(host: &str) -> Result<MemInfo, String> {
    // 内存信息
    let output = exec_ssh_command(host, "free -m")?;

    let mem_re = Regex::new(r"\bMem:\s+(\d+)\s+(\d+)\s+(\d+)").unwrap();
    let mem_caps = mem_re.captures(&output).ok_or("No memory data found")?;
    let total = mem_caps[1].parse::<u32>().map_err(|e| e.to_string())?;
    let used = mem_caps[2].parse::<u32>().map_err(|e| e.to_string())?;
    let free = mem_caps[3].parse::<u32>().map_err(|e| e.to_string())?;
    // let available = mem_caps[4].parse::<u32>().map_err(|e| e.to_string())?;
    let mem_info = MemInfo { used, free, total };
    Ok(mem_info)
}

#[tauri::command]
pub fn get_mem_info(host: &str) -> Result<String, String> {
    match get_mem_info_l(host) {
        Ok(data) => {
            let response = Response {
                code: 0,
                message: "success".to_string(),
                data: Some(data),
            };
            serde_json::to_string(&response).map_err(|e| e.to_string())
        }
        Err(err) => {
            let response = Response::<String> {
                code: -1,
                message: err.clone(),
                data: None,
            };
            error!("get_mem_info failed, err: {}", err);
            serde_json::to_string(&response).map_err(|e| e.to_string())
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::ssh::ssh_api::*;
    use dotenv::dotenv;
    use std::env;
    use std::{thread, time::Duration};

    #[test]
    fn test_get_process_info() {
        dotenv::from_path("../.env").ok();
        log4rs::init_file("log4rs.yaml", Default::default()).unwrap();

        let host = env::var("VITE_HOST").unwrap();
        let port = env::var("VITE_PORT").unwrap();
        let user = env::var("VITE_USER").unwrap();
        let passwd = env::var("VITE_PASSWORD").unwrap();

        add_ssh_connect(
            format!("{}:{}", host, port).as_str(),
            user.as_str(),
            passwd.as_str(),
        );

        let ret = get_mem_info(format!("{}:{}", host, port).as_str()).unwrap();
        info!("{}", ret);
        disconnect_ssh(format!("{}:{}", host, port).as_str());
    }
}
