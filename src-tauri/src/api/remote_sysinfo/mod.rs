#![allow(warnings)]
use lazy_static::lazy_static;
use log::*;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::{Arc, Mutex, RwLock};
use std::thread;
use tokio::sync::oneshot;
use tokio::task::spawn_blocking;
use tokio::time::{self, Duration};

pub mod cpu_detail;
pub mod cpu_info;
pub mod disk_detail;
pub mod disk_info;
pub mod gpu_detail;
pub mod load_info;
pub mod mem_info;
pub mod net_info;
pub mod process_info;

pub use super::Response;

use cpu_detail::*;
use cpu_info::*;
use disk_detail::*;
use disk_info::*;
use gpu_detail::*;
use load_info::*;
use mem_info::*;
use net_info::*;
use process_info::*;

#[derive(Default, Serialize, Deserialize, Clone)]
struct SysInfo {
    cpu_detail: Vec<CpuDetail>,
    cpu_info: CpuInfo,
    mem_info: MemInfo,
    disk_info: String,
    disk_detail: Vec<DiskDetail>,
    process_info: Vec<ProcessInfo>,
    net_info: Vec<NetInfo>,
    load_info: LoadInfo,
    gpu_detail: Vec<GpuDetail>,
}

lazy_static! {
    // 创建一个全局的、惰性初始化的 SysInfo 实例
    static ref SHARED_SYSINFO: Arc<RwLock<SysInfo>> = Arc::new(RwLock::new(SysInfo::default()));

    static ref CHECK_THREAD: Mutex<HashMap<String, oneshot::Sender<()>>> = Mutex::new(HashMap::new());

}

pub fn start_fetch_sysinfo(host: &str) {
    let host = host.to_string();
    let hosttmp = host.clone();
    let (tx, mut rx) = oneshot::channel();
    tokio::spawn(async move {
        let mut interval = time::interval(Duration::from_secs(3));
        loop {
            tokio::select! {
                _ = interval.tick() => {
                    match get_net_info_l(&host).await {
                        Ok(res) => {
                            let mut sysinfo = SHARED_SYSINFO.write().unwrap();
                            sysinfo.net_info = res;
                        }
                        Err(err) => {
                            error!("{}", err);
                        }
                    }
                    match get_cpu_detail_l(&host).await {
                        Ok(res) => {
                            let mut sysinfo = SHARED_SYSINFO.write().unwrap();
                            sysinfo.cpu_detail = res;
                        }
                        Err(err) => {
                            error!("{}", err);
                        }
                    }

                    match get_cpu_info_l(&host) {
                        Ok(res) => {
                            let mut sysinfo = SHARED_SYSINFO.write().unwrap();
                            sysinfo.cpu_info = res;
                        }
                        Err(err) => {
                            error!("{}", err);
                        }
                    }

                    match get_disk_info_l(&host) {
                        Ok(res) => {
                            let mut sysinfo = SHARED_SYSINFO.write().unwrap();
                            sysinfo.disk_info = res;
                        }
                        Err(err) => {
                            error!("{}", err);
                        }
                    }

                    match get_disk_detail_l(&host) {
                        Ok(res) => {
                            let mut sysinfo = SHARED_SYSINFO.write().unwrap();
                            sysinfo.disk_detail = res;
                        }
                        Err(err) => {
                            error!("{}", err);
                        }
                    }

                    match get_load_info_l(&host) {
                        Ok(res) => {
                            let mut sysinfo = SHARED_SYSINFO.write().unwrap();
                            sysinfo.load_info = res;
                        }
                        Err(err) => {
                            error!("{}", err);
                        }
                    }

                    match get_mem_info_l(&host) {
                        Ok(res) => {
                            let mut sysinfo = SHARED_SYSINFO.write().unwrap();
                            sysinfo.mem_info = res;
                        }
                        Err(err) => {
                            error!("{}", err);
                        }
                    }

                    match get_process_info_l(&host) {
                        Ok(res) => {
                            let mut sysinfo = SHARED_SYSINFO.write().unwrap();
                            sysinfo.process_info = res;
                        }
                        Err(err) => {
                            error!("{}", err);
                        }
                    }
                    match get_gpu_detail_l(&host).await {
                        Ok(res) => {
                            let mut sysinfo = SHARED_SYSINFO.write().unwrap();
                            sysinfo.gpu_detail = res;
                        }
                        Err(err) => {
                            error!("{}", err);
                        }
                    }
                },
                _ = &mut rx => {
                    info!("exit...");
                    break;
                }

            }
        }
        info!("exit success.");
    });
    let mut map = CHECK_THREAD.lock().unwrap();
    map.insert(hosttmp.to_string(), tx);
}

pub fn stop_fetch_sysinfo(host: &str) {
    let mut map = CHECK_THREAD.lock().unwrap();
    if let Some(sender) = map.remove(host) {
        // 尝试移除并获取host对应的value
        // 如果找到了对应的sender，发送一个空的消息以通知接收端停止
        let _ = sender.send(()); // 忽略结果，因为接收端可能已经丢弃
        info!("Send exit signal to host: {}", host);
    } else {
        // 如果没有找到，可能是因为这个host没有对应的条目，或者已经被移除
        info!("No active fetch task for host: {}", host);
    }
}

#[tauri::command]
pub fn get_sysinfo(host: &str) -> Result<String, String> {
    let sysinfo = SHARED_SYSINFO.read().unwrap();

    let response = Response {
        code: 0,
        message: "success".to_string(),
        data: Some(&*sysinfo),
    };
    serde_json::to_string(&response).map_err(|e| e.to_string())
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::ssh::ssh_api::*;
    use dotenv::dotenv;
    use std::env;
    use std::{thread, time::Duration};

    #[tokio::test]
    async fn test_get_process_info() {
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

        for _ in 0..3 {
            tokio::time::sleep(Duration::from_secs(3)).await;

            match get_sysinfo(format!("{}:{}", host, port).as_str()) {
                Ok(ret) => {
                    info!("{}", ret);
                }
                Err(err) => {
                    error!("{}", err);
                }
            };
        }
        tokio::time::sleep(Duration::from_secs(15)).await;

        disconnect_ssh(format!("{}:{}", host, port).as_str());
        tokio::time::sleep(Duration::from_secs(15)).await;
    }
}
