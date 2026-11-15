use crate::models::TelemetrySnapshot;
use std::sync::Mutex;
use sysinfo::{Pid, ProcessRefreshKind, RefreshKind, System};

pub struct TelemetryState {
    pub sys: Mutex<System>,
    pub current_pid: Pid,
    pub last_launch_latency_ms: Mutex<f64>,
    pub indexed_items_count: Mutex<usize>,
}

impl TelemetryState {
    pub fn new() -> Self {
        let mut sys = System::new_with_specifics(
            RefreshKind::new()
                .with_cpu(sysinfo::CpuRefreshKind::everything())
                .with_processes(ProcessRefreshKind::new().with_memory()),
        );
        sys.refresh_cpu_usage();
        let pid = sysinfo::get_current_pid().unwrap_or_else(|_| Pid::from(0));
        Self {
            sys: Mutex::new(sys),
            current_pid: pid,
            last_launch_latency_ms: Mutex::new(0.0),
            indexed_items_count: Mutex::new(0),
        }
    }

    pub fn snapshot(&self) -> TelemetrySnapshot {
        let mut sys = self.sys.lock().unwrap();
        sys.refresh_cpu_usage();
        sys.refresh_processes_specifics(
            sysinfo::ProcessesToUpdate::Some(&[self.current_pid]),
            true,
            ProcessRefreshKind::new().with_memory(),
        );

        let cpu_usage_percent = sys.global_cpu_usage();
        let ram_usage_mb = sys
            .process(self.current_pid)
            .map(|p| (p.memory() as f32) / (1024.0 * 1024.0))
            .unwrap_or(0.0);

        let last_launch_latency_ms = *self.last_launch_latency_ms.lock().unwrap();
        let indexed_items_count = *self.indexed_items_count.lock().unwrap();

        TelemetrySnapshot {
            cpu_usage_percent,
            ram_usage_mb,
            indexed_items_count,
            last_launch_latency_ms,
        }
    }
}

#[tauri::command]
pub fn get_system_telemetry(state: tauri::State<'_, TelemetryState>) -> TelemetrySnapshot {
    state.snapshot()
}
