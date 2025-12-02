use anyhow::Result;
use pico_args::Arguments;

pub struct CliArgs {
    pub show_cpu: bool,
    pub show_memory: bool,
    pub show_system: bool,
    pub show_battery: bool,
    pub show_disk: bool,
    pub show_network: bool,
    pub show_temp: bool,
    pub show_process: bool,
    pub show_hosts: bool,
    pub show_all: bool,
    pub json: bool,
    pub watch: bool,
    pub interval: u64,
    pub process_top: Option<usize>,
    pub process_sort_cpu: bool,
    pub hosts_filter_comments: bool,
    pub help: bool,
}

impl CliArgs {
    pub fn parse() -> Result<Self> {
        let mut args = Arguments::from_env();

        let help = args.contains(["-h", "--help"]);

        if help {
            return Ok(Self {
                show_cpu: false,
                show_memory: false,
                show_system: false,
                show_battery: false,
                show_disk: false,
                show_network: false,
                show_temp: false,
                show_process: false,
                show_hosts: false,
                show_all: false,
                json: false,
                watch: false,
                interval: 2,
                process_top: None,
                process_sort_cpu: false,
                hosts_filter_comments: true,
                help: true,
            });
        }

        let show_cpu = args.contains("--cpu");
        let show_memory = args.contains("--memory");
        let show_system = args.contains("--system");
        let show_battery = args.contains("--battery");
        let show_disk = args.contains("--disk");
        let show_network = args.contains("--network");
        let show_temp = args.contains("--temp");
        let show_process = args.contains("--process");
        let show_hosts = args.contains("--hosts");
        let json = args.contains("--json");
        let watch = args.contains(["-w", "--watch"]);
        let interval: u64 = args.opt_value_from_str(["-i", "--interval"])?.unwrap_or(2);
        let process_top: Option<usize> = args.opt_value_from_str("--top")?;
        let process_sort_cpu = args.contains("--sort-cpu");
        let hosts_filter_comments = !args.contains("--show-comments");

        let show_all = !show_cpu && !show_memory && !show_system && !show_battery && !show_disk && !show_network && !show_temp && !show_process && !show_hosts;

        let remaining = args.finish();
        if !remaining.is_empty() {
            anyhow::bail!("Unknown arguments: {:?}", remaining);
        }

        Ok(Self {
            show_cpu,
            show_memory,
            show_system,
            show_battery,
            show_disk,
            show_network,
            show_temp,
            show_process,
            show_hosts,
            show_all,
            json,
            watch,
            interval,
            process_top,
            process_sort_cpu,
            hosts_filter_comments,
            help,
        })
    }

    pub fn print_help() {
        println!(
            r#"weni - Lightweight cross-platform system information tool

USAGE:
    weni [OPTIONS]

OPTIONS:
    --cpu                 Show CPU information
    --memory              Show memory information
    --system              Show system information
    --battery             Show battery information
    --disk                Show disk information
    --network             Show network information
    --temp                Show temperature information
    --process             Show running processes
    --hosts               Show hosts file contents

GENERAL OPTIONS:
    --json                Output in JSON format
    -w, --watch           Enable watch mode (live updates)
    -i, --interval <SEC>  Update interval in seconds (default: 2)
    -h, --help            Print help information

PROCESS OPTIONS:
    --top <N>             Show only top N processes (sorted by resource usage)
    --sort-cpu            Sort processes by CPU usage (default: by memory)

HOSTS OPTIONS:
    --show-comments       Show comments in hosts file (default: filter out)

EXAMPLES:
    weni                        # Show all information
    weni --cpu --memory         # Show only CPU and memory
    weni --temp                 # Show temperature information
    weni --process              # Show all running processes
    weni --process --top 10     # Show top 10 processes
    weni --process --sort-cpu   # Show processes sorted by CPU usage
    weni --hosts                # Show hosts file contents
    weni --json                 # Output all info as JSON
    weni --watch                # Live monitoring mode
    weni --watch --interval 5   # Monitor with 5 second interval
"#
        );
    }
}
