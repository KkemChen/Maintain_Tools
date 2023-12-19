use super::Response;
use crate::ssh::ssh_api::*;
use serde::{Deserialize, Serialize};
use serde_json::Value;
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

#[derive(Serialize, Deserialize, Debug)]
struct StreamInfo {
    video: Vec<VideoInfo>,
    audio: Vec<AudioInfo>,
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

    match serde_json::from_str::<Value>(&output) {
        Ok(json) => {
            let mut video_infos: Vec<VideoInfo> = Vec::new();
            let mut audio_infos: Vec<AudioInfo> = Vec::new();

            if let Some(streams) = json["streams"].as_array() {
                for stream in streams {
                    match stream["codec_type"].as_str() {
                        Some("video") => {
                            let video_info = VideoInfo {
                                index: stream["index"].as_i64().unwrap_or_default() as i32,
                                codec_name: stream["codec_name"]
                                    .as_str()
                                    .unwrap_or_default()
                                    .to_string(),
                                codec_type: stream["codec_type"]
                                    .as_str()
                                    .unwrap_or_default()
                                    .to_string(),
                                width: stream["width"].as_i64().unwrap_or_default() as i32,
                                height: stream["height"].as_i64().unwrap_or_default() as i32,
                                pix_fmt: stream["pix_fmt"].as_str().unwrap_or_default().to_string(),
                            };

                            video_infos.push(video_info);
                        }
                        Some("audio") => {
                            let audio_info = AudioInfo {
                                index: stream["index"].as_i64().unwrap_or_default() as i32,
                                codec_name: stream["codec_name"]
                                    .as_str()
                                    .unwrap_or_default()
                                    .to_string(),
                                codec_type: stream["codec_type"]
                                    .as_str()
                                    .unwrap_or_default()
                                    .to_string(),
                                sample_fmt: stream["sample_fmt"]
                                    .as_str()
                                    .unwrap_or_default()
                                    .to_string(),
                                sample_rate: stream["sample_rate"]
                                    .as_str()
                                    .unwrap_or_default()
                                    .to_string(),
                                channels: stream["channels"].as_i64().unwrap_or_default() as i32,
                            };
                            audio_infos.push(audio_info);
                        }
                        _ => {}
                    }
                }
                let stream_info = StreamInfo {
                    video: video_infos,
                    audio: audio_infos,
                };
                let response = Response {
                    code: 0,
                    message: "success".to_string(),
                    data: Some(stream_info),
                };
                serde_json::to_string(&response).map_err(|e| e.to_string())
            } else {
                let response = Response::<String> {
                    code: -2,
                    message: "No streams found or unknow stream".to_string(),
                    data: None,
                };
                match serde_json::to_string(&response) {
                    Ok(json_string) => Err(json_string),
                    Err(e) => Err(e.to_string()),
                }
            }
        }
        Err(_) => {
            let response = Response::<String> {
                code: -1,
                message: "Timeout".to_string(),
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
            "rtsp://kkem.me:1554/live/test",
        )
        .await;
        println!("{}", result.unwrap());
        // assert!(result.is_ok());
        disconnect_ssh(format!("{}:{}", host, port).as_str());
    }
}
