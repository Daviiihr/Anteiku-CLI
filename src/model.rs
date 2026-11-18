use chrono::{DateTime, Utc};
use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
pub struct Snapshot {
    pub timestamp: DateTime<Utc>,
    pub cpu: CpuInfo,
    pub memory: MemInfo,
    pub disks: Vec<DiskInfo>,
    pub networks: Vec<NetInfo>,
    pub processes: Vec<ProcInfo>,
    pub temperatures: Vec<TempInfo>,
    /// puede venir vacío
    pub system: SystemInfo,
}

#[derive(Debug, Clone, Serialize)]
pub struct CpuInfo {
    pub total_percent: f32,
    pub per_core: Vec<f32>,
    pub frequency_mhz: Option<u64>,
}

#[derive(Debug, Clone, Serialize)]
pub struct MemInfo {
    pub used_bytes: u64,
    pub total_bytes: u64,
    pub swap_used_bytes: u64,
    pub swap_total_bytes: u64,
}

#[derive(Debug, Clone, Serialize)]
pub struct DiskInfo {
    pub mount_point: String,
    pub used_bytes: u64,
    pub total_bytes: u64,
}

#[derive(Debug, Clone, Serialize)]
pub struct NetInfo {
    pub interface: String,
    pub rx_bytes_per_sec: u64,
    pub tx_bytes_per_sec: u64,
}

#[derive(Debug, Clone, Serialize)]
pub struct ProcInfo {
    pub pid: u32,
    pub name: String,
    pub cpu_percent: f32,
    pub memory_bytes: u64,
}

#[derive(Debug, Clone, Serialize)]
pub struct TempInfo {
    pub label: String,
    pub celsius: f32,
}

#[derive(Debug, Clone, Serialize)]
pub struct SystemInfo {
    pub os_name: String,
    pub kernel: String,
    pub hostname: String,
    pub username: String,
    pub uptime_secs: u64,
}
