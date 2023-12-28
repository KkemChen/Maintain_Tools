use super::Response;
use crate::ssh::ssh_api::*;
use log::*;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use tokio::task;

#[derive(Serialize, Deserialize, Debug)]
struct VerInfo {
    name: String,
    date: String,
    commit: String,
    md5: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct Model {
    name: String,
    md5: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct ModelInfo {
    models: Vec<Model>,
    date: String,
    commit: String,
}

async fn get_ver_info_l(host: &str) -> Result<String, String> {
    let host_clone = host.to_string();

    match task::spawn_blocking(move || {
        let model_ver_commit = exec_ssh_command(
            &host_clone,
            "sed -n \'2p\' /opt/ivauto_ivs_server/model_ver.txt",
        );

        let prison_md5 = exec_ssh_command(
            &host_clone,
            "md5sum /opt/ivauto_ivs_server/data/prison-rt | awk \'{print $1}\'
        ",
        );

        let coeff_prison_md5 = exec_ssh_command(
            &host_clone,
            "md5sum /opt/ivauto_ivs_server/data/coeff-prison | awk \'{print $1}\'
        ",
        );

        let ivs_md5 = exec_ssh_command(
            &host_clone,
            "md5sum /opt/ivauto_ivs_server/ivauto_ivs_server | awk \'{print $1}\'
        ",
        );

        let ivqd_md5 = exec_ssh_command(
            &host_clone,
            "md5sum /opt/ivauto_ivs_server/ivauto_quality_detection | awk \'{print $1}\'
        ",
        );

        let ivsm_md5 = exec_ssh_command(
            &host_clone,
            "md5sum /opt/ivauto_ivs_server/ivauto_summary_server | awk \'{print $1}\'
        ",
        );

        let ivs_ver_commit = exec_ssh_command(
            &host_clone,
            "sed -n \'2p\' /opt/ivauto_ivs_server/ivs_ver.txt",
        );

        let qd_ver_commit = exec_ssh_command(
            &host_clone,
            "sed -n \'2p\' /opt/ivauto_ivs_server/qd_ver.txt",
        );

        ivs_md5
    })
    .await
    {
        Ok(result) => result,
        Err(join_error) => Err(join_error.to_string()),
    }
}

#[tauri::command]
pub async fn get_ver_info(host: &str) -> Result<String, String> {
    let output = get_ver_info_l(host).await;
    output
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
        let result = get_ver_info(format!("{}:{}", host, port,).as_str()).await;
        info!("{}", result.unwrap());
        // assert!(result.is_ok());
        disconnect_ssh(format!("{}:{}", host, port).as_str());
    }
}
