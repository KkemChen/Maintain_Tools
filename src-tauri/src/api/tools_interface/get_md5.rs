use std::vec;

use super::Response;
use crate::{app, ssh::ssh_api::*};
use log::*;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use tokio::task;
#[derive(Serialize, Deserialize, Debug)]
struct Md5Info {
    date: String,
    md5: String,
}

async fn get_md5_l(host: &str, path: &str) -> Result<String, String> {
    let host_clone = host.to_string();
    let command = format!("md5sum {} | awk '{{print $1}}' | tr -d '\\n'", path);

    match task::spawn_blocking(move || exec_ssh_command(host_clone.as_str(), &command)).await {
        Ok(result) => result,
        Err(join_error) => Err(join_error.to_string()),
    }
}

async fn get_date_l(host: &str, path: &str) -> Result<String, String> {
    let host_clone = host.to_string();
    let command = format!(
        "date -d @$(stat -c %Y {}) \"+%Y-%m-%d %H:%M:%S\" | tr -d '\n'",
        path
    );

    match task::spawn_blocking(move || exec_ssh_command(host_clone.as_str(), &command)).await {
        Ok(result) => result,
        Err(join_error) => Err(join_error.to_string()),
    }
}

#[tauri::command]
pub async fn get_md5(host: &str, path: &str) -> Result<String, String> {
    let mut md5_info = Md5Info {
        date: String::new(), // 使用默认值初始化
        md5: String::new(),  // 使用默认值初始化
    };

    if let Ok(md5) = get_md5_l(host, path).await {
        md5_info.md5 = md5;
    }

    if let Ok(date) = get_date_l(host, path).await {
        md5_info.date = date;
    }

    let response = Response {
        code: 0,
        message: "success".to_string(),
        data: Some(md5_info),
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
    async fn test_get_md5() {
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
        let result = get_md5(
            format!("{}:{}", host, port,).as_str(),
            "/opt/ivauto_ivs_server/data/prison-rt",
        )
        .await;
        info!("{}", result.unwrap());
        // assert!(result.is_ok());
        disconnect_ssh(format!("{}:{}", host, port).as_str());
    }
}
