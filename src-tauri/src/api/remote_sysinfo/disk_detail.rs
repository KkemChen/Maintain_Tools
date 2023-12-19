use super::Response;
use crate::ssh::ssh_api::*;
use log::*;
use regex::Regex;
use serde::{Deserialize, Serialize};
#[derive(Serialize, Deserialize, Debug)]
pub struct DiskDetail {
    name: String,
    size: String,
    used: String,
    avail: String,
    use_percentage: String,
    mounted_on: String,
}

fn get_disk_detail_l(host: &str) -> Result<Vec<DiskDetail>, String> {
    let output = exec_ssh_command(host, "df -h")?;
    let re = Regex::new(
        r"(?m)^(?P<name>\S+)\s+(?P<size>\S+)\s+(?P<used>\S+)\s+(?P<avail>\S+)\s+(?P<use_percentage>\S+%)\s+(?P<mounted_on>\S+)$"
    ).map_err(|e| e.to_string())?;

    let mut disk_infos = Vec::new();

    for cap in re.captures_iter(&output) {
        disk_infos.push(DiskDetail {
            name: cap["name"].to_string(),
            size: cap["size"].to_string(),
            used: cap["used"].to_string(),
            avail: cap["avail"].to_string(),
            use_percentage: cap["use_percentage"].to_string(),
            mounted_on: cap["mounted_on"].to_string(),
        });
    }
    Ok(disk_infos)
}

#[tauri::command]
pub fn get_disk_detail(host: &str) -> Result<String, String> {
    match get_disk_detail_l(host) {
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
            error!("get_disk_detail failed, err: {}", err);
            serde_json::to_string(&response).map_err(|e| e.to_string())
            /*  error!("get_disk_info failed, err: {}", err);
            Err(err) // 直接返回错误 */
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

        match get_disk_detail(format!("{}:{}", host, port).as_str()) {
            Ok(res) => {
                info!("{}", res);
            }
            Err(err) => {
                error!("{}", err);
            }
        };

        disconnect_ssh(format!("{}:{}", host, port).as_str());
    }
}
