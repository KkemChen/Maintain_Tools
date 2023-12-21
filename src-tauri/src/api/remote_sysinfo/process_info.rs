use super::Response;
use crate::ssh::ssh_api::*;
use log::*;
use regex::Regex;
use serde::{Deserialize, Serialize};
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct ProcessInfo {
    pid: String,
    user: String,
    virt: String,
    res: String,
    cpu: String,
    mem: String,
    command: String,
}

pub fn get_process_info_l(host: &str) -> Result<Vec<ProcessInfo>, String> {
    let output = exec_ssh_command_on_shell(host, "top -b -n 1")?;
    //println!("{}", output);
    let re = Regex::new(
        r"(?m)^\s*(\d+)\s+(\S+)\s+\d+\s+\d+\s+(\S+)\s+(\S+)\s+\S+\s+\S+\s+(\S+)\s+(\S+)\s+\S+\s+(.+)$",
    ).map_err(|e| e.to_string())?;

    let mut process_infos = Vec::new();

    for cap in re.captures_iter(&output) {
        process_infos.push(ProcessInfo {
            pid: cap[1].to_string(),
            user: cap[2].to_string(),
            virt: cap[3].to_string(),
            res: cap[4].to_string(),
            cpu: cap[5].to_string(),
            mem: cap[6].to_string(),
            command: cap[7].to_string(),
        });
    }

    Ok(process_infos)
}

#[tauri::command]
pub fn get_process_info(host: &str) -> Result<String, String> {
    match get_process_info_l(host) {
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
            error!("get_process_info failed, err: {}", err);
            serde_json::to_string(&response).map_err(|e| e.to_string())
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use log4rs;
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

        let ret = get_process_info(format!("{}:{}", host, port).as_str()).unwrap();
        info!("{}", ret);
        disconnect_ssh(format!("{}:{}", host, port).as_str());
    }
}
