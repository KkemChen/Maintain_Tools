use super::Response;
use crate::ssh::ssh_api::*;
use log::*;
use regex::Regex;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct LoadInfo {
    load1: f32,
    load5: f32,
    load15: f32,
}

fn get_load_info_l(host: &str) -> Result<LoadInfo, String> {
    let output = exec_ssh_command(host, "uptime")?;

    let load_re = Regex::new(r"load average: (\d+\.\d+), (\d+\.\d+), (\d+\.\d+)").unwrap();
    let load_caps = load_re.captures(&output).ok_or("No load data found")?;
    let load1 = load_caps[1].parse::<f32>().map_err(|e| e.to_string())?;
    let load5 = load_caps[2].parse::<f32>().map_err(|e| e.to_string())?;
    let load15 = load_caps[3].parse::<f32>().map_err(|e| e.to_string())?;
    let load_info = LoadInfo {
        load1,
        load5,
        load15,
    };

    Ok(load_info)
}

#[tauri::command]
pub fn get_load_info(host: &str) -> Result<String, String> {
    match get_load_info_l(host) {
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
            error!("get_load_info failed, err: {}", err);
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

        let ret = get_load_info(format!("{}:{}", host, port).as_str()).unwrap();
        info!("{}", ret);
        disconnect_ssh(format!("{}:{}", host, port).as_str());
    }
}
