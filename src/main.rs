mod audio;
mod config;
mod recorder;

use anyhow::Result;
use iced::widget::{button, column, container, pick_list, row, text, Space};
use iced::{executor, Application, Command, Element, Length, Settings, Theme};
use std::path::PathBuf;
use std::time::{Duration, Instant};

use config::Config;
use recorder::{AudioSource, CaptureRegion, OutputFormat, Recorder, RecordingConfig};

fn main() -> Result<()> {
    App::run(Settings {
        window: iced::window::Settings {
            size: iced::Size::new(300.0, 400.0),
            resizable: false,
            ..Default::default()
        },
        ..Default::default()
    })?;
    Ok(())
}

#[derive(Debug, Clone)]
enum Message {
    FormatSelected(OutputFormat),
    AudioSelected(AudioSource),
    RegionSelected(CaptureRegion),
    BrowseFolder,
    FolderSelected(PathBuf),
    StartRecording,
    StopRecording,
    Tick,
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum AppState {
    Settings,
    Countdown(u8),
    Recording,
}

struct App {
    state: AppState,
    config: Config,
    recorder: Option<Recorder>,
    recording_start: Option<Instant>,
    recording_duration: Duration,
}

impl Application for App {
    type Executor = executor::Default;
    type Message = Message;
    type Theme = Theme;
    type Flags = ();

    fn new(_flags: ()) -> (Self, Command<Message>) {
        let config = Config::load().unwrap_or_default();
        
        (
            App {
                state: AppState::Settings,
                config,
                recorder: None,
                recording_start: None,
                recording_duration: Duration::default(),
            },
            Command::none(),
        )
    }

    fn title(&self) -> String {
        String::from("WF Recorder")
    }

    fn update(&mut self, message: Message) -> Command<Message> {
        match message {
            Message::FormatSelected(format) => {
                self.config.format = format;
                let _ = self.config.save();
                Command::none()
            }
            Message::AudioSelected(audio) => {
                self.config.audio = audio;
                let _ = self.config.save();
                Command::none()
            }
            Message::RegionSelected(region) => {
                self.config.region = region;
                let _ = self.config.save();
                Command::none()
            }
            Message::BrowseFolder => {
                Command::perform(
                    async move {
                        if let Some(path) = rfd::AsyncFileDialog::new()
                            .set_directory(std::env::current_dir().unwrap_or_default())
                            .pick_folder()
                            .await
                        {
                            Some(path.path().to_path_buf())
                        } else {
                            None
                        }
                    },
                    |path| {
                        if let Some(p) = path {
                            Message::FolderSelected(p)
                        } else {
                            Message::Tick // No-op
                        }
                    },
                )
            }
            Message::FolderSelected(path) => {
                self.config.output_dir = path;
                let _ = self.config.save();
                Command::none()
            }
            Message::StartRecording => {
                let recording_config = RecordingConfig {
                    format: self.config.format,
                    audio: self.config.audio,
                    region: self.config.region,
                    output_dir: self.config.output_dir.clone(),
                };

                let _recorder = Recorder::new(recording_config.clone());
                
                // Start countdown
                self.state = AppState::Countdown(3);
                
                // Handle region selection vs fullscreen
                let delay = match recording_config.region {
                    CaptureRegion::Selection => 100,
                    CaptureRegion::FullScreen => 1000,
                };
                
                Command::perform(
                    async move {
                        tokio::time::sleep(Duration::from_millis(delay)).await;
                    },
                    |_| Message::Tick,
                )
            }
            Message::StopRecording => {
                if let Some(recorder) = &mut self.recorder {
                    let _ = recorder.stop();
                }
                self.recorder = None;
                self.state = AppState::Settings;
                self.recording_start = None;
                self.recording_duration = Duration::default();
                Command::none()
            }
            Message::Tick => {
                match self.state {
                    AppState::Countdown(count) => {
                        if count > 1 {
                            self.state = AppState::Countdown(count - 1);
                            Command::perform(
                                async {
                                    tokio::time::sleep(Duration::from_secs(1)).await;
                                },
                                |_| Message::Tick,
                            )
                        } else {
                            // Start recording
                            let recording_config = RecordingConfig {
                                format: self.config.format,
                                audio: self.config.audio,
                                region: self.config.region,
                                output_dir: self.config.output_dir.clone(),
                            };
                            
                            let mut recorder = Recorder::new(recording_config);
                            if let Err(e) = recorder.start() {
                                eprintln!("Failed to start recording: {}", e);
                                self.state = AppState::Settings;
                                Command::none()
                            } else {
                                self.recorder = Some(recorder);
                                self.state = AppState::Recording;
                                self.recording_start = Some(Instant::now());
                                
                                // Start timer updates
                                Command::perform(
                                    async {
                                        tokio::time::sleep(Duration::from_secs(1)).await;
                                    },
                                    |_| Message::Tick,
                                )
                            }
                        }
                    }
                    AppState::Recording => {
                        if let Some(start) = self.recording_start {
                            self.recording_duration = start.elapsed();
                        }
                        Command::perform(
                            async {
                                tokio::time::sleep(Duration::from_secs(1)).await;
                            },
                            |_| Message::Tick,
                        )
                    }
                    AppState::Settings => Command::none(),
                }
            }
        }
    }

    fn view(&self) -> Element<Message> {
        let content = match self.state {
            AppState::Settings => self.view_settings(),
            AppState::Countdown(count) => self.view_countdown(count),
            AppState::Recording => self.view_recording(),
        };

        container(content)
            .width(Length::Fill)
            .height(Length::Fill)
            .center_x()
            .center_y()
            .padding(20)
            .into()
    }
}

impl App {
    fn view_settings(&self) -> Element<Message> {
        let format_picker = pick_list(
            vec![OutputFormat::WebM, OutputFormat::Mp4, OutputFormat::Mkv],
            Some(self.config.format),
            Message::FormatSelected,
        );

        let audio_picker = pick_list(
            vec![AudioSource::None, AudioSource::System, AudioSource::Microphone],
            Some(self.config.audio),
            Message::AudioSelected,
        );

        let region_picker = pick_list(
            vec![CaptureRegion::FullScreen, CaptureRegion::Selection],
            Some(self.config.region),
            Message::RegionSelected,
        );

        let folder_text = self.config.output_dir.to_string_lossy().to_string();
        let folder_row = row![
            text(folder_text).size(14),
            button("Browse").on_press(Message::BrowseFolder),
        ]
        .spacing(10);

        let record_button = button("Record")
            .on_press(Message::StartRecording)
            .padding(10)
            .width(Length::Fill);

        column![
            text("WF Recorder").size(24),
            Space::with_height(20),
            text("Format:").size(16),
            format_picker,
            Space::with_height(10),
            text("Audio:").size(16),
            audio_picker,
            Space::with_height(10),
            text("Capture:").size(16),
            region_picker,
            Space::with_height(10),
            text("Save to:").size(16),
            folder_row,
            Space::with_height(30),
            record_button,
        ]
        .spacing(5)
        .into()
    }

    fn view_countdown(&self, count: u8) -> Element<Message> {
        column![
            text("Recording in...").size(20),
            Space::with_height(20),
            text(count.to_string()).size(48),
            Space::with_height(20),
            button("Cancel")
                .on_press(Message::StopRecording)
                .padding(10)
                .width(Length::Fill),
        ]
        .align_items(iced::Alignment::Center)
        .into()
    }

    fn view_recording(&self) -> Element<Message> {
        let minutes = self.recording_duration.as_secs() / 60;
        let seconds = self.recording_duration.as_secs() % 60;
        let time_text = format!("{:02}:{:02}", minutes, seconds);

        column![
            text("Recording").size(20),
            Space::with_height(20),
            text(time_text).size(42),
            Space::with_height(20),
            button("Stop")
                .on_press(Message::StopRecording)
                .padding(10)
                .width(Length::Fill),
        ]
        .align_items(iced::Alignment::Center)
        .into()
    }
}

// Implement Display for our types so they work with pick_list
impl std::fmt::Display for OutputFormat {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            OutputFormat::WebM => write!(f, "WebM"),
            OutputFormat::Mp4 => write!(f, "MP4"),
            OutputFormat::Mkv => write!(f, "MKV"),
        }
    }
}

impl std::fmt::Display for AudioSource {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AudioSource::None => write!(f, "No Audio"),
            AudioSource::System => write!(f, "System Audio"),
            AudioSource::Microphone => write!(f, "Microphone"),
        }
    }
}

impl std::fmt::Display for CaptureRegion {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CaptureRegion::FullScreen => write!(f, "Full Screen"),
            CaptureRegion::Selection => write!(f, "Select Region"),
        }
    }
}