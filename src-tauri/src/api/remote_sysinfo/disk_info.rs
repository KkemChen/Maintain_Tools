use super::Response;
use crate::ssh::ssh_api::*;
use log::*;

fn get_disk_info_l(host: &str) -> Result<String, String> {
    let command = "df --output=pcent | awk \'NR>1 {sum+=$1; count++} END {print sum/count}\'";

    let output = exec_ssh_command(host, command)?;

    Ok(output.trim().to_string())
}

#[tauri::command]
pub fn get_disk_info(host: &str) -> Result<String, String> {
    match get_disk_info_l(host) {
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
            error!("get_disk_info failed, err: {}", err);
            serde_json::to_string(&response).map_err(|e| e.to_string())
            /*  error!("get_disk_info failed, err: {}", err);
            Err(err) // 直接返回错误 */
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::ssh::ssh_api::*;
    use dotenv::dotenv;
    use std::env;
    use std::{thread, time::Duration};

    #[test]
    fn test_get_process_info() {
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

        let ret = get_disk_info(format!("{}:{}", host, port).as_str()).unwrap();
        info!("{}", ret);
        disconnect_ssh(format!("{}:{}", host, port).as_str());
    }
}
