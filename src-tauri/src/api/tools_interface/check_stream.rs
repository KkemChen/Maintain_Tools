use super::Response;
use crate::ssh::ssh_api::*;
use regex::Regex;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use tokio::task;

#[derive(Serialize, Deserialize, Debug)]
struct VideoInfo {
    index: i32,
    codec_name: String,
    codec_type: String,
    width: i32,
    height: i32,
    pix_fmt: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct AudioInfo {
    index: i32,
    codec_name: String,
    codec_type: String,
    sample_fmt: String,
    sample_rate: String,
    channels: i32,
}

async fn check_stream_l(host: &str, url: &str) -> Result<String, String> {
    let host_clone = host.to_string();
    let url_clone = url.to_string();

    match task::spawn_blocking(move || {
        exec_ssh_command(
            &host_clone,
            &format!(
                "timeout 10 ffprobe -v quiet -print_format json -show_streams {}",
                url_clone
            ),
        )
    })
    .await
    {
        Ok(result) => result,
        Err(join_error) => Err(join_error.to_string()),
    }
}

#[tauri::command]
pub async fn check_stream(host: &str, url: &str) -> Result<String, String> {
    let output = check_stream_l(host, url).await?;

    println!("xx: {}", output);
    Ok(output)
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::ssh::ssh_api::*;
    use dotenv::dotenv;
    use std::env;

    #[tokio::test]
    async fn test_net_info() {
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
        let result = check_stream(
            format!("{}:{}", host, port,).as_str(),
            "rtsp://47.243.129.22:1554/live/test",
        )
        .await;
        assert!(result.is_ok());
        disconnect_ssh(format!("{}:{}", host, port).as_str());
    }
}
