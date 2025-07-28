use std::path::PathBuf;

mod config;
mod recorder;

use config::Config;
use recorder::{AudioSource, CaptureRegion, OutputFormat};

fn main() {
    println!("Testing config persistence...\n");
    
    // Test 1: Load default config
    let config = Config::load().unwrap_or_default();
    println!("Loaded config:");
    println!("  Output dir: {:?}", config.output_dir);
    println!("  Format: {:?}", config.format);
    println!("  Audio: {:?}", config.audio);
    println!("  Region: {:?}", config.region);
    
    // Test 2: Modify and save config
    let mut test_config = config.clone();
    test_config.output_dir = PathBuf::from("/home/user/Videos/Screencasts");
    test_config.format = OutputFormat::WebM;
    test_config.audio = AudioSource::System;
    test_config.region = CaptureRegion::Selection;
    
    println!("\nSaving modified config...");
    test_config.save().expect("Failed to save config");
    
    // Test 3: Load again to verify persistence
    let loaded_config = Config::load().expect("Failed to load config");
    println!("\nReloaded config:");
    println!("  Output dir: {:?}", loaded_config.output_dir);
    println!("  Format: {:?}", loaded_config.format);
    println!("  Audio: {:?}", loaded_config.audio);
    println!("  Region: {:?}", loaded_config.region);
    
    // Test 4: Restore original config
    config.save().expect("Failed to restore config");
    println!("\nConfig test complete!");
}