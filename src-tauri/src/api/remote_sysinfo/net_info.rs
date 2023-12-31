use super::Response;
use crate::ssh::ssh_api::*;
use log::*;
use regex::Regex;
use serde::{Deserialize, Serialize};
use tokio::time::{sleep, Duration};
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct NetInfo {
    device: String,
    receive: f64,
    transmit: f64,
}

pub async fn get_net_info_l(host: &str) -> Result<Vec<NetInfo>, String> {
    // 第一次读取
    let initial_output = exec_ssh_command_on_shell(host, "cat /proc/net/dev")?;
    let initial_net_infos = parse_net_data(&initial_output)?;

    // thread::sleep(Duration::from_millis(200));
    sleep(Duration::from_millis(1000)).await;
    // 第二次读取
    let final_output = exec_ssh_command_on_shell(host, "cat /proc/net/dev")?;
    let final_net_infos = parse_net_data(&final_output)?;

    // 计算瞬时流量
    let mut net_infos = Vec::new();
    for (initial, final_) in initial_net_infos.iter().zip(final_net_infos.iter()) {
        let receive_rate = ((final_.receive - initial.receive) as f64) / 3.0 / 1024.0; // 这里的时间间隔是 1 秒
        let transmit_rate = ((final_.transmit - initial.transmit) as f64) / 3.0 / 1024.0; // 同上

        net_infos.push(NetInfo {
            device: final_.device.clone(),
            receive: receive_rate,
            transmit: transmit_rate,
        });
    }

    Ok(net_infos)
}

fn parse_net_data(output: &str) -> Result<Vec<NetInfo>, String> {
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
                receive: receive as f64,
                transmit: transmit as f64,
            });
        }
    }
    Ok(net_infos)
}

#[tauri::command]
pub async fn get_net_info(host: &str) -> Result<String, String> {
    match get_net_info_l(host).await {
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
    use log4rs;
    use std::env;
    #[tokio::test]
    async fn test_net_info() {
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
        let ret = get_net_info(format!("{}:{}", host, port).as_str())
            .await
            .unwrap();
        info!("{}", ret);
        disconnect_ssh(format!("{}:{}", host, port).as_str());
    }
}
