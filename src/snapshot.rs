use anyhow::{Context, Result};
use chrono::Local;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

use crate::sysfs;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Snapshot {
    pub timestamp: String,
    pub cpu_governor: String,
    pub swappiness: String,
    pub vfs_cache_pressure: String,
    pub dirty_ratio: String,
    pub dirty_background_ratio: String,
    pub dirty_expire_centisecs: String,
    pub dirty_writeback_centisecs: String,
    pub transparent_hugepages: String,
    pub zram_comp_algorithm: String,
    pub gpu_power_profile: String,
    pub io_scheduler: String,
    pub tcp_congestion: String,
}

impl Snapshot {
    pub fn capture() -> Self {
        Snapshot {
            timestamp: Local::now().format("%Y-%m-%d %H:%M:%S").to_string(),
            cpu_governor: sysfs::read_cpu_governor(),
            swappiness: sysfs::read_sysctl_or("vm.swappiness", "?"),
            vfs_cache_pressure: sysfs::read_sysctl_or("vm.vfs_cache_pressure", "?"),
            dirty_ratio: sysfs::read_sysctl_or("vm.dirty_ratio", "?"),
            dirty_background_ratio: sysfs::read_sysctl_or("vm.dirty_background_ratio", "?"),
            dirty_expire_centisecs: sysfs::read_sysctl_or("vm.dirty_expire_centisecs", "?"),
            dirty_writeback_centisecs: sysfs::read_sysctl_or("vm.dirty_writeback_centisecs", "?"),
            transparent_hugepages: sysfs::read_thp(),
            zram_comp_algorithm: sysfs::read_zram_algorithm(),
            gpu_power_profile: sysfs::read_gpu_power_profile(),
            io_scheduler: sysfs::read_io_scheduler(),
            tcp_congestion: sysfs::read_tcp_congestion(),
        }
    }
}

fn snapshot_path() -> PathBuf {
    let dir = dirs_or_default();
    dir.join("snapshot.json")
}

fn dirs_or_default() -> PathBuf {
    // When running under sudo, dirs::config_dir() returns /root/.config,
    // but snapshots are saved as the normal user. Use SUDO_USER to find
    // the original user's config directory.
    let config_dir = if let Ok(sudo_user) = std::env::var("SUDO_USER") {
        PathBuf::from(format!("/home/{}", sudo_user)).join(".config")
    } else {
        dirs::config_dir().unwrap_or_else(|| PathBuf::from("/tmp"))
    };

    let p = config_dir.join("cachyos-tune");
    let _ = fs::create_dir_all(&p);
    p
}

pub fn save_snapshot() -> Result<PathBuf> {
    let snap = Snapshot::capture();
    let path = snapshot_path();
    let json = serde_json::to_string_pretty(&snap)?;
    fs::write(&path, json).with_context(|| format!("Failed to write snapshot to {:?}", path))?;
    Ok(path)
}

pub fn load_snapshot() -> Result<Snapshot> {
    let path = snapshot_path();
    let json = fs::read_to_string(&path)
        .with_context(|| format!("No snapshot found at {:?}", path))?;
    let snap: Snapshot = serde_json::from_str(&json)?;
    Ok(snap)
}
