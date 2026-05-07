use anyhow::{Context, Result};
use std::fs;

/// Read a sysfs/procfs value, trimming whitespace.
pub fn read_sysfs(path: &str) -> Result<String> {
    fs::read_to_string(path)
        .with_context(|| format!("Failed to read {}", path))
        .map(|s| s.trim().to_string())
}

/// Read a sysfs value, returning a default if the file doesn't exist or isn't readable.
pub fn read_sysfs_or(path: &str, default: &str) -> String {
    read_sysfs(path).unwrap_or_else(|_| default.to_string())
}

/// Write a value to a sysfs/procfs path.
pub fn write_sysfs(path: &str, value: &str) -> Result<()> {
    fs::write(path, value).with_context(|| format!("Failed to write '{}' to {}", value, path))
}

/// Write a sysctl value via /proc/sys.
pub fn write_sysctl(key: &str, value: &str) -> Result<()> {
    let path = format!("/proc/sys/{}", key.replace('.', "/"));
    write_sysfs(&path, value)
}

/// Read a sysctl value via /proc/sys.
pub fn read_sysctl(key: &str) -> Result<String> {
    let path = format!("/proc/sys/{}", key.replace('.', "/"));
    read_sysfs(&path)
}

/// Read a sysctl value, returning a default if unreadable.
pub fn read_sysctl_or(key: &str, default: &str) -> String {
    read_sysctl(key).unwrap_or_else(|_| default.to_string())
}



/// Read the current CPU governor (from cpu0).
pub fn read_cpu_governor() -> String {
    read_sysfs_or(
        "/sys/devices/system/cpu/cpu0/cpufreq/scaling_governor",
        "unknown",
    )
}

/// Read transparent hugepages setting, extracting the active value in brackets.
pub fn read_thp() -> String {
    let raw = read_sysfs_or("/sys/kernel/mm/transparent_hugepage/enabled", "unknown");
    // Format is: "always [madvise] never" — extract bracketed value
    if let Some(start) = raw.find('[') {
        if let Some(end) = raw.find(']') {
            return raw[start + 1..end].to_string();
        }
    }
    raw
}

/// Read ZRAM compression algorithm, extracting the active one in brackets.
pub fn read_zram_algorithm() -> String {
    let raw = read_sysfs_or("/sys/block/zram0/comp_algorithm", "unknown");
    if let Some(start) = raw.find('[') {
        if let Some(end) = raw.find(']') {
            return raw[start + 1..end].to_string();
        }
    }
    raw
}

/// Read GPU power profile mode.
pub fn read_gpu_power_profile() -> String {
    // Try card0 first, then card1
    for card in &["card0", "card1"] {
        let path = format!("/sys/class/drm/{}/device/power_dpm_force_performance_level", card);
        if let Ok(val) = read_sysfs(&path) {
            return val;
        }
    }
    "unknown".to_string()
}

/// Read IO scheduler for the first nvme device.
pub fn read_io_scheduler() -> String {
    let raw = read_sysfs_or("/sys/block/nvme0n1/queue/scheduler", "unknown");
    // Format: "[none] mq-deadline kyber bfq" — extract bracketed
    if let Some(start) = raw.find('[') {
        if let Some(end) = raw.find(']') {
            return raw[start + 1..end].to_string();
        }
    }
    raw
}

/// Read TCP congestion control algorithm.
pub fn read_tcp_congestion() -> String {
    read_sysctl_or("net.ipv4.tcp_congestion_control", "unknown")
}
