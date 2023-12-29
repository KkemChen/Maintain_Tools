use super::Response;
use crate::ssh::ssh_api::*;
use log::*;
use regex::Regex;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct CpuInfo {
    user_usage: f32,
    sys_usage: f32,
    pub usage: f32,
}

//CPU总体信息
pub fn get_cpu_info_l(host: &str) -> Result<CpuInfo, String> {
    let output = exec_ssh_command(host, "top -b -n 2").map_err(|e| e.to_string())?;

    // 使用时间戳作为分隔符来分割输出
    let section_re = Regex::new(r"top -").unwrap();
    let sections: Vec<_> = section_re.split(&output).collect();

    let last_output = sections.last().ok_or("No CPU data found in the output")?;

    let cpu_re =
        Regex::new(r"%Cpu\(s\):\s*(\d+\.\d+)\s*us,\s*(\d+\.\d+)\s*sy,\s*.*,\s*(\d+\.\d+)\s*id,.*")
            .map_err(|err| err.to_string())?;

    // 应用正则表达式到最后一次迭代的输出
    let cpu_caps = cpu_re.captures(last_output).ok_or("No CPU data found")?;
    let user_usage = cpu_caps[1].parse::<f32>().map_err(|e| e.to_string())? / 100.0;
    let sys_usage = cpu_caps[2].parse::<f32>().map_err(|e| e.to_string())? / 100.0;
    let idle = cpu_caps[3].parse::<f32>().map_err(|e| e.to_string())? / 100.0;
    let total_usage = user_usage + sys_usage;

    Ok(CpuInfo {
        user_usage: user_usage,
        sys_usage: sys_usage,
        usage: total_usage,
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
