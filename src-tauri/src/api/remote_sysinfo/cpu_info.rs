use super::Response;
use crate::ssh::ssh_api::*;
use log::error;
use regex::Regex;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
struct CpuInfo {
    user_usage: f32,
    sys_usage: f32,
    usage: f32,
}

//CPU总体信息
fn get_cpu_info_l(host: &str) -> Result<CpuInfo, String> {
    let output = exec_ssh_command(host, "vmstat").map_err(|e| e.to_string())?;

    let cpu_re = Regex::new(
        r"\d+\s+\d+\s+\d+\s+\d+\s+\d+\s+\d+\s+\d+\s+\d+\s+\d+\s+\d+\s+\d+\s+\d+\s+(\d+)\s+(\d+)\s+\d+\s+\d+\s+\d",
    ).unwrap();
    let cpu_caps = cpu_re.captures(&output).ok_or("No CPU data found")?;
    let user_usage = cpu_caps[1].parse::<f32>().map_err(|e| e.to_string())? / 100.0;
    let sys_usage = cpu_caps[2].parse::<f32>().map_err(|e| e.to_string())? / 100.0;

    Ok(CpuInfo {
        user_usage: user_usage,
        sys_usage: sys_usage,
        usage: user_usage + sys_usage, // 将 user_usage 和 sys_usage 相加
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

    #[test]
    fn test_get_cpu_info() {
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
