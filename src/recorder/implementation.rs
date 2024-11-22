use anyhow::{Context, Result};
use chrono::Local;
use std::path::PathBuf;
use std::process::Command;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum OutputFormat {
    WebM,
    Mp4,
    Mkv,
}

impl OutputFormat {
    pub fn extension(&self) -> &'static str {
        match self {
            OutputFormat::WebM => "webm",
            OutputFormat::Mp4 => "mp4",
            OutputFormat::Mkv => "mkv",
        }
    }

    pub fn all() -> &'static [(OutputFormat, &'static str)] {
        &[
            (OutputFormat::WebM, "WebM - Best for web"),
            (OutputFormat::Mp4, "MP4 - Most compatible"),
            (OutputFormat::Mkv, "MKV - Best quality"),
        ]
    }

    fn codec(&self) -> &'static str {
        match self {
            OutputFormat::WebM => "libvpx",
            OutputFormat::Mp4 => "libx264",
            OutputFormat::Mkv => "libx264",
        }
    }
}

#[derive(Debug, Clone)]
pub enum AudioSource {
    None,
    System,
    Microphone,
}

#[derive(Debug, Clone)]
pub enum CaptureRegion {
    FullScreen,
    Selection,
}

#[derive(Debug, Clone)]
pub struct RecordingConfig {
    pub format: OutputFormat,
    pub audio: AudioSource,
    pub region: CaptureRegion,
    pub output_dir: PathBuf,
}

#[derive(Clone)]
pub struct Recorder {
    config: RecordingConfig,
    pid: Option<u32>,
}

impl Recorder {
    pub fn new(config: RecordingConfig) -> Self {
        Self { config, pid: None }
    }

    fn generate_filename(&self) -> PathBuf {
        let timestamp = Local::now().format("%Y%m%d_%H%M%S");
        let mut path = self.config.output_dir.clone();
        path.push(format!(
            "recording_{}.{}",
            timestamp,
            self.config.format.extension()
        ));
        path
    }

    pub fn start(&mut self) -> Result<()> {
        // Ensure wf-recorder is installed
        which::which("wf-recorder").context("wf-recorder not found. Please install it first.")?;

        // Build base command
        let mut cmd = Command::new("wf-recorder");

        // Generate unique filename
        let output_file = self.generate_filename();
        cmd.arg("-f").arg(&output_file);

        // Use software encoding by default
        cmd.arg("--codec").arg(self.config.format.codec());

        // Add audio configuration
        match self.config.audio {
            AudioSource::None => {}
            AudioSource::System => {
                cmd.arg("-a");
            }
            AudioSource::Microphone => {
                // Get default mic
                let output = Command::new("pactl")
                    .args(["list", "sources", "short"])
                    .output()?;
                let sources = String::from_utf8_lossy(&output.stdout);
                if let Some(mic) = sources.lines().next() {
                    let mic_name = mic.split('\t').next().unwrap_or("");
                    cmd.arg("-a").arg(mic_name);
                }
            }
        }

        // Add region selection if needed
        if let CaptureRegion::Selection = self.config.region {
            cmd.arg("-g").arg("");
        }

        // Start the recording process
        let child = cmd.spawn().context("Failed to start wf-recorder")?;
        self.pid = Some(child.id());

        Ok(())
    }

    pub fn stop(&mut self) -> Result<()> {
        if let Some(pid) = self.pid.take() {
            Command::new("kill")
                .args(["-s", "INT", &pid.to_string()])
                .spawn()?
                .wait()?;
        }
        Ok(())
    }
}

impl Drop for Recorder {
    fn drop(&mut self) {
        let _ = self.stop();
    }
}
