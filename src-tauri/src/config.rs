// Configuration management
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppConfig {
    pub hotkey: String,
    pub default_style: String,
    pub model_provider: String, // openai, claude, gemini, perplexity
    pub api_key: Option<String>,
    pub theme: String,
    pub start_on_login: bool,
    pub auto_update: bool,
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            hotkey: "CommandOrControl+Shift+R".to_string(),
            default_style: "professional".to_string(),
            model_provider: "proxy".to_string(), // Default to proxy server (uses your API key)
            api_key: None, // Optional: user can provide their own API key
            theme: "system".to_string(),
            start_on_login: false,
            auto_update: true,
        }
    }
}

fn get_config_path() -> Result<PathBuf, Box<dyn std::error::Error>> {
    let home = std::env::var("HOME")
        .or_else(|_| std::env::var("USERPROFILE"))?;
    
    let config_dir = if cfg!(target_os = "macos") {
        PathBuf::from(home).join("Library/Application Support/Rephraser")
    } else {
        PathBuf::from(home).join("AppData/Roaming/Rephraser")
    };
    
    fs::create_dir_all(&config_dir)?;
    
    Ok(config_dir.join("config.json"))
}

pub fn load() -> Result<AppConfig, Box<dyn std::error::Error>> {
    let config_path = get_config_path()?;
    
    if !config_path.exists() {
        return Ok(AppConfig::default());
    }
    
    let content = fs::read_to_string(config_path)?;
    let config: AppConfig = serde_json::from_str(&content)?;
    
    Ok(config)
}

pub fn save(config: &AppConfig) -> Result<(), Box<dyn std::error::Error>> {
    let config_path = get_config_path()?;
    let content = serde_json::to_string_pretty(config)?;
    fs::write(config_path, content)?;
    
    Ok(())
}


