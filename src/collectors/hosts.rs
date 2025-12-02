use serde::Serialize;
use std::fs;
use std::path::Path;

#[derive(Debug, Clone, Serialize)]
pub struct HostEntry {
    pub ip: String,
    pub hostnames: Vec<String>,
}

#[derive(Debug, Clone, Serialize)]
pub struct HostsInfo {
    pub entries: Vec<HostEntry>,
    pub error: Option<String>,
}

impl HostsInfo {
    pub fn collect(filter_comments: bool) -> Self {
        let hosts_path = if cfg!(windows) {
            r"C:\Windows\System32\drivers\etc\hosts"
        } else {
            "/etc/hosts"
        };

        match Self::read_hosts_file(hosts_path, filter_comments) {
            Ok(entries) => Self {
                entries,
                error: None,
            },
            Err(e) => Self {
                entries: vec![],
                error: Some(format!("無法讀取 hosts 檔案: {} (可能需要管理員權限)", e)),
            },
        }
    }

    fn read_hosts_file(path: &str, filter_comments: bool) -> Result<Vec<HostEntry>, std::io::Error> {
        let content = fs::read_to_string(Path::new(path))?;
        let mut entries = Vec::new();

        for line in content.lines() {
            let trimmed = line.trim();

            // Skip empty lines
            if trimmed.is_empty() {
                continue;
            }

            // Skip comments if filtering is enabled
            if filter_comments && trimmed.starts_with('#') {
                continue;
            }

            // Handle inline comments
            let line_content = if let Some(pos) = trimmed.find('#') {
                if filter_comments {
                    trimmed[..pos].trim()
                } else {
                    trimmed
                }
            } else {
                trimmed
            };

            // Skip if line is only a comment
            if line_content.is_empty() {
                continue;
            }

            // Parse IP and hostnames
            let parts: Vec<&str> = line_content.split_whitespace().collect();
            if parts.len() >= 2 {
                let ip = parts[0].to_string();
                let hostnames: Vec<String> = parts[1..].iter().map(|s| s.to_string()).collect();

                entries.push(HostEntry { ip, hostnames });
            }
        }

        Ok(entries)
    }
}
