use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum MuxPref {
    Tmux,
    Zellij,
}

impl Default for MuxPref {
    fn default() -> Self {
        MuxPref::Tmux
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppConfig {
    #[serde(default)]
    pub mux: MuxPref,
}

impl Default for AppConfig {
    fn default() -> Self {
        AppConfig { mux: MuxPref::default() }
    }
}

fn config_path() -> PathBuf {
    let home = std::env::var("HOME").unwrap_or_default();
    PathBuf::from(format!("{}/.config/keybind-hud/config.json", home))
}

pub fn load() -> AppConfig {
    let path = config_path();
    if let Ok(s) = std::fs::read_to_string(&path) {
        serde_json::from_str(&s).unwrap_or_default()
    } else {
        AppConfig::default()
    }
}

pub fn save(cfg: &AppConfig) {
    let path = config_path();
    if let Some(parent) = path.parent() {
        let _ = std::fs::create_dir_all(parent);
    }
    if let Ok(s) = serde_json::to_string_pretty(cfg) {
        let _ = std::fs::write(path, s);
    }
}
