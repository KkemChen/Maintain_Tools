use super::Response;
use crate::ssh::ssh_api::*;
use regex::Regex;
use serde::{Deserialize, Serialize};
use serde_json::json;

#[derive(Serialize, Deserialize, Debug)]
struct NetInfo {
    device: String,
    receive: u64,
    transmit: u64,
}

fn get_net_info_l(host: &str) -> Result<String, String> {
    let output = exec_ssh_command_on_shell(host, "cat /proc/net/dev")?;
    // println!("{}", output);
    let re =
        Regex::new(r"(\w+):\s+(\d+)\s+\d+\s+\d+\s+\d+\s+\d+\s+\d+\s+\d+\s+\d+\s+(\d+)").unwrap();

    let mut net_infos = Vec::new();
    for line in output.lines() {
        if let Some(caps) = re.captures(line) {
            let device = caps.get(1).unwrap().as_str().to_string();
            let receive = caps
                .get(2)
                .unwrap()
                .as_str()
                .parse::<u64>()
                .map_err(|e| e.to_string())?;
            let transmit = caps
                .get(3)
                .unwrap()
                .as_str()
                .parse::<u64>()
                .map_err(|e| e.to_string())?;
            net_infos.push(NetInfo {
                device,
                receive,
                transmit,
            });
        }
    }

    let json = serde_json::to_string(&net_infos).map_err(|e| e.to_string())?;
    // println!("JSON output: {}", json);
    Ok(json)
}

#[tauri::command]
pub fn get_net_info(host: &str) -> Result<String, String> {
    match get_net_info_l(host) {
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
                message: err,
                data: None,
            };
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

    #[test]
    fn test_net_info() {
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
        get_net_info(format!("{}:{}", host, port).as_str());
        disconnect_ssh(format!("{}:{}", host, port).as_str());
    }
}
