use sysinfo::{NetworkExt, NetworksExt, ProcessExt, System, SystemExt,CpuExt};
use serde_json::error::Error as SerdeError;
use serde::{Serialize,Deserialize};

#[derive(Serialize, Deserialize)]
struct CpuInfo {
    index: usize,
    usage: f32,
}


#[tauri::command]
pub fn get_cpu_info() -> Result<String, String> {
    let mut sys = System::new_all();
    sys.refresh_cpu();

    let cpus: Vec<CpuInfo> = sys.cpus().iter().enumerate()
        .map(|(index, cpu)| CpuInfo {
            index,
            usage: cpu.cpu_usage(),
        })
        .collect();

    serde_json::to_string(&cpus).map_err(|e| e.to_string())
}


