use std::env;
use std::fs;
use std::process::Command;

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
                    return line[colon_pos + 1..].trim().to_string();
                }
            }
        }
    }
    "Unknown CPU".to_string()
}

fn get_gpu_model() -> String {
    if let Ok(output) = Command::new("sh")
        .arg("-c")
        .arg("lspci | grep -iE 'vga|3d' | cut -d ':' -f3 | sed 's/ (rev .*//' | sed 's/^ //'")
        .output()
    {
        let gpu = String::from_utf8_lossy(&output.stdout).trim().to_string();
        if !gpu.is_empty() {
            return gpu;
        }
    }
    "Unknown GPU".to_string()
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
    print!("{}⠀⠀⠀⠀⠈⡄⠀⠀⠀⠀⠀⠸⠀  ", CYAN);
    for i in 0..8 {
        print!("\x1b[4{}m  ", i);
    }
    println!("{}", RESET);
}

fn main() {
    let data = FetchData {
        distro: get_distro(),
        kernel: get_kernel(),
        de_wm: get_de_wm(),
        cpu: get_cpu_model(),
        gpu: get_gpu_model(),
        mem_info: get_mem_usage(),
    };

    let username = get_username();
    let hostname = get_hostname();

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
        "{}⠘⡺⠁⠀⠀⢸⠀⠀⠀⠀⢸⠀⠀  {}{}{}OS:      {}{}",
        CYAN, RESET, BOLD, YELLOW, RESET, data.distro
    );
    println!(
        "{}⢸⠀⠀⠀⠀⢄⠀⠀⠀⠀⡎⠀⠀  {}{}{}Kernel:  {}{}",
        CYAN, RESET, BOLD, YELLOW, RESET, data.kernel
    );
    println!(
        "{}⠈⡄⠀⠀⠀⠘⠄⠀⢀⡜⠀⠀⠀  {}{}{}DE/WM:   {}{}",
        CYAN, RESET, BOLD, YELLOW, RESET, data.de_wm
    );
    println!(
        "{}⠀⠘⡄⠀⠀⠀⠈⠠⠎⡇⠀⠀⠀  {}{}{}CPU:     {}{}",
        CYAN, RESET, BOLD, YELLOW, RESET, data.cpu
    );
    println!(
        "{}⠀⠀⠘⡄⠀⠀⠀⠀⠀⠇⠀⠀⠀  {}{}{}GPU:     {}{}",
        CYAN, RESET, BOLD, YELLOW, RESET, data.gpu
    );
    println!(
        "{}⠀⠀⠀⠘⡀⠀⠀⠀⠀⠘⡄⠀⠀  {}{}{}Memory:  {}{}",
        CYAN, RESET, BOLD, YELLOW, RESET, data.mem_info
    );

    println!("{}⠀⠀⠀⠀⢡⠀⠀⠀⠀⠀⠈⡄⠀  ", CYAN);
    print_colors();
    println!("{}⠀⠀⠀⠀⠀⢣⠀⠀⠀⠀⠀⢀⠆  ", CYAN);
    println!("{}⠀⠀⠀⠀⠀⠀⠳⠄⣀⣀⠤⠊⠀  ", CYAN);
}