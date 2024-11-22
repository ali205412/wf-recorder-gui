use std::process::Command;
use anyhow::{Result, Context};

pub fn get_audio_devices() -> Result<Vec<AudioDevice>> {
    let output = Command::new("pactl")
        .args(["list", "sources"])
        .output()
        .context("Failed to execute pactl")?;

    let output = String::from_utf8_lossy(&output.stdout);
    parse_audio_devices(&output)
}

fn parse_audio_devices(output: &str) -> Result<Vec<AudioDevice>> {
    let mut devices = Vec::new();
    let mut current_device: Option<AudioDevice> = None;

    for line in output.lines() {
        if line.starts_with("Source #") {
            // Save previous device if exists
            if let Some(device) = current_device.take() {
                devices.push(device);
            }
            // Start new device
            current_device = Some(AudioDevice {
                name: String::new(),
                description: String::new(),
            });
        } else if let Some(device) = &mut current_device {
            if line.trim().starts_with("Name: ") {
                device.name = line.trim()["Name: ".len()..].to_string();
            } else if line.trim().starts_with("Description: ") {
                device.description = line.trim()["Description: ".len()..].to_string();
            }
        }
    }

    // Add last device
    if let Some(device) = current_device {
        devices.push(device);
    }

    Ok(devices)
}

#[derive(Debug, Clone)]
pub struct AudioDevice {
    pub name: String,
    pub description: String,
}
