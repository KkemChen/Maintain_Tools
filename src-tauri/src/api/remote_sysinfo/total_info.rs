use super::Response;
use crate::ssh::ssh_api::*;
use regex::Regex;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
struct ProcessInfo {
    pid: String,
    user: String,
    virt: String,
    res: String,
    cpu: String,
    mem: String,
    command: String,
}
#[derive(Serialize, Deserialize, Debug)]
struct CpuInfo {
    user_useage: f32,
    sys_useage: f32,
    useage: f32,
}
#[derive(Serialize, Deserialize, Debug)]
struct MemInfo {
    used: u32,
    free: u32,
    total: u32,
}
#[derive(Serialize, Deserialize, Debug)]
struct LoadInfo(f32, f32, f32);

#[derive(Serialize, Deserialize, Debug)]
struct SystemInfo {
    process_info: Vec<ProcessInfo>,
    cpu_total_info: CpuInfo,
    mem_info: MemInfo,
    load_info: LoadInfo,
}

pub fn get_total_info_l(host: &str) -> Result<String, String> {
    let output = exec_ssh_command_on_shell(host, "top -b -n 1")?;
    // println!("{}", output);

    let process_re = Regex::new(
        r"(?m)^\s*(\d+)\s+(\S+)\s+\d+\s+\d+\s+(\S+)\s+(\S+)\s+\S+\s+\S+\s+(\S+)\s+(\S+)\s+\S+\s+(.+)$",
    ).map_err(|e| e.to_string())?;

    let mut process_infos = Vec::new();

    for cap in process_re.captures_iter(&output) {
        process_infos.push(ProcessInfo {
            pid: cap[1].to_string(),
            user: cap[2].to_string(),
            virt: cap[3].to_string(),
            res: cap[4].to_string(),
            cpu: cap[5].to_string(),
            mem: cap[6].to_string(),
            command: cap[7].to_string(),
        });
    }

    // CPU 总体信息
    let cpu_re = Regex::new(r"%Cpu\(s\):\s+(\d+\.\d+) us,\s+(\d+\.\d+) sy").unwrap();
    let cpu_caps = cpu_re.captures(&output).ok_or("No CPU data found")?;
    let user_usage_result = cpu_caps[1].parse::<f32>().map_err(|e| e.to_string());
    let sys_usage_result = cpu_caps[2].parse::<f32>().map_err(|e| e.to_string());

    let user_usage = user_usage_result?;
    let sys_usage = sys_usage_result?;

    let cpu_total_info = CpuInfo {
        user_useage: user_usage,
        sys_useage: sys_usage,
        useage: user_usage + sys_usage, // 将 user_usage 和 sys_usage 相加
    };

    // 内存信息
    let mem_re = Regex::new(r"KiB Mem :\s+(\d+) total,\s+(\d+) free,\s+(\d+) used").unwrap();
    let mem_caps = mem_re.captures(&output).ok_or("No memory data found")?;
    let mem_info = MemInfo {
        total: mem_caps[1].parse::<u32>().map_err(|e| e.to_string())?,
        free: mem_caps[2].parse::<u32>().map_err(|e| e.to_string())?,
        used: mem_caps[3].parse::<u32>().map_err(|e| e.to_string())?,
    };

    // 负载信息
    let load_re = Regex::new(r"load average: (\d+\.\d+), (\d+\.\d+), (\d+\.\d+)").unwrap();
    let load_caps = load_re.captures(&output).ok_or("No load data found")?;
    let load_info = LoadInfo(
        load_caps[1].parse::<f32>().map_err(|e| e.to_string())?,
        load_caps[2].parse::<f32>().map_err(|e| e.to_string())?,
        load_caps[3].parse::<f32>().map_err(|e| e.to_string())?,
    );

    let json = serde_json::to_string(&SystemInfo {
        process_info: process_infos,
        cpu_total_info: cpu_total_info,
        mem_info: mem_info,
        load_info: load_info,
    })
    .map_err(|e| e.to_string())?;
    //  println!("JSON output: {}", json);
    Ok(json)
}

#[tauri::command]
pub fn get_total_info(host: &str) -> Result<String, String> {
    match get_total_info_l(host) {
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

        get_total_info(format!("{}:{}", host, port).as_str());
        disconnect_ssh(format!("{}:{}", host, port).as_str());
    }
}
