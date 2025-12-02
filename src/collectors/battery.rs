use anyhow::Result;
use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
pub struct BatteryData {
    pub state: String,
    pub percentage: f32,
    pub time_to_full: Option<String>,
    pub time_to_empty: Option<String>,
    pub health: f32,
    pub technology: String,
    pub temperature: Option<f32>,
}

#[derive(Debug, Clone, Serialize)]
pub struct BatteryInfo {
    pub data: Option<BatteryData>,
    pub error: Option<String>,
}

// Battery functionality is disabled on i686-pc-windows-msvc due to battery crate compilation issues
#[cfg(all(target_os = "windows", target_arch = "x86"))]
impl BatteryInfo {
    pub fn collect() -> Self {
        // Battery crate doesn't compile on i686-pc-windows-msvc
        Self {
            data: None,
            error: Some("Battery information is not supported on 32-bit Windows".to_string()),
        }
    }
}

#[cfg(not(all(target_os = "windows", target_arch = "x86")))]
impl BatteryInfo {
    pub fn collect() -> Self {
        use battery::{Manager, State};

        let manager = match Manager::new() {
            Ok(m) => m,
            Err(e) => return Self {
                data: None,
                error: Some(format!("Failed to initialize battery manager: {}", e)),
            },
        };

        let batteries: Vec<_> = match manager.batteries() {
            Ok(iter) => match iter.collect::<Result<Vec<_>, _>>() {
                Ok(b) => b,
                Err(e) => return Self {
                    data: None,
                    error: Some(format!("Failed to enumerate batteries: {}", e)),
                },
            },
            Err(e) => return Self {
                data: None,
                error: Some(format!("Failed to access batteries: {}", e)),
            },
        };

        if batteries.is_empty() {
            return Self {
                data: None,
                error: Some("No battery found (this may be a desktop system)".to_string()),
            };
        }

        let battery = &batteries[0];

        let state = match battery.state() {
            State::Charging => "Charging",
            State::Discharging => "Discharging",
            State::Full => "Full",
            State::Empty => "Empty",
            _ => "Unknown",
        }
        .to_string();

        let percentage = battery.state_of_charge().get::<battery::units::ratio::percent>();

        let time_to_full = battery.time_to_full().map(|duration| {
            let secs = duration.get::<battery::units::time::second>() as u64;
            Self::format_duration(secs)
        });

        let time_to_empty = battery.time_to_empty().map(|duration| {
            let secs = duration.get::<battery::units::time::second>() as u64;
            Self::format_duration(secs)
        });

        let health = battery.state_of_health().get::<battery::units::ratio::percent>();

        let technology = format!("{:?}", battery.technology());

        let temperature = battery.temperature().map(|t| {
            t.get::<battery::units::thermodynamic_temperature::degree_celsius>()
        });

        Self {
            data: Some(BatteryData {
                state,
                percentage,
                time_to_full,
                time_to_empty,
                health,
                technology,
                temperature,
            }),
            error: None,
        }
    }

    fn format_duration(seconds: u64) -> String {
        let hours = seconds / 3600;
        let minutes = (seconds % 3600) / 60;
        format!("{}h {}m", hours, minutes)
    }
}
