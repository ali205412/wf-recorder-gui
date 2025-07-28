use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

use crate::recorder::{AudioSource, CaptureRegion, OutputFormat};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    pub output_dir: PathBuf,
    pub format: OutputFormat,
    pub audio: AudioSource,
    pub region: CaptureRegion,
}

impl Default for Config {
    fn default() -> Self {
        // Try to use ~/Videos/Screencasts as default, fallback to temp
        let default_dir = dirs::home_dir()
            .map(|h| h.join("Videos/Screencasts"))
            .unwrap_or_else(|| std::env::temp_dir());
        
        Self {
            output_dir: default_dir,
            format: OutputFormat::Mp4,
            audio: AudioSource::None,
            region: CaptureRegion::FullScreen,
        }
    }
}

impl Config {
    pub fn config_dir() -> Result<PathBuf> {
        let config_dir = dirs::config_dir()
            .ok_or_else(|| anyhow::anyhow!("Could not find config directory"))?
            .join("wf-recorder-gui");
        
        if !config_dir.exists() {
            fs::create_dir_all(&config_dir)?;
        }
        
        Ok(config_dir)
    }
    
    pub fn config_path() -> Result<PathBuf> {
        Ok(Self::config_dir()?.join("config.json"))
    }
    
    pub fn load() -> Result<Self> {
        let path = Self::config_path()?;
        
        if path.exists() {
            let content = fs::read_to_string(&path)?;
            let mut config: Config = serde_json::from_str(&content)?;
            
            // Ensure the output directory exists and create ~/Videos/Screencasts if needed
            if !config.output_dir.exists() {
                // If saved dir doesn't exist, try to create it or use default
                if config.output_dir.to_string_lossy().contains("Videos/Screencasts") {
                    fs::create_dir_all(&config.output_dir).ok();
                }
                
                // If it still doesn't exist after creation attempt, use default
                if !config.output_dir.exists() {
                    config.output_dir = Self::default().output_dir;
                }
            }
            
            Ok(config)
        } else {
            let config = Self::default();
            // Try to create the default Videos/Screencasts directory
            if config.output_dir.to_string_lossy().contains("Videos/Screencasts") {
                fs::create_dir_all(&config.output_dir).ok();
            }
            Ok(config)
        }
    }
    
    pub fn save(&self) -> Result<()> {
        let path = Self::config_path()?;
        let content = serde_json::to_string_pretty(self)?;
        fs::write(&path, content)?;
        Ok(())
    }
}
