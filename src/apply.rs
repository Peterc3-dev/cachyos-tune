use anyhow::{bail, Result};
use std::fs;

use crate::profiles::Profile;
use crate::style;
use crate::sysfs;

/// Apply a full profile to the system. Requires root.
pub fn apply_profile(profile: &Profile) -> Result<()> {
    // Check if we can write (rough sudo check)
    if !can_write_sysctl() {
        bail!(
            "Permission denied. Run with sudo:\n  sudo cachyos-tune apply {}",
            profile.name
        );
    }

    let mut errors: Vec<String> = Vec::new();

    // CPU governor
    match set_cpu_governor(&profile.cpu_governor) {
        Ok(count) => style::print_applied("cpu_governor", &profile.cpu_governor, &format!("{} cores", count)),
        Err(e) => errors.push(format!("cpu_governor: {}", e)),
    }

    // VM sysctls
    for (key, value) in [
        ("vm.swappiness", profile.swappiness.to_string()),
        ("vm.vfs_cache_pressure", profile.vfs_cache_pressure.to_string()),
        ("vm.dirty_ratio", profile.dirty_ratio.to_string()),
        ("vm.dirty_background_ratio", profile.dirty_background_ratio.to_string()),
    ] {
        match sysfs::write_sysctl(key, &value) {
            Ok(()) => style::print_applied(key, &value, ""),
            Err(e) => errors.push(format!("{}: {}", key, e)),
        }
    }

    // Optional dirty page timing controls
    if let Some(expire) = profile.dirty_expire_centisecs {
        let value = expire.to_string();
        match sysfs::write_sysctl("vm.dirty_expire_centisecs", &value) {
            Ok(()) => style::print_applied("vm.dirty_expire_centisecs", &value, ""),
            Err(e) => errors.push(format!("vm.dirty_expire_centisecs: {}", e)),
        }
    }
    if let Some(writeback) = profile.dirty_writeback_centisecs {
        let value = writeback.to_string();
        match sysfs::write_sysctl("vm.dirty_writeback_centisecs", &value) {
            Ok(()) => style::print_applied("vm.dirty_writeback_centisecs", &value, ""),
            Err(e) => errors.push(format!("vm.dirty_writeback_centisecs: {}", e)),
        }
    }

    // Transparent hugepages
    match sysfs::write_sysfs("/sys/kernel/mm/transparent_hugepage/enabled", &profile.transparent_hugepages) {
        Ok(()) => style::print_applied("transparent_hugepages", &profile.transparent_hugepages, ""),
        Err(e) => errors.push(format!("transparent_hugepages: {}", e)),
    }

    // ZRAM compression algorithm
    if is_zram_active_swap("zram0") {
        style::print_warn("zram_comp_algorithm: cannot change while zram0 is active as swap");
        style::print_warn("  To change: swapoff /dev/zram0, set algorithm, then swapon /dev/zram0");
    } else {
        match sysfs::write_sysfs("/sys/block/zram0/comp_algorithm", &profile.zram_comp_algorithm) {
            Ok(()) => style::print_applied("zram_comp_algorithm", &profile.zram_comp_algorithm, ""),
            Err(e) => errors.push(format!("zram_comp_algorithm: {}", e)),
        }
    }

    // GPU power profile
    match set_gpu_power_profile(&profile.gpu_power_profile) {
        Ok(()) => style::print_applied("gpu_power_profile", &profile.gpu_power_profile, ""),
        Err(e) => errors.push(format!("gpu_power_profile: {}", e)),
    }

    // IO scheduler
    match set_io_scheduler(&profile.io_scheduler) {
        Ok(count) => style::print_applied("io_scheduler", &profile.io_scheduler, &format!("{} devices", count)),
        Err(e) => errors.push(format!("io_scheduler: {}", e)),
    }

    // TCP congestion
    match sysfs::write_sysctl("net.ipv4.tcp_congestion_control", &profile.tcp_congestion) {
        Ok(()) => style::print_applied("tcp_congestion", &profile.tcp_congestion, ""),
        Err(e) => errors.push(format!("tcp_congestion: {}", e)),
    }

    if !errors.is_empty() {
        eprintln!();
        style::print_warn("Some settings could not be applied:");
        for e in &errors {
            style::print_warn(&format!("  {}", e));
        }
    }

    Ok(())
}

/// Apply settings from a snapshot (for restore).
pub fn apply_snapshot(snap: &crate::snapshot::Snapshot) -> Result<()> {
    if !can_write_sysctl() {
        bail!("Permission denied. Run with sudo:\n  sudo cachyos-tune restore");
    }

    let mut errors: Vec<String> = Vec::new();

    match set_cpu_governor(&snap.cpu_governor) {
        Ok(count) => style::print_applied("cpu_governor", &snap.cpu_governor, &format!("{} cores", count)),
        Err(e) => errors.push(format!("cpu_governor: {}", e)),
    }

    for (key, value) in [
        ("vm.swappiness", &snap.swappiness),
        ("vm.vfs_cache_pressure", &snap.vfs_cache_pressure),
        ("vm.dirty_ratio", &snap.dirty_ratio),
        ("vm.dirty_background_ratio", &snap.dirty_background_ratio),
        ("vm.dirty_expire_centisecs", &snap.dirty_expire_centisecs),
        ("vm.dirty_writeback_centisecs", &snap.dirty_writeback_centisecs),
    ] {
        match sysfs::write_sysctl(key, value) {
            Ok(()) => style::print_applied(key, value, ""),
            Err(e) => errors.push(format!("{}: {}", key, e)),
        }
    }

    match sysfs::write_sysfs("/sys/kernel/mm/transparent_hugepage/enabled", &snap.transparent_hugepages) {
        Ok(()) => style::print_applied("transparent_hugepages", &snap.transparent_hugepages, ""),
        Err(e) => errors.push(format!("transparent_hugepages: {}", e)),
    }

    if is_zram_active_swap("zram0") {
        style::print_warn("zram_comp_algorithm: cannot change while zram0 is active as swap");
        style::print_warn("  To change: swapoff /dev/zram0, set algorithm, then swapon /dev/zram0");
    } else {
        match sysfs::write_sysfs("/sys/block/zram0/comp_algorithm", &snap.zram_comp_algorithm) {
            Ok(()) => style::print_applied("zram_comp_algorithm", &snap.zram_comp_algorithm, ""),
            Err(e) => errors.push(format!("zram_comp_algorithm: {}", e)),
        }
    }

    match set_gpu_power_profile(&snap.gpu_power_profile) {
        Ok(()) => style::print_applied("gpu_power_profile", &snap.gpu_power_profile, ""),
        Err(e) => errors.push(format!("gpu_power_profile: {}", e)),
    }

    match set_io_scheduler(&snap.io_scheduler) {
        Ok(count) => style::print_applied("io_scheduler", &snap.io_scheduler, &format!("{} devices", count)),
        Err(e) => errors.push(format!("io_scheduler: {}", e)),
    }

    match sysfs::write_sysctl("net.ipv4.tcp_congestion_control", &snap.tcp_congestion) {
        Ok(()) => style::print_applied("tcp_congestion", &snap.tcp_congestion, ""),
        Err(e) => errors.push(format!("tcp_congestion: {}", e)),
    }

    if !errors.is_empty() {
        eprintln!();
        style::print_warn("Some settings could not be restored:");
        for e in &errors {
            style::print_warn(&format!("  {}", e));
        }
    }

    Ok(())
}

fn can_write_sysctl() -> bool {
    unsafe { libc::geteuid() == 0 }
}

fn set_cpu_governor(governor: &str) -> Result<usize> {
    let paths = glob_cpu_governors();
    if paths.is_empty() {
        bail!("No CPU governor paths found");
    }
    let mut count = 0;
    for path in &paths {
        sysfs::write_sysfs(path, governor)?;
        count += 1;
    }
    Ok(count)
}

fn glob_cpu_governors() -> Vec<String> {
    let mut results = Vec::new();
    // Enumerate cpuN directories
    if let Ok(entries) = fs::read_dir("/sys/devices/system/cpu") {
        for entry in entries.flatten() {
            let name = entry.file_name().to_string_lossy().to_string();
            if name.starts_with("cpu") && name[3..].chars().all(|c| c.is_ascii_digit()) {
                let path = format!(
                    "/sys/devices/system/cpu/{}/cpufreq/scaling_governor",
                    name
                );
                if std::path::Path::new(&path).exists() {
                    results.push(path);
                }
            }
        }
    }
    results.sort();
    results
}

fn set_gpu_power_profile(level: &str) -> Result<()> {
    for card in &["card0", "card1"] {
        let path = format!(
            "/sys/class/drm/{}/device/power_dpm_force_performance_level",
            card
        );
        if std::path::Path::new(&path).exists() {
            sysfs::write_sysfs(&path, level)?;
            return Ok(());
        }
    }
    bail!("No GPU DPM control found");
}

/// Check if a zram device is currently in use as swap by reading /proc/swaps.
fn is_zram_active_swap(device: &str) -> bool {
    if let Ok(swaps) = fs::read_to_string("/proc/swaps") {
        let needle = format!("/dev/{}", device);
        swaps.lines().any(|line| line.starts_with(&needle))
    } else {
        false
    }
}

fn set_io_scheduler(scheduler: &str) -> Result<usize> {
    let mut count = 0;
    if let Ok(entries) = fs::read_dir("/sys/block") {
        for entry in entries.flatten() {
            let name = entry.file_name().to_string_lossy().to_string();
            if name.starts_with("nvme") || name.starts_with("sd") {
                let path = format!("/sys/block/{}/queue/scheduler", name);
                if std::path::Path::new(&path).exists() {
                    sysfs::write_sysfs(&path, scheduler)?;
                    count += 1;
                }
            }
        }
    }
    if count == 0 {
        bail!("No block device scheduler paths found");
    }
    Ok(count)
}
