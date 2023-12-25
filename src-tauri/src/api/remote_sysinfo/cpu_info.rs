use super::Response;
use crate::ssh::ssh_api::*;
use log::*;
use regex::Regex;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct CpuInfo {
    user_usage: f32,
    sys_usage: f32,
    usage: f32,
}

//CPU总体信息
pub fn get_cpu_info_l(host: &str) -> Result<CpuInfo, String> {
    let output = exec_ssh_command(host, "top -b -n 1").map_err(|e| e.to_string())?;

    // 匹配 top 命令输出中的 CPU 使用率行，这里的正则表达式可能需要根据实际输出进行调整
    let cpu_re =
        Regex::new(r"%Cpu\(s\):.*?(\d+\.\d+) us.*?(\d+\.\d+) sy.*?(\d+\.\d+) id.*?").unwrap();
    let cpu_caps = cpu_re.captures(&output).ok_or("No CPU data found")?;
    let user_usage = cpu_caps[1].parse::<f32>().map_err(|e| e.to_string())? / 100.0;
    let sys_usage = cpu_caps[2].parse::<f32>().map_err(|e| e.to_string())? / 100.0;
    // 计算空闲时间，并从 1 减去得到总的 CPU 使用率
    let idle = cpu_caps[3].parse::<f32>().map_err(|e| e.to_string())? / 100.0;
    let total_usage = 1.0 - idle;

    Ok(CpuInfo {
        user_usage: user_usage,
        sys_usage: sys_usage,
        usage: total_usage, // 使用 total_usage 代替之前的 user_usage + sys_usage
    })
}

#[tauri::command]
pub fn get_cpu_info(host: &str) -> Result<String, String> {
    match get_cpu_info_l(host) {
        Ok(cpu_info) => {
            let res = Response {
                code: 0,
                message: "success".to_string(),
                data: Some(cpu_info),
            };
            serde_json::to_string(&res).map_err(|e| e.to_string())
        }
        Err(err) => {
            let res = Response::<String> {
                code: -1,
                message: format!("Err: {}", err),
                data: None,
            };
            error!("get_cpu_info failed, err: {}", err);
            serde_json::to_string(&res).map_err(|e| e.to_string())
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::ssh::ssh_api::*;
    use dotenv::dotenv;
    use log::*;
    use std::env;

    #[tokio::test]
    async fn test_get_cpu_info() {
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

        match get_cpu_info(format!("{}:{}", host, port).as_str()) {
            Ok(res) => {
                info!("success: {}", res);
            }
            Err(err) => {
                error!("{}", err.to_string())
            }
        }

        disconnect_ssh(format!("{}:{}", host, port).as_str());
    }
}
