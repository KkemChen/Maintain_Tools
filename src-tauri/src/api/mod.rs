use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Response<T> {
    code: i32,
    message: String,
    data: Option<T>,
}

pub mod local_sysinfo;
pub mod remote_sysinfo;
