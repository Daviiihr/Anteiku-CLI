use std::thread;
use std::time::Instant;

use chrono::Utc;
use sysinfo::{
    Components, CpuRefreshKind, Disks, MINIMUM_CPU_UPDATE_INTERVAL, MemoryRefreshKind, Networks,
    ProcessRefreshKind, ProcessesToUpdate, RefreshKind, System,
};

use crate::collector::Collector;
use crate::error::CollectorError;
use crate::model::{CpuInfo, DiskInfo, MemInfo, NetInfo, ProcInfo, Snapshot, SystemInfo, TempInfo};

/// Recolector real de metricas del sistema basado en `sysinfo`.
pub struct SystemCollector {
    system: System,
    disks: Disks,
    networks: Networks,
    components: Components,
    static_sys_info: StaticSystemInfo,
    last_net_refresh: Instant,
}

struct StaticSystemInfo {
    os_name: String,
    kernel: String,
    hostname: String,
    username: String,
}

impl SystemCollector {
    /// Inicializa el recolector con refrescos selectivos y realiza la espera
    /// minima para que la primera llamada a `collect()` tenga deltas validos de CPU y red.
    pub fn new() -> Result<Self, CollectorError> {
        let refresh_kind = Self::tick_refresh_kind();

        let system = System::new_with_specifics(refresh_kind).map_err(|e| {
            CollectorError::Initialization {
                component: "system",
                reason: e.to_string(),
            }
        })?;

        let disks =
            Disks::new_with_refreshed_list().map_err(|e| CollectorError::Initialization {
                component: "disks",
                reason: e.to_string(),
            })?;

        let networks =
            Networks::new_with_refreshed_list().map_err(|e| CollectorError::Initialization {
                component: "networks",
                reason: e.to_string(),
            })?;
        let last_net_refresh = Instant::now();

        let components =
            Components::new_with_refreshed_list().map_err(|e| CollectorError::Initialization {
                component: "components",
                reason: e.to_string(),
            })?;

        let static_sys_info = StaticSystemInfo {
            os_name: System::name().unwrap_or_else(|_| "Desconocido".to_string()),
            kernel: System::kernel_version().unwrap_or_else(|_| "Desconocido".to_string()),
            hostname: System::host_name().unwrap_or_else(|_| "Desconocido".to_string()),
            username: std::env::var("USER")
                .or_else(|_| std::env::var("USERNAME"))
                .unwrap_or_else(|_| "usuario".to_string()),
        };

        // `sysinfo` necesita un intervalo mínimo entre la primera y segunda lectura
        // para calcular el porcentaje de CPU y las tasas de red por diferencia.
        thread::sleep(MINIMUM_CPU_UPDATE_INTERVAL);

        Ok(Self {
            system,
            disks,
            networks,
            components,
            static_sys_info,
            last_net_refresh,
        })
    }

    fn tick_refresh_kind() -> RefreshKind {
        RefreshKind::nothing()
            .with_cpu(CpuRefreshKind::nothing().with_usage().with_frequency())
            .with_memory(MemoryRefreshKind::everything())
            .with_processes(ProcessRefreshKind::nothing().with_cpu().with_memory())
    }
}

impl Collector for SystemCollector {
    fn collect(&mut self) -> Result<Snapshot, CollectorError> {
        let refresh_kind = Self::tick_refresh_kind();
        self.system.refresh_specifics(refresh_kind);
        self.system.refresh_processes_specifics(
            ProcessesToUpdate::All,
            true,
            ProcessRefreshKind::nothing().with_cpu().with_memory(),
        );
        self.disks.refresh(true);
        self.networks.refresh(true);
        self.components.refresh(true);

        let elapsed_secs = self.last_net_refresh.elapsed().as_secs_f64().max(0.001);
        self.last_net_refresh = Instant::now();

        let per_core: Vec<f32> = self.system.cpus().iter().map(|c| c.usage()).collect();
        let frequency_mhz = self.system.cpus().first().map(|c| c.frequency());

        let cpu = CpuInfo {
            total_percent: self.system.global_cpu_usage(),
            per_core,
            frequency_mhz,
        };

        let memory = MemInfo {
            used_bytes: self.system.used_memory(),
            total_bytes: self.system.total_memory(),
            swap_used_bytes: self.system.used_swap(),
            swap_total_bytes: self.system.total_swap(),
        };

        let disks = self
            .disks
            .iter()
            .map(|d| {
                let total = d.total_space();
                let available = d.available_space();
                DiskInfo {
                    mount_point: d.mount_point().to_string_lossy().into_owned(),
                    used_bytes: total.saturating_sub(available),
                    total_bytes: total,
                }
            })
            .collect();

        let networks = self
            .networks
            .iter()
            .map(|(iface, data)| NetInfo {
                interface: iface.to_string_lossy().into_owned(),
                rx_bytes_per_sec: (data.received() as f64 / elapsed_secs) as u64,
                tx_bytes_per_sec: (data.transmitted() as f64 / elapsed_secs) as u64,
            })
            .collect();

        let processes = self
            .system
            .processes()
            .iter()
            .map(|(pid, proc_)| ProcInfo {
                pid: pid.as_u32(),
                name: proc_.name().to_string_lossy().into_owned(),
                cpu_percent: proc_.cpu_usage(),
                memory_bytes: proc_.memory(),
            })
            .collect();

        let temperatures = self
            .components
            .iter()
            .filter_map(|comp| {
                comp.temperature().map(|celsius| TempInfo {
                    label: comp.label().to_string(),
                    celsius,
                })
            })
            .collect();

        let system = SystemInfo {
            os_name: self.static_sys_info.os_name.clone(),
            kernel: self.static_sys_info.kernel.clone(),
            hostname: self.static_sys_info.hostname.clone(),
            username: self.static_sys_info.username.clone(),
            uptime_secs: System::uptime().unwrap_or(0),
        };

        Ok(Snapshot {
            timestamp: Utc::now(),
            cpu,
            memory,
            disks,
            networks,
            processes,
            temperatures,
            system,
        })
    }
}
