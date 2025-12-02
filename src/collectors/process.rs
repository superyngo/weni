use serde::Serialize;
use sysinfo::{ProcessRefreshKind, ProcessesToUpdate, System};

#[derive(Debug, Clone, Serialize)]
pub struct ProcessEntry {
    pub pid: u32,
    pub name: String,
    pub cpu_usage: f32,
    pub memory_usage: u64,
    pub disk_read: u64,
    pub disk_write: u64,
}

#[derive(Debug, Clone, Serialize)]
pub struct ProcessInfo {
    pub processes: Vec<ProcessEntry>,
    pub total_count: usize,
}

impl ProcessInfo {
    pub fn collect(top_n: Option<usize>, sort_by_cpu: bool) -> Self {
        let mut sys = System::new();

        // Refresh processes
        sys.refresh_processes_specifics(
            ProcessesToUpdate::All,
            ProcessRefreshKind::new()
                .with_cpu()
                .with_memory()
                .with_disk_usage(),
        );

        // Need to refresh twice to get accurate CPU usage
        std::thread::sleep(std::time::Duration::from_millis(200));
        sys.refresh_processes_specifics(
            ProcessesToUpdate::All,
            ProcessRefreshKind::new()
                .with_cpu()
                .with_memory()
                .with_disk_usage(),
        );

        let mut processes: Vec<ProcessEntry> = sys
            .processes()
            .iter()
            .map(|(pid, process)| {
                let disk_usage = process.disk_usage();
                ProcessEntry {
                    pid: pid.as_u32(),
                    name: process.name().to_string_lossy().to_string(),
                    cpu_usage: process.cpu_usage(),
                    memory_usage: process.memory(),
                    disk_read: disk_usage.read_bytes,
                    disk_write: disk_usage.written_bytes,
                }
            })
            .collect();

        let total_count = processes.len();

        // Sort processes
        if sort_by_cpu {
            processes.sort_by(|a, b| b.cpu_usage.partial_cmp(&a.cpu_usage).unwrap());
        } else {
            // Default: sort by memory usage
            processes.sort_by(|a, b| b.memory_usage.cmp(&a.memory_usage));
        }

        // Limit to top N if specified
        if let Some(n) = top_n {
            processes.truncate(n);
        }

        Self {
            processes,
            total_count,
        }
    }
}
