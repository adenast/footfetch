use std::env;
use std::fs;
use std::process::Command;
use std::thread;
use std::time::Duration;

const VERSION: &str = env!("CARGO_PKG_VERSION");
const RESET: &str = "\x1b[0m";
const BOLD: &str = "\x1b[1m";
const CYAN: &str = "\x1b[36m";
const YELLOW: &str = "\x1b[33m";

struct FetchData {
    distro: String,
    kernel: String,
    de_wm: String,
    cpu: String,
    gpu: String,
    mem_info: String,
    processes: String,
    packages: String,
}

fn get_distro() -> String {
    if let Ok(content) = fs::read_to_string("/etc/os-release") {
        for line in content.lines() {
            if line.starts_with("PRETTY_NAME=") {
                return line
                    .trim_start_matches("PRETTY_NAME=")
                    .trim_matches('"')
                    .to_string();
            }
        }
    }
    "Linux".to_string()
}

fn get_cpu_model() -> String {
    if let Ok(content) = fs::read_to_string("/proc/cpuinfo") {
        for line in content.lines() {
            if line.starts_with("model name") {
                if let Some(colon_pos) = line.find(':') {
                    let cpu_name = line[colon_pos + 1..].trim().to_string();
                    let cpu_usage = get_cpu_usage();
                    return format!("{} ({}%)", cpu_name, cpu_usage);
                }
            }
        }
    }
    "Unknown CPU".to_string()
}

fn get_cpu_usage() -> String {
    if let Ok(content) = fs::read_to_string("/proc/stat") {
        if let Some(line) = content.lines().next() {
            let values: Vec<u64> = line
                .split_whitespace()
                .skip(1)
                .filter_map(|s| s.parse().ok())
                .collect();

            if values.len() >= 4 {
                let idle = values[3];
                let total: u64 = values.iter().sum();

                thread::sleep(Duration::from_millis(100));

                if let Ok(content2) = fs::read_to_string("/proc/stat") {
                    if let Some(line2) = content2.lines().next() {
                        let values2: Vec<u64> = line2
                            .split_whitespace()
                            .skip(1)
                            .filter_map(|s| s.parse().ok())
                            .collect();

                        if values2.len() >= 4 {
                            let idle2 = values2[3];
                            let total2: u64 = values2.iter().sum();

                            let idle_delta = idle2 - idle;
                            let total_delta = total2 - total;

                            if total_delta > 0 {
                                let usage = 100.0 * (1.0 - idle_delta as f64 / total_delta as f64);
                                return format!("{:.1}", usage);
                            }
                        }
                    }
                }
            }
        }
    }
    "0.0".to_string()
}

fn get_gpu_model() -> String {
    if let Ok(output) = Command::new("sh")
        .arg("-c")
        .arg("lspci | grep -iE 'vga|3d' | cut -d ':' -f3 | sed 's/ (rev .*//' | sed 's/^ //'")
        .output()
    {
        let gpu = String::from_utf8_lossy(&output.stdout).trim().to_string();
        if !gpu.is_empty() {
            let gpu_usage = get_gpu_usage();
            return format!("{} ({}%)", gpu, gpu_usage);
        }
    }
    "Unknown GPU".to_string()
}

fn get_gpu_usage() -> String {
    if let Ok(output) = Command::new("nvidia-smi")
        .arg("--query-gpu=utilization.gpu")
        .arg("--format=csv,noheader,nounits")
        .output()
    {
        if output.status.success() {
            let usage = String::from_utf8_lossy(&output.stdout).trim().to_string();
            if !usage.is_empty() {
                return usage;
            }
        }
    }

    if let Ok(output) = Command::new("sh")
        .arg("-c")
        .arg("radeontop -d - -l 1 2>/dev/null | grep -o 'gpu [0-9]*' | awk '{print $2}'")
        .output()
    {
        if output.status.success() {
            let usage = String::from_utf8_lossy(&output.stdout).trim().to_string();
            if !usage.is_empty() {
                return usage;
            }
        }
    }

    "N/A".to_string()
}

fn get_mem_usage() -> String {
    if let Ok(content) = fs::read_to_string("/proc/meminfo") {
        let mut total = 0u64;
        let mut available = 0u64;

        for line in content.lines() {
            if line.starts_with("MemTotal:") {
                if let Some(value) = line.split_whitespace().nth(1) {
                    total = value.parse().unwrap_or(0);
                }
            } else if line.starts_with("MemAvailable:") {
                if let Some(value) = line.split_whitespace().nth(1) {
                    available = value.parse().unwrap_or(0);
                }
            }
        }

        let used_gb = (total - available) as f64 / 1024.0 / 1024.0;
        let total_gb = total as f64 / 1024.0 / 1024.0;
        return format!("{:.1} GB / {:.1} GB", used_gb, total_gb);
    }
    "N/A".to_string()
}

fn get_process_count() -> String {
    if let Ok(output) = Command::new("sh").arg("-c").arg("ps aux | wc -l").output() {
        let count = String::from_utf8_lossy(&output.stdout).trim().to_string();
        if let Ok(num) = count.parse::<i32>() {
            return (num - 1).to_string();
        }
    }
    "N/A".to_string()
}

fn get_package_count() -> String {
    if let Ok(output) = Command::new("dpkg").arg("-l").output() {
        if output.status.success() {
            let lines = String::from_utf8_lossy(&output.stdout)
                .lines()
                .filter(|line| line.starts_with("ii"))
                .count();
            if lines > 0 {
                return format!("{} (dpkg)", lines);
            }
        }
    }

    if let Ok(output) = Command::new("rpm").arg("-qa").output() {
        if output.status.success() {
            let lines = String::from_utf8_lossy(&output.stdout).lines().count();
            if lines > 0 {
                return format!("{} (rpm)", lines);
            }
        }
    }

    if let Ok(output) = Command::new("pacman").arg("-Q").output() {
        if output.status.success() {
            let lines = String::from_utf8_lossy(&output.stdout).lines().count();
            if lines > 0 {
                return format!("{} (pacman)", lines);
            }
        }
    }

    if let Ok(output) = Command::new("apk").arg("list").arg("--installed").output() {
        if output.status.success() {
            let lines = String::from_utf8_lossy(&output.stdout).lines().count();
            if lines > 0 {
                return format!("{} (apk)", lines);
            }
        }
    }

    "N/A".to_string()
}

fn get_de_wm() -> String {
    env::var("XDG_CURRENT_DESKTOP")
        .or_else(|_| env::var("DESKTOP_SESSION"))
        .unwrap_or_else(|_| "N/A".to_string())
}

fn get_kernel() -> String {
    if let Ok(output) = Command::new("uname").arg("-r").output() {
        return String::from_utf8_lossy(&output.stdout).trim().to_string();
    }
    "Unknown".to_string()
}

fn get_username() -> String {
    env::var("USER").unwrap_or_else(|_| "user".to_string())
}

fn get_hostname() -> String {
    if let Ok(output) = Command::new("hostname").output() {
        return String::from_utf8_lossy(&output.stdout).trim().to_string();
    }
    "hostname".to_string()
}

fn print_colors() {
    print!("{}⠀⠀⠀⠀⠀⠀⠳⠄⣀⣀⠤⠊⠀  ", CYAN);
    for i in 0..8 {
        print!("\x1b[4{}m  ", i);
    }
    println!("{}", RESET);
}

fn clear_screen() {
    print!("\x1b[2J\x1b[H");
}

fn print_fetch(data: &FetchData, username: &str, hostname: &str) {
    println!("{}⠀⠀⠀⢀⡤⣾⠉⠑⡄⠀⠀⠀⠀  ", CYAN);
    println!(
        "{}⠀⢀⣔⠙⡄⠀⠈⡆⠀⢀⠀⠀⠀⠀  {}{}{}{}{}@{}{}{}{}",
        CYAN, RESET, BOLD, CYAN, username, RESET, BOLD, CYAN, hostname, RESET
    );
    println!(
        "{}⣀⣌⠈⡆⣗⣚⠯⠚⠘⢆⠀⠀⠀  {}─────────────────────────────",
        CYAN, RESET
    );
    println!(
        "{}⠘⡺⠁⠀⠀⢸⠀⠀⠀⠀⢸⠀⠀  {}{}{}OS:        {}{}",
        CYAN, RESET, BOLD, YELLOW, RESET, data.distro
    );
    println!(
        "{}⢸⠀⠀⠀⠀⢄⠀⠀⠀⠀⡎⠀⠀  {}{}{}Kernel:    {}{}",
        CYAN, RESET, BOLD, YELLOW, RESET, data.kernel
    );
    println!(
        "{}⠈⡄⠀⠀⠀⠘⠄⠀⢀⡜⠀⠀⠀  {}{}{}DE/WM:     {}{}",
        CYAN, RESET, BOLD, YELLOW, RESET, data.de_wm
    );
    println!(
        "{}⠀⠘⡄⠀⠀⠀⠈⠠⠎⡇⠀⠀⠀  {}{}{}Packages:  {}{}",
        CYAN, RESET, BOLD, YELLOW, RESET, data.packages
    );
    println!(
        "{}⠀⠀⠘⡄⠀⠀⠀⠀⠀⠇⠀⠀⠀  {}{}{}Processes: {}{}",
        CYAN, RESET, BOLD, YELLOW, RESET, data.processes
    );
    println!(
        "{}⠀⠀⠀⠘⡀⠀⠀⠀⠀⠘⡄⠀⠀  {}{}{}CPU:       {}{}",
        CYAN, RESET, BOLD, YELLOW, RESET, data.cpu
    );
    println!(
        "{}⠀⠀⠀⠀⢡⠀⠀⠀⠀⠀⠈⡄⠀  {}{}{}GPU:       {}{}",
        CYAN, RESET, BOLD, YELLOW, RESET, data.gpu
    );
    println!(
        "{}⠀⠀⠀⠀⠈⡄⠀⠀⠀⠀⠀⠸⠀  {}{}{}Memory:    {}{}",
        CYAN, RESET, BOLD, YELLOW, RESET, data.mem_info
    );
    println!("{}⠀⠀⠀⠀⠀⢣⠀⠀⠀⠀⠀⢀⠆  ", CYAN);
    print_colors();
}

fn print_version() {
    println!(
        "{}{}Footfetch{} version {}{}{}",
        BOLD, CYAN, RESET, BOLD, VERSION, RESET
    );
    println!("A lightweight and high-performance, neofetch-like tool for those who prefer feet over faces written in Rust.");
}

fn print_help() {
    println!("A lightweight and high-performance, neofetch-like tool for those who prefer feet over faces written in Rust.");
    println!("\n{}Usage:{}", BOLD, RESET);
    println!("  footfetch [OPTIONS]");
    println!("\n{}Options:{}", BOLD, RESET);
    println!("  -v, --version    Show version information");
    println!("  --live           Live mode (updates every 2 seconds)");
    println!("  -h, --help       Show this help message");
}

fn live_mode() {
    let username = get_username();
    let hostname = get_hostname();

    loop {
        let data = FetchData {
            distro: get_distro(),
            kernel: get_kernel(),
            de_wm: get_de_wm(),
            cpu: get_cpu_model(),
            gpu: get_gpu_model(),
            mem_info: get_mem_usage(),
            processes: get_process_count(),
            packages: get_package_count(),
        };

        clear_screen();
        print_fetch(&data, &username, &hostname);

        thread::sleep(Duration::from_secs(2));
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() > 1 {
        match args[1].as_str() {
            "-v" | "--version" => {
                print_version();
                return;
            }
            "--live" => {
                live_mode();
                return;
            }
            "-h" | "--help" => {
                print_help();
                return;
            }
            _ => {
                eprintln!("{}Unknown option: {}{}", BOLD, args[1], RESET);
                eprintln!("Use --help for usage information");
                std::process::exit(1);
            }
        }
    }

    let data = FetchData {
        distro: get_distro(),
        kernel: get_kernel(),
        de_wm: get_de_wm(),
        cpu: get_cpu_model(),
        gpu: get_gpu_model(),
        mem_info: get_mem_usage(),
        processes: get_process_count(),
        packages: get_package_count(),
    };

    let username = get_username();
    let hostname = get_hostname();

    print_fetch(&data, &username, &hostname);
}
