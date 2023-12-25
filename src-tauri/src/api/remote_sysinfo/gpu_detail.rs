use super::Response;
use crate::ssh::ssh_api::*;
use log::{error, info};
use regex::Regex;
use serde::{Deserialize, Serialize};
use std::str::FromStr;
use tokio::time::{sleep, Duration};
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct GpuDetail {
    name: String,
    index: u32,
    usage: f32, //核心使用率
    total_memory: f32,
    used_memory: f32,
    power: f32,
    power_limit: f32,
}

pub async fn get_gpu_detail_l(host: &str) -> Result<Vec<GpuDetail>, String> {
    let output = exec_ssh_command_on_shell(host, "nvidia-smi --format=csv --query-gpu=name,index,utilization.gpu,memory.total,memory.used,power.draw,power.limit")
        .map_err(|e| e.to_string())?;

    // 更新后的正则表达式，用于匹配数据行，并捕获GPU名称
    let data_re = Regex::new(
        r"(?P<name>NVIDIA GeForce .+?),\s*(?P<index>\d+),\s*(?P<usage>\d+) %,\s*(?P<total>\d+) MiB,\s*(?P<used>\d+) MiB,\s*(?P<power>[\d.]+) W,\s*(?P<power_limit>[\d.]+) W",
    )
    .map_err(|_| "Invalid regex".to_string())?;

    let mut gpu_details = Vec::new();

    for line in output.lines().skip(1) {
        if let Some(caps) = data_re.captures(line) {
            // 从捕获组中动态提取GPU名称
            let name = caps["name"].to_string();
            let index =
                u32::from_str(&caps["index"]).map_err(|e| format!("Invalid index: {}", e))?;
            let usage =
                f32::from_str(&caps["usage"]).map_err(|e| format!("Invalid usage: {}", e))?;
            let total_memory = f32::from_str(&caps["total"])
                .map_err(|e| format!("Invalid total memory: {}", e))?;
            let used_memory =
                f32::from_str(&caps["used"]).map_err(|e| format!("Invalid used memory: {}", e))?;
            let power =
                f32::from_str(&caps["power"]).map_err(|e| format!("Invalid power: {}", e))?;
            let power_limit = f32::from_str(&caps["power_limit"])
                .map_err(|e| format!("Invalid power limit: {}", e))?;

            let gpu_detail = GpuDetail {
                name,
                index,
                usage,
                total_memory,
                used_memory,
                power,
                power_limit,
            };
            gpu_details.push(gpu_detail);
        }
    }

    Ok(gpu_details)
}

#[tauri::command]
pub async fn get_gpu_detail(host: &str) -> Result<String, String> {
    match get_gpu_detail_l(host).await {
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
            error!("get_gpu_detail: {}", err);
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
    async fn test_top() {
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

        let ret = get_gpu_detail(format!("{}:{}", host, port).as_str())
            .await
            .unwrap();
        info!("{}", ret);
        disconnect_ssh(format!("{}:{}", host, port).as_str());
    }
}
