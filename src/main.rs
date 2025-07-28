mod audio;
mod config;
mod recorder;
mod theme;

use anyhow::Result;
use iced::widget::{button, column, container, pick_list, row, text, Space};
use iced::{
    alignment, executor, Application, Command, Element, Font, Length, Settings, Theme as IcedTheme,
};
use std::path::PathBuf;
use std::time::{Duration, Instant};

use config::Config;
use recorder::{AudioSource, CaptureRegion, OutputFormat, Recorder, RecordingConfig};
use theme::{design, Theme};

fn main() -> Result<()> {
    App::run(Settings {
        window: iced::window::Settings {
            size: iced::Size::new(420.0, 520.0), // Onagre-like size
            resizable: false,
            decorations: true,
            transparent: false,
            ..Default::default()
        },
        antialiasing: true,
        default_font: Font::default(),
        ..Default::default()
    })?;
    Ok(())
}

#[derive(Debug, Clone)]
enum Message {
    FormatSelected(OutputFormat),
    ToggleRegion(bool),
    ToggleAudio(AudioSource),
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
    theme: Theme,
}

impl Application for App {
    type Executor = executor::Default;
    type Message = Message;
    type Theme = IcedTheme;
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
                theme: Theme::default(),
            },
            Command::none(),
        )
    }

    fn title(&self) -> String {
        String::from("WF Recorder")
    }

    fn theme(&self) -> IcedTheme {
        IcedTheme::Dark
    }

    fn update(&mut self, message: Message) -> Command<Message> {
        match message {
            Message::FormatSelected(format) => {
                self.config.format = format;
                let _ = self.config.save();
                Command::none()
            }
            Message::ToggleRegion(is_fullscreen) => {
                self.config.region = if is_fullscreen {
                    CaptureRegion::FullScreen
                } else {
                    CaptureRegion::Selection
                };
                let _ = self.config.save();
                Command::none()
            }
            Message::ToggleAudio(source) => {
                self.config.audio = source;
                let _ = self.config.save();
                Command::none()
            }
            Message::BrowseFolder => {
                let current_dir = self.config.output_dir.clone();
                Command::perform(
                    async move {
                        rfd::AsyncFileDialog::new()
                            .set_directory(current_dir)
                            .pick_folder()
                            .await
                            .map(|path| path.path().to_path_buf())
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
                                eprintln!("Failed to start recording: {e}");
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

        // Main window container with onagre-style padding
        container(content)
            .width(Length::Fill)
            .height(Length::Fill)
            .padding(design::WINDOW_PADDING)
            .style(iced::theme::Container::Custom(Box::new(
                theme::WindowStyle(self.theme.colors),
            )))
            .into()
    }
}

impl App {
    fn view_settings(&self) -> Element<Message> {
        let colors = self.theme.colors;

        // Title with subtitle - like onagre's search bar
        let title_section = container(
            column![
                text("WF Recorder")
                    .size(design::TITLE_SIZE)
                    .font(Font {
                        weight: iced::font::Weight::Bold,
                        ..Default::default()
                    })
                    .style(iced::theme::Text::Color(colors.text)),
                text(format!("{} • 30FPS", self.config.format))
                    .size(design::SUBTITLE_SIZE)
                    .style(iced::theme::Text::Color(colors.text_secondary)),
            ]
            .spacing(4),
        )
        .width(Length::Fill)
        .padding([0, 0, design::SECTION_SPACING, 0])
        .style(iced::theme::Container::Custom(Box::new(
            theme::ContainerStyle(colors),
        )));

        // Capture mode buttons - like onagre's rows
        let capture_buttons = row![
            self.create_option_button(
                "🖥",
                "Screen",
                matches!(self.config.region, CaptureRegion::FullScreen),
                Message::ToggleRegion(true),
            ),
            Space::with_width(Length::Fixed(design::CONTAINER_PADDING as f32)),
            self.create_option_button(
                "◰",
                "Region",
                matches!(self.config.region, CaptureRegion::Selection),
                Message::ToggleRegion(false),
            ),
        ];

        let capture_section = self.create_section("CAPTURE MODE", capture_buttons);

        // Audio source buttons
        let audio_buttons = row![
            self.create_option_button(
                "🔊",
                "System",
                matches!(self.config.audio, AudioSource::System),
                Message::ToggleAudio(AudioSource::System),
            ),
            Space::with_width(Length::Fixed(design::CONTAINER_PADDING as f32)),
            self.create_option_button(
                "🎤",
                "Mic",
                matches!(self.config.audio, AudioSource::Microphone),
                Message::ToggleAudio(AudioSource::Microphone),
            ),
            Space::with_width(Length::Fixed(design::CONTAINER_PADDING as f32)),
            self.create_option_button(
                "🔇",
                "None",
                matches!(self.config.audio, AudioSource::None),
                Message::ToggleAudio(AudioSource::None),
            ),
        ];

        let audio_section = self.create_section("AUDIO SOURCE", audio_buttons);

        // Format picker - styled like onagre's search input
        let format_section = self.create_section(
            "OUTPUT FORMAT",
            container(
                pick_list(
                    vec![OutputFormat::WebM, OutputFormat::Mp4, OutputFormat::Mkv],
                    Some(self.config.format),
                    Message::FormatSelected,
                )
                .padding([design::CONTAINER_PADDING, design::CONTAINER_PADDING])
                .width(Length::Fill)
                .text_size(design::INPUT_TEXT_SIZE),
            )
            .width(Length::Fill)
            .style(iced::theme::Container::Custom(Box::new(
                theme::SearchStyle(colors),
            ))),
        );

        // Save location
        let folder_text = self.config.output_dir.to_string_lossy().to_string();
        let folder_display = if folder_text.len() > 35 {
            format!("...{}", &folder_text[folder_text.len() - 32..])
        } else {
            folder_text
        };

        let location_section = self.create_section(
            "SAVE LOCATION",
            container(
                row![
                    text(folder_display)
                        .size(design::BUTTON_TEXT_SIZE)
                        .style(iced::theme::Text::Color(colors.text_secondary)),
                    Space::with_width(Length::Fill),
                    button(text("Browse").size(design::BUTTON_TEXT_SIZE))
                        .on_press(Message::BrowseFolder)
                        .padding([8, 16])
                        .style(iced::theme::Button::Custom(Box::new(
                            theme::SecondaryButton(colors)
                        ))),
                ]
                .align_items(alignment::Alignment::Center),
            )
            .padding(design::CONTAINER_PADDING)
            .width(Length::Fill)
            .style(iced::theme::Container::Custom(Box::new(
                theme::SearchStyle(colors),
            ))),
        );

        // Record button - primary action
        let record_button = button(
            text("Start Recording")
                .size(design::INPUT_TEXT_SIZE)
                .horizontal_alignment(alignment::Horizontal::Center),
        )
        .on_press(Message::StartRecording)
        .padding([design::BUTTON_PADDING_V, design::BUTTON_PADDING_H])
        .width(Length::Fill)
        .style(iced::theme::Button::Custom(Box::new(theme::PrimaryButton(
            colors,
        ))));

        // Layout with onagre-style spacing
        container(
            column![
                title_section,
                capture_section,
                audio_section,
                format_section,
                location_section,
                Space::with_height(Length::Fill), // Push button to bottom
                record_button,
            ]
            .spacing(design::SECTION_SPACING),
        )
        .width(Length::Fill)
        .height(Length::Fill)
        .style(iced::theme::Container::Custom(Box::new(
            theme::ContainerStyle(colors),
        )))
        .into()
    }

    fn view_countdown(&self, count: u8) -> Element<Message> {
        let colors = self.theme.colors;

        container(
            column![
                text("Recording in...")
                    .size(design::SUBTITLE_SIZE)
                    .style(iced::theme::Text::Color(colors.text_secondary)),
                Space::with_height(Length::Fixed(32.0)),
                container(
                    text(count.to_string())
                        .size(72)
                        .font(Font {
                            weight: iced::font::Weight::Bold,
                            ..Default::default()
                        })
                        .style(iced::theme::Text::Color(colors.primary))
                )
                .width(Length::Fixed(120.0))
                .height(Length::Fixed(120.0))
                .center_x()
                .center_y()
                .style(iced::theme::Container::Custom(Box::new(
                    theme::SearchStyle(colors)
                ))),
                Space::with_height(Length::Fixed(32.0)),
                button(text("Cancel").size(design::BUTTON_TEXT_SIZE))
                    .on_press(Message::StopRecording)
                    .padding([design::BUTTON_PADDING_V, design::BUTTON_PADDING_H])
                    .style(iced::theme::Button::Custom(Box::new(theme::DangerButton(
                        colors
                    )))),
            ]
            .align_items(alignment::Alignment::Center)
            .spacing(0),
        )
        .width(Length::Fill)
        .height(Length::Fill)
        .center_x()
        .center_y()
        .style(iced::theme::Container::Custom(Box::new(
            theme::ContainerStyle(colors),
        )))
        .into()
    }

    fn view_recording(&self) -> Element<Message> {
        let colors = self.theme.colors;
        let minutes = self.recording_duration.as_secs() / 60;
        let seconds = self.recording_duration.as_secs() % 60;
        let time_text = format!("{minutes:02}:{seconds:02}");

        container(
            column![
                row![
                    text("⏸")
                        .size(20)
                        .style(iced::theme::Text::Color(colors.danger)),
                    Space::with_width(Length::Fixed(8.0)),
                    text("Recording")
                        .size(design::SUBTITLE_SIZE)
                        .style(iced::theme::Text::Color(colors.text)),
                ]
                .align_items(alignment::Alignment::Center),
                Space::with_height(Length::Fixed(32.0)),
                text(time_text)
                    .size(56)
                    .font(Font {
                        family: iced::font::Family::Monospace,
                        weight: iced::font::Weight::Light,
                        ..Default::default()
                    })
                    .style(iced::theme::Text::Color(colors.text)),
                Space::with_height(Length::Fixed(32.0)),
                button(text("Stop Recording").size(design::BUTTON_TEXT_SIZE))
                    .on_press(Message::StopRecording)
                    .padding([design::BUTTON_PADDING_V, design::BUTTON_PADDING_H])
                    .style(iced::theme::Button::Custom(Box::new(theme::DangerButton(
                        colors
                    )))),
            ]
            .align_items(alignment::Alignment::Center)
            .spacing(0),
        )
        .width(Length::Fill)
        .height(Length::Fill)
        .center_x()
        .center_y()
        .style(iced::theme::Container::Custom(Box::new(
            theme::ContainerStyle(colors),
        )))
        .into()
    }

    // Helper to create sections with labels
    fn create_section<'a>(
        &self,
        label: &str,
        content: impl Into<Element<'a, Message>>,
    ) -> Element<'a, Message> {
        column![
            text(label)
                .size(design::LABEL_SIZE)
                .style(iced::theme::Text::Color(self.theme.colors.text_secondary)),
            Space::with_height(Length::Fixed(8.0)),
            content.into(),
        ]
        .spacing(0)
        .into()
    }

    // Helper to create option buttons matching onagre's row style
    fn create_option_button(
        &self,
        icon: &str,
        label: &str,
        is_active: bool,
        message: Message,
    ) -> Element<Message> {
        button(
            column![
                text(icon).size(24), // Fixed icon size
                Space::with_height(Length::Fixed(4.0)),
                text(label).size(design::BUTTON_TEXT_SIZE)
            ]
            .spacing(0)
            .align_items(alignment::Alignment::Center)
            .width(Length::Fixed(design::BUTTON_WIDTH as f32)),
        )
        .on_press(message)
        .padding([design::BUTTON_PADDING_V, 0]) // Vertical padding only
        .width(Length::Fixed(design::BUTTON_WIDTH as f32))
        .height(Length::Fixed(design::BUTTON_HEIGHT as f32))
        .style(iced::theme::Button::Custom(Box::new(theme::RowStyle(
            self.theme.colors,
            is_active,
        ))))
        .into()
    }
}

// Implement Display for our types
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
