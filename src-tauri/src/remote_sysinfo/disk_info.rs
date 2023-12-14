use crate::ssh::ssh_api::*;
use regex::Regex;
use serde::{Deserialize, Serialize};
use serde_json::json;

#[derive(Serialize, Deserialize, Debug)]
struct DiskInfo {
    name: String,
    size: String,
    used: String,
    avail: String,
    use_percentage: String,
    mounted_on: String,
}

#[tauri::command]
pub fn get_disk_info(host: &str) -> Result<String, String> {
    let output = exec_ssh_command(host, "df -h")?;
    //println!("{}", output);

    /* let re = Regex::new(
        r"(?m)^(?P<name>\S+)\s+(?P<size>\S+)\s+(?P<used>\S+)\s+(?P<avail>\S+)\s+(?P<use_percentage>\S+)%\s+(?P<mounted_on>\S+)$"
    ).map_err(|e| e.to_string())?; */

    //匹配百分号
    let re = Regex::new(
        r"(?m)^(?P<name>\S+)\s+(?P<size>\S+)\s+(?P<used>\S+)\s+(?P<avail>\S+)\s+(?P<use_percentage>\S+%)\s+(?P<mounted_on>\S+)$"
    ).map_err(|e| e.to_string())?;

    let mut disk_infos = Vec::new();

    for cap in re.captures_iter(&output) {
        disk_infos.push(DiskInfo {
            name: cap["name"].to_string(),
            size: cap["size"].to_string(),
            used: cap["used"].to_string(),
            avail: cap["avail"].to_string(),
            use_percentage: cap["use_percentage"].to_string(),
            mounted_on: cap["mounted_on"].to_string(),
        });
    }

    let json = serde_json::to_string(&disk_infos).map_err(|e| e.to_string())?;
    //println!("{}", json);
    Ok(json)
}

#[cfg(test)]
mod test {
    use std::{thread, time::Duration};

    use super::*;
    use crate::ssh::ssh_api::*;

    #[test]
    fn test_get_process_info() {
        add_ssh_connect("192.168.1.172:6622", "root", "ivauto@123");
        get_disk_info("192.168.1.172:6622");
        disconnect_ssh("192.168.1.172:6622");
    }
}
