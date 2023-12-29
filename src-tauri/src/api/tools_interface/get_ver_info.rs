use std::vec;

use super::Response;
use crate::{app, ssh::ssh_api::*};
use log::*;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use tokio::task;

#[derive(Serialize, Deserialize, Debug)]
struct VerInfo {
    date: String,
    commit_hash: String,
}

async fn get_commit_hash_l(host: &str, path: &str) -> Result<String, String> {
    let host_clone = host.to_string();
    let command = format!("sed -n '2p' {} | tr -d '\\n'", path);

    match task::spawn_blocking(move || exec_ssh_command(host_clone.as_str(), &command)).await {
        Ok(result) => result,
        Err(join_error) => Err(join_error.to_string()),
    }
}

async fn get_date_l(host: &str, path: &str) -> Result<String, String> {
    let host_clone = host.to_string();
    let command = format!("date -d @$(stat -c %Y {}) \"+%Y-%m-%d %H:%M:%S\" | tr -d '\n'", path);

    match task::spawn_blocking(move || exec_ssh_command(host_clone.as_str(), &command)).await {
        Ok(result) => result,
        Err(join_error) => Err(join_error.to_string()),
    }
}

#[tauri::command]
pub async fn get_ver_info(host: &str, path: &str) -> Result<String, String> {
    let mut ver_info = VerInfo {
        date: String::new(),        // 使用默认值初始化
        commit_hash: String::new(), // 使用默认值初始化
    };

    if let Ok(hash) = get_commit_hash_l(host, path).await {
        ver_info.commit_hash = hash;
    }

    if let Ok(date) = get_date_l(host, path).await {
        ver_info.date = date;
    }

    let response = Response {
        code: 0,
        message: "success".to_string(),
        data: Some(ver_info),
    };
    serde_json::to_string(&response).map_err(|e| e.to_string())
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
        let result = get_ver_info(
            format!("{}:{}", host, port,).as_str(),
            "/opt/ivauto_ivs_server/ivs_ver.txt",
        )
        .await;
        info!("{}", result.unwrap());
        // assert!(result.is_ok());
        disconnect_ssh(format!("{}:{}", host, port).as_str());
    }
}
