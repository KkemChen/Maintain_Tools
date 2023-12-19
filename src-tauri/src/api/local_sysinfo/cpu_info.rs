use serde::{Deserialize, Serialize};
use sysinfo::{CpuExt, CpuRefreshKind, System, SystemExt};

#[derive(Serialize, Deserialize)]
struct CpuInfo {
    index: usize,
    usage: f32,
}

#[tauri::command]
pub fn get_cpu_info_local() -> Result<String, String> {
    let mut sys = System::new_all();
    // sys.refresh_cpu();
    sys.refresh_cpu_specifics(CpuRefreshKind::everything());
    std::thread::sleep(System::MINIMUM_CPU_UPDATE_INTERVAL); // and also double interval
    sys.refresh_cpu_specifics(CpuRefreshKind::new().with_cpu_usage());
    let cpus: Vec<CpuInfo> = sys
        .cpus()
        .iter()
        .enumerate()
        .map(|(index, cpu)| CpuInfo {
            index,
            usage: cpu.cpu_usage(),
        })
        .collect();

    serde_json::to_string(&cpus).map_err(|e| e.to_string())
}
