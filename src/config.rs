use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use std::fs;
use dirs::home_dir;

#[derive(Serialize, Deserialize, Clone)]
pub struct Config {
    pub delay_ms: u64,
    pub cooldown_ms: u64,
    pub enabled: bool,
    pub blacklist: Vec<BlacklistEntry>,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct BlacklistEntry {
    pub window_title: String,
    pub class_name: String,
    pub process_name: String,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            delay_ms: 50,
            cooldown_ms: 300,
            enabled: true,
            blacklist: vec![
                BlacklistEntry {
                    window_title: "Task Manager".to_string(),
                    class_name: "".to_string(),
                    process_name: "Taskmgr.exe".to_string(),
                },
            ],
        }
    }
}

impl Config {
    pub fn load() -> Self {
        let config_path = get_config_path();
        if let Ok(contents) = fs::read_to_string(config_path) {
            serde_json::from_str(&contents).unwrap_or_default()
        } else {
            let config = Config::default();
            config.save();
            config
        }
    }

    pub fn save(&self) {
        if let Ok(serialized) = serde_json::to_string_pretty(self) {
            let config_path = get_config_path();
            if let Some(parent) = config_path.parent() {
                let _ = fs::create_dir_all(parent);
            }
            let _ = fs::write(config_path, serialized);
        }
    }
}

fn get_config_path() -> PathBuf {
    home_dir()
        .unwrap_or_else(|| PathBuf::from("."))
        .join(".config")
        .join("whma")
        .join("config.json")
}