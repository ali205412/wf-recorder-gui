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
    pub available_outputs: Vec<AvailableOutput>,
    pub selected_output: Option<AvailableOutput>,
    pub format: OutputFormat,
    pub audio: AudioSource,
    pub region: CaptureRegion,
    pub output_dir: PathBuf,
}

impl RecordingConfig {
    pub fn new() -> Result<Self> {
        let available_outputs = AvailableOutput::list()?;
        Ok(Self {
            available_outputs: available_outputs,
            selected_output: None,
            format: OutputFormat::Mp4,
            audio: AudioSource::None,
            region: CaptureRegion::FullScreen,
            output_dir: std::env::temp_dir(),
        })
    }

    pub fn new_with_defaults() -> Self {
        let available_outputs = AvailableOutput::list().unwrap_or_default();
        Self {
            available_outputs: available_outputs,
            selected_output: None,
            format: OutputFormat::Mp4,
            audio: AudioSource::None,
            region: CaptureRegion::FullScreen,
            output_dir: std::env::temp_dir(),
        }
    }

    pub fn has_multiple_outputs(&self) -> bool {
        self.available_outputs.len() > 1
    }

    pub fn get_available_outputs(&self) -> &Vec<AvailableOutput> {
        &self.available_outputs
    }

    pub fn set_selected_output(&mut self, index: usize) {
        if let Some(output) = self.available_outputs.get(index) {
            self.selected_output = Some(output.clone());
        }
    }

    pub fn set_selected_output_by_name(&mut self, output_name: &str) {
        if let Some(output) = self
            .available_outputs
            .iter()
            .find(|o| o.output_name == output_name)
        {
            self.selected_output = Some(output.clone());
        }
    }

    pub fn get_selected_output(&self) -> Option<&AvailableOutput> {
        self.selected_output.as_ref()
    }

    // Auto-select first output if multiple available and none selected
    pub fn ensure_output_selected(&mut self) {
        if self.selected_output.is_none() && !self.available_outputs.is_empty() {
            self.selected_output = Some(self.available_outputs[0].clone());
        }
    }
}

impl Default for RecordingConfig {
    fn default() -> Self {
        Self::new_with_defaults()
    }
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
            // Check for slurp
            which::which("slurp")
                .context("slurp not found. Please install it first to use region selection.")?;

            // Run slurp to get geometry
            let geometry = Command::new("slurp")
                .output()
                .context("Failed to run slurp")?;

            let geometry = String::from_utf8_lossy(&geometry.stdout);
            let geometry = geometry.trim();

            cmd.arg("-g").arg(geometry);
        } else {
            if self.config.has_multiple_outputs() {
                if let Some(selected_output) = self.config.get_selected_output() {
                    cmd.arg("-o").arg(&selected_output.output_name);
                }
            }
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
#[derive(Debug, Clone, PartialEq)]
pub struct AvailableOutput {
    pub output_name: String,
    pub description: String,
}

impl AvailableOutput {
    pub fn list() -> Result<Vec<AvailableOutput>> {
        which::which("wf-recorder").context("wf-recorder not found. Please install it first.")?;

        let output = Command::new("wf-recorder")
            .args(["--list-output"])
            .output()?;

        let sources = String::from_utf8_lossy(&output.stdout);
        let mut available_outputs = Vec::new();

        for line in sources.lines() {
            if let Some(parsed) = Self::parse_line(line) {
                available_outputs.push(parsed);
            }
        }

        Ok(available_outputs)
    }

    fn parse_line(line: &str) -> Option<AvailableOutput> {
        let name_start = line.find("Name: ")? + 6;
        let name_part = &line[name_start..];
        let name_end = name_part.find(" Description: ")?;
        let output_name = name_part[..name_end].to_string();

        let desc_start = line.find("Description: ")? + 13;
        let description = line[desc_start..].to_string();

        Some(AvailableOutput {
            output_name,
            description,
        })
    }

    pub fn display_name(&self) -> String {
        format!(
            "{} ({})",
            self.output_name,
            self.description
                .split_whitespace()
                .take(3)
                .collect::<Vec<_>>()
                .join(" ")
        )
    }
}
