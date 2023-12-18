use super::Response;
use crate::ssh::ssh_api::*;
use regex::Regex;
use serde::{Deserialize, Serialize};
use tokio::time::{sleep, Duration};

#[derive(Serialize, Deserialize, Debug)]
struct CpuDetail {
    index: u32,
    useage: f64,
}
struct CpuTime {
    index: usize,
    user_time: u64,
    nice_time: u64,
    system_time: u64,
    idle_time: u64,
    iowait_time: u64,
    irq_time: u64,
    softirq_time: u64,
    steal_time: u64,
}

impl CpuTime {
    fn total_time(&self) -> u64 {
        self.user_time
            + self.nice_time
            + self.system_time
            + self.idle_time
            + self.iowait_time
            + self.irq_time
            + self.softirq_time
            + self.steal_time
    }
}

fn parse_cpu_times(output: &str) -> Result<Vec<CpuTime>, String> {
    let cpu_line_re = Regex::new(r"^cpu(\d+)").map_err(|e| e.to_string())?;
    let mut cpu_times = Vec::new();

    for line in output.lines() {
        if let Some(caps) = cpu_line_re.captures(line) {
            let parts: Vec<&str> = line.split_whitespace().collect();
            if parts.len() >= 11 {
                let index = caps[1].parse().unwrap_or(0);
                let user_time: u64 = parts[1].parse().unwrap_or(0);
                let nice_time: u64 = parts[2].parse().unwrap_or(0);
                let system_time: u64 = parts[3].parse().unwrap_or(0);
                let idle_time: u64 = parts[4].parse().unwrap_or(0);
                let iowait_time: u64 = parts[5].parse().unwrap_or(0);
                let irq_time: u64 = parts[6].parse().unwrap_or(0);
                let softirq_time: u64 = parts[7].parse().unwrap_or(0);
                let steal_time: u64 = parts[8].parse().unwrap_or(0);

                cpu_times.push(CpuTime {
                    index,
                    user_time,
                    nice_time,
                    system_time,
                    idle_time,
                    iowait_time,
                    irq_time,
                    softirq_time,
                    steal_time,
                });
            }
        }
    }
    Ok(cpu_times)
}

async fn get_cpu_detail_l(host: &str) -> Result<String, String> {
    let initial_output = exec_ssh_command_on_shell(host, "cat /proc/stat")?;
    let initial_cpu_times = parse_cpu_times(&initial_output)?;

    // 等待一段时间
    sleep(Duration::from_millis(300)).await;

    // 第二次读取 /proc/stat
    let final_output = exec_ssh_command_on_shell(host, "cat /proc/stat")?;
    let final_cpu_times = parse_cpu_times(&final_output)?;

    // 计算 CPU 使用率
    let mut cpu_details = Vec::new();
    for (initial, final_) in initial_cpu_times.iter().zip(final_cpu_times.iter()) {
        let idle_diff = final_.idle_time - initial.idle_time;
        let total_diff = final_.total_time() - initial.total_time();

        let usage = if total_diff > 0 {
            100.0 * (1.0 - (idle_diff as f64 / total_diff as f64))
        } else {
            0.0
        };

        cpu_details.push(CpuDetail {
            index: final_.index as u32,
            useage: usage,
        });
    }

    // 序列化为 JSON 字符串
    let json = serde_json::to_string(&cpu_details).map_err(|e| e.to_string())?;
    Ok(json)
}

#[tauri::command]
pub async fn get_cpu_detail(host: &str) -> Result<String, String> {
    match get_cpu_detail_l(host).await {
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
