use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, clap::ValueEnum)]
#[serde(rename_all = "kebab-case")]
pub enum ProfileName {
    Default,
    MlInference,
    MlTraining,
    Gaming,
    Battery,
    Compile,
}

impl fmt::Display for ProfileName {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ProfileName::Default => write!(f, "default"),
            ProfileName::MlInference => write!(f, "ml-inference"),
            ProfileName::MlTraining => write!(f, "ml-training"),
            ProfileName::Gaming => write!(f, "gaming"),
            ProfileName::Battery => write!(f, "battery"),
            ProfileName::Compile => write!(f, "compile"),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Profile {
    pub name: ProfileName,
    pub cpu_governor: String,
    pub swappiness: u32,
    pub vfs_cache_pressure: u32,
    pub dirty_ratio: u32,
    pub dirty_background_ratio: u32,
    pub dirty_expire_centisecs: Option<u32>,
    pub dirty_writeback_centisecs: Option<u32>,
    pub transparent_hugepages: String,
    pub zram_comp_algorithm: String,
    pub gpu_power_profile: String,
    pub io_scheduler: String,
    pub tcp_congestion: String,
}

impl Profile {
    pub fn default_profile() -> Self {
        Profile {
            name: ProfileName::Default,
            cpu_governor: "schedutil".into(),
            swappiness: 60,
            vfs_cache_pressure: 100,
            dirty_ratio: 20,
            dirty_background_ratio: 10,
            dirty_expire_centisecs: None,
            dirty_writeback_centisecs: None,
            transparent_hugepages: "madvise".into(),
            zram_comp_algorithm: "zstd".into(),
            gpu_power_profile: "auto".into(),
            io_scheduler: "none".into(),
            tcp_congestion: "cubic".into(),
        }
    }

    pub fn ml_inference() -> Self {
        Profile {
            name: ProfileName::MlInference,
            cpu_governor: "performance".into(),
            swappiness: 30,
            vfs_cache_pressure: 50,
            dirty_ratio: 10,
            dirty_background_ratio: 5,
            dirty_expire_centisecs: Some(3000),
            dirty_writeback_centisecs: Some(500),
            transparent_hugepages: "always".into(),
            zram_comp_algorithm: "lz4".into(),
            gpu_power_profile: "high".into(),
            io_scheduler: "none".into(),
            tcp_congestion: "bbr".into(),
        }
    }

    pub fn gaming() -> Self {
        Profile {
            name: ProfileName::Gaming,
            cpu_governor: "performance".into(),
            swappiness: 10,
            vfs_cache_pressure: 50,
            dirty_ratio: 20,
            dirty_background_ratio: 10,
            dirty_expire_centisecs: None,
            dirty_writeback_centisecs: None,
            transparent_hugepages: "never".into(),
            zram_comp_algorithm: "lz4".into(),
            gpu_power_profile: "high".into(),
            io_scheduler: "none".into(),
            tcp_congestion: "bbr".into(),
        }
    }

    pub fn battery() -> Self {
        Profile {
            name: ProfileName::Battery,
            cpu_governor: "powersave".into(),
            swappiness: 80,
            vfs_cache_pressure: 100,
            dirty_ratio: 10,
            dirty_background_ratio: 5,
            dirty_expire_centisecs: None,
            dirty_writeback_centisecs: None,
            transparent_hugepages: "madvise".into(),
            zram_comp_algorithm: "zstd".into(),
            gpu_power_profile: "low".into(),
            io_scheduler: "none".into(),
            tcp_congestion: "cubic".into(),
        }
    }

    pub fn compile() -> Self {
        Profile {
            name: ProfileName::Compile,
            cpu_governor: "performance".into(),
            swappiness: 30,
            vfs_cache_pressure: 50,
            dirty_ratio: 15,
            dirty_background_ratio: 10,
            dirty_expire_centisecs: None,
            dirty_writeback_centisecs: None,
            transparent_hugepages: "always".into(),
            zram_comp_algorithm: "lz4".into(),
            gpu_power_profile: "auto".into(),
            io_scheduler: "none".into(),
            tcp_congestion: "cubic".into(),
        }
    }

    pub fn ml_training() -> Self {
        Profile {
            name: ProfileName::MlTraining,
            cpu_governor: "performance".into(),
            swappiness: 30,
            vfs_cache_pressure: 50,
            dirty_ratio: 15,
            dirty_background_ratio: 5,
            dirty_expire_centisecs: Some(3000),
            dirty_writeback_centisecs: Some(500),
            transparent_hugepages: "always".into(),
            zram_comp_algorithm: "lz4".into(),
            gpu_power_profile: "high".into(),
            io_scheduler: "none".into(),
            tcp_congestion: "bbr".into(),
        }
    }

    pub fn from_name(name: ProfileName) -> Self {
        match name {
            ProfileName::Default => Self::default_profile(),
            ProfileName::MlInference => Self::ml_inference(),
            ProfileName::MlTraining => Self::ml_training(),
            ProfileName::Gaming => Self::gaming(),
            ProfileName::Battery => Self::battery(),
            ProfileName::Compile => Self::compile(),
        }
    }

    pub fn all_profiles() -> Vec<Self> {
        vec![
            Self::default_profile(),
            Self::ml_inference(),
            Self::ml_training(),
            Self::gaming(),
            Self::battery(),
            Self::compile(),
        ]
    }
}
