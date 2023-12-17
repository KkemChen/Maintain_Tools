use super::Response;
use crate::ssh::ssh_api::*;
use regex::Regex;
use serde::{Deserialize, Serialize};
use serde_json::json;

#[derive(Serialize, Deserialize, Debug)]
struct CpuDetail {
    index: u32,
    useage: f32,
}

fn get_cpu_detail_l(host: &str) -> Result<String, String> {
    let output = exec_ssh_command_on_shell(host, "cat /proc/stat")?;
    // println!("{}", output);

    let cpu_line_re = Regex::new(r"^cpu\d+").map_err(|e| e.to_string())?;

    let mut cpu_details = Vec::new();

    for line in output.lines() {
        if cpu_line_re.is_match(line) {
            let parts: Vec<&str> = line.split_whitespace().collect();
            if parts.len() >= 5 {
                let user_time: u64 = parts[1].parse().unwrap_or(0);
                let system_time: u64 = parts[3].parse().unwrap_or(0);
                let idle_time: u64 = parts[4].parse().unwrap_or(0);
                let total_time = user_time + system_time + idle_time;

                let usage = if total_time > 0 {
                    100.0 - (idle_time as f32 / total_time as f32 * 100.0)
                } else {
                    0.0
                };

                let cpu_index = parts[0]
                    .trim_start_matches("cpu")
                    .parse::<usize>()
                    .unwrap_or(0);

                cpu_details.push(CpuDetail {
                    index: cpu_index as u32,
                    useage: usage,
                });
            }
        }
    }

    let json = serde_json::to_string(&cpu_details).map_err(|e| e.to_string())?;
    // println!("JSON output: {}", json);
    Ok(json)
}

#[tauri::command]
pub fn get_cpu_detail(host: &str) -> Result<String, String> {
    match get_cpu_detail_l(host) {
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
    fn test_top() {
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

        get_cpu_detail(format!("{}:{}", host, port).as_str());
        disconnect_ssh(format!("{}:{}", host, port).as_str());
    }
}
