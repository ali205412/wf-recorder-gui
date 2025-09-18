mod audio;
mod config;
mod recorder;
mod theme;

use anyhow::Result;
use iced::widget::{button, column, container, pick_list, row, text, Space};
use iced::{
    alignment, executor, window, Application, Command, Element, Font, Length, Point, Settings,
    Size, Theme as IcedTheme,
};
use std::path::PathBuf;
use std::time::{Duration, Instant};

use config::Config;
use recorder::{AudioSource, CaptureRegion, OutputFormat, Recorder, RecordingConfig};
use theme::{design, Theme};

fn main() -> Result<()> {
    // Detect screen size early to set proper initial window size
    let screen_size = App::detect_screen_size();
    let scale_factor = design::scale_factor(screen_size.width, screen_size.height);

    // Calculate optimal initial size
    let initial_size = Size::new(
        (design::BASE_WINDOW_WIDTH * scale_factor)
            .clamp(design::MIN_WINDOW_WIDTH, design::MAX_WINDOW_WIDTH),
        (design::BASE_WINDOW_HEIGHT * scale_factor)
            .clamp(design::MIN_WINDOW_HEIGHT, design::MAX_WINDOW_HEIGHT),
    );

    App::run(Settings {
        window: iced::window::Settings {
            size: initial_size,
            resizable: true,
            decorations: true,
            transparent: false,
            min_size: Some(iced::Size::new(
                design::MIN_WINDOW_WIDTH,
                design::MIN_WINDOW_HEIGHT,
            )),
            max_size: Some(iced::Size::new(
                design::MAX_WINDOW_WIDTH,
                design::MAX_WINDOW_HEIGHT,
            )),
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
    #[allow(dead_code)]
    ResizeWindow(Size),
    #[allow(dead_code)]
    PositionWindow(Point),
    #[allow(dead_code)]
    MinimizeWindow,
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum AppState {
    Settings,
    CompactCountdown(u8),
    CompactRecording,
}

struct App {
    state: AppState,
    config: Config,
    recorder: Option<Recorder>,
    recording_start: Option<Instant>,
    recording_duration: Duration,
    theme: Theme,
    screen_size: Size,
    scale_factor: f32,
}

impl Application for App {
    type Executor = executor::Default;
    type Message = Message;
    type Theme = IcedTheme;
    type Flags = ();

    fn new(_flags: ()) -> (Self, Command<Message>) {
        let config = Config::load().unwrap_or_default();

        // Try to get actual screen size, fallback to safe default
        let screen_size = Self::detect_screen_size();
        let scale_factor = design::scale_factor(screen_size.width, screen_size.height);

        let app = App {
            state: AppState::Settings,
            config,
            recorder: None,
            recording_start: None,
            recording_duration: Duration::default(),
            theme: Theme::default(),
            screen_size,
            scale_factor,
        };

        // Ensure window is properly sized on startup
        let optimal_size = app.get_settings_size();
        let initial_command = window::resize(window::Id::MAIN, optimal_size);

        (app, initial_command)
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

                // Always use compact mode for recording - non-intrusive
                self.state = AppState::CompactCountdown(3);

                // Handle region selection vs fullscreen
                let delay = match recording_config.region {
                    CaptureRegion::Selection => 100,
                    CaptureRegion::FullScreen => 1000,
                };

                // Always switch to compact mode for recording
                let compact_size = self.get_compact_size();
                let position = self.get_compact_position();

                Command::batch([
                    window::resize(window::Id::MAIN, compact_size),
                    window::move_to(window::Id::MAIN, position),
                    Command::perform(
                        async move {
                            tokio::time::sleep(Duration::from_millis(delay)).await;
                        },
                        |_| Message::Tick,
                    ),
                ])
            }
            Message::StopRecording => {
                if let Some(recorder) = &mut self.recorder {
                    let _ = recorder.stop();
                }
                self.recorder = None;
                self.state = AppState::Settings;
                self.recording_start = None;
                self.recording_duration = Duration::default();

                // Return to normal settings window size and center
                let settings_size = self.get_settings_size();
                let center_position = self.get_center_position(settings_size);

                Command::batch([
                    window::resize(window::Id::MAIN, settings_size),
                    window::move_to(window::Id::MAIN, center_position),
                ])
            }
            Message::Tick => {
                match self.state {
                    AppState::CompactCountdown(count) => {
                        if count > 1 {
                            self.state = AppState::CompactCountdown(count - 1);
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
                                let settings_size = self.get_settings_size();
                                let center_position = self.get_center_position(settings_size);
                                Command::batch([
                                    window::resize(window::Id::MAIN, settings_size),
                                    window::move_to(window::Id::MAIN, center_position),
                                ])
                            } else {
                                self.recorder = Some(recorder);
                                // Always use compact recording mode - non-intrusive
                                self.state = AppState::CompactRecording;
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
                    AppState::CompactRecording => {
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
            Message::ResizeWindow(size) => window::resize(window::Id::MAIN, size),
            Message::PositionWindow(position) => window::move_to(window::Id::MAIN, position),
            Message::MinimizeWindow => window::minimize(window::Id::MAIN, true),
        }
    }

    fn view(&self) -> Element<Message> {
        let content = match self.state {
            AppState::Settings => self.view_settings(),
            AppState::CompactCountdown(count) => self.view_compact_countdown(count),
            AppState::CompactRecording => self.view_compact_recording(),
        };

        // Dynamic window padding based on scale factor and mode
        let padding = match self.state {
            AppState::CompactCountdown(_) | AppState::CompactRecording => {
                design::COMPACT_BUTTON_PADDING
            }
            _ => design::window_padding(self.scale_factor),
        };

        // Main window container with responsive padding
        container(content)
            .width(Length::Fill)
            .height(Length::Fill)
            .padding(padding)
            .style(iced::theme::Container::Custom(Box::new(
                theme::WindowStyle(self.theme.colors),
            )))
            .into()
    }
}

impl App {
    // Detect screen size using multiple methods
    pub fn detect_screen_size() -> Size {
        // Method 1: Check environment variables
        if let (Ok(width), Ok(height)) = (
            std::env::var("SCREEN_WIDTH")
                .and_then(|w| w.parse::<f32>().map_err(|_| std::env::VarError::NotPresent)),
            std::env::var("SCREEN_HEIGHT")
                .and_then(|h| h.parse::<f32>().map_err(|_| std::env::VarError::NotPresent)),
        ) {
            return Size::new(width, height);
        }

        // Method 2: Try xrandr on Linux (common case)
        if let Ok(output) = std::process::Command::new("xrandr")
            .arg("--current")
            .output()
        {
            if let Ok(output_str) = String::from_utf8(output.stdout) {
                // Parse xrandr output for primary display
                for line in output_str.lines() {
                    if line.contains("primary") || line.contains("*") {
                        if let Some(resolution) = line.split_whitespace().find(|s| {
                            s.contains("x") && s.chars().next().unwrap_or(' ').is_ascii_digit()
                        }) {
                            let parts: Vec<&str> = resolution.split('x').collect();
                            if parts.len() == 2 {
                                if let (Ok(w), Ok(h)) =
                                    (parts[0].parse::<f32>(), parts[1].parse::<f32>())
                                {
                                    return Size::new(w, h);
                                }
                            }
                        }
                    }
                }
            }
        }

        // Method 3: Common screen size detection based on typical resolutions
        // This could be expanded with more platform-specific code

        // Default fallback - use reference resolution
        Size::new(design::REFERENCE_WIDTH, design::REFERENCE_HEIGHT)
    }
    fn view_settings(&self) -> Element<Message> {
        let colors = self.theme.colors;

        // Dynamic sizes based on scale factor
        let title_size = design::title_size(self.scale_factor);
        let subtitle_size = design::subtitle_size(self.scale_factor);
        let section_spacing = design::section_spacing(self.scale_factor);
        let container_padding = design::container_padding(self.scale_factor);

        // Title with subtitle - responsive sizing
        let title_section = container(
            column![
                text("WF Recorder")
                    .size(title_size)
                    .font(Font {
                        weight: iced::font::Weight::Bold,
                        ..Default::default()
                    })
                    .style(iced::theme::Text::Color(colors.text)),
                text(format!("{} • 30FPS", self.config.format))
                    .size(subtitle_size)
                    .style(iced::theme::Text::Color(colors.text_secondary)),
            ]
            .spacing(design::tiny_space(self.scale_factor) as u16),
        )
        .width(Length::Fill)
        .padding([0, 0, section_spacing, 0])
        .style(iced::theme::Container::Custom(Box::new(
            theme::ContainerStyle(colors),
        )));

        // Capture mode buttons - responsive spacing
        let capture_buttons = row![
            self.create_option_button(
                "🖥",
                "Screen",
                matches!(self.config.region, CaptureRegion::FullScreen),
                Message::ToggleRegion(true),
            ),
            Space::with_width(Length::Fixed(container_padding as f32)),
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
            Space::with_width(Length::Fixed(container_padding as f32)),
            self.create_option_button(
                "🎤",
                "Mic",
                matches!(self.config.audio, AudioSource::Microphone),
                Message::ToggleAudio(AudioSource::Microphone),
            ),
            Space::with_width(Length::Fixed(container_padding as f32)),
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
                .padding([container_padding, container_padding])
                .width(Length::Fill)
                .text_size(design::input_text_size(self.scale_factor)),
            )
            .width(Length::Fill)
            .style(iced::theme::Container::Custom(Box::new(theme::CardStyle(
                colors,
            )))),
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
                        .size(design::button_text_size(self.scale_factor))
                        .style(iced::theme::Text::Color(colors.text_secondary)),
                    Space::with_width(Length::Fill),
                    button(text("Browse").size(design::button_text_size(self.scale_factor)))
                        .on_press(Message::BrowseFolder)
                        .padding([8, 16])
                        .style(iced::theme::Button::Custom(Box::new(
                            theme::SecondaryButton(colors)
                        ))),
                ]
                .align_items(alignment::Alignment::Center),
            )
            .padding(container_padding)
            .width(Length::Fill)
            .style(iced::theme::Container::Custom(Box::new(theme::CardStyle(
                colors,
            )))),
        );

        // Record button - primary action
        let record_button = button(
            text("Start Recording")
                .size(design::input_text_size(self.scale_factor))
                .horizontal_alignment(alignment::Horizontal::Center),
        )
        .on_press(Message::StartRecording)
        .padding([
            design::button_padding_v(self.scale_factor),
            design::button_padding_h(self.scale_factor),
        ])
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
            .spacing(section_spacing),
        )
        .width(Length::Fill)
        .height(Length::Fill)
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
                .size(design::label_size(self.scale_factor))
                .style(iced::theme::Text::Color(self.theme.colors.text_secondary)),
            Space::with_height(Length::Fixed(design::small_space(self.scale_factor))),
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
                text(icon).size(24), // Keep icon size fixed for consistency
                Space::with_height(Length::Fixed(design::tiny_space(self.scale_factor))),
                text(label).size(design::button_text_size(self.scale_factor))
            ]
            .spacing(0)
            .align_items(alignment::Alignment::Center)
            .width(Length::Fixed(design::button_width(self.scale_factor) as f32)),
        )
        .on_press(message)
        .padding([design::button_padding_v(self.scale_factor), 0]) // Vertical padding only
        .width(Length::Fixed(design::button_width(self.scale_factor) as f32))
        .height(Length::Fixed(
            design::button_height(self.scale_factor) as f32
        ))
        .style(iced::theme::Button::Custom(Box::new(
            theme::OptionCardStyle(self.theme.colors, is_active),
        )))
        .into()
    }

    // Compact countdown view - minimal UI for recording
    fn view_compact_countdown(&self, count: u8) -> Element<Message> {
        let colors = self.theme.colors;

        container(
            row![
                text(count.to_string())
                    .size(design::compact_countdown_size(self.scale_factor))
                    .font(Font {
                        weight: iced::font::Weight::Bold,
                        ..Default::default()
                    })
                    .style(iced::theme::Text::Color(colors.primary)),
                Space::with_width(Length::Fixed(design::small_space(self.scale_factor))),
                button(text("✕").size(design::COMPACT_ICON_SIZE))
                    .on_press(Message::StopRecording)
                    .padding(design::COMPACT_BUTTON_PADDING)
                    .style(iced::theme::Button::Custom(Box::new(theme::CompactButton(
                        colors
                    )))),
            ]
            .align_items(alignment::Alignment::Center),
        )
        .width(Length::Fill)
        .height(Length::Fill)
        .center_x()
        .center_y()
        .padding(design::COMPACT_BUTTON_PADDING)
        .style(iced::theme::Container::Custom(Box::new(
            theme::CompactStyle(colors),
        )))
        .into()
    }

    // Compact recording view - minimal recording indicator
    fn view_compact_recording(&self) -> Element<Message> {
        let colors = self.theme.colors;
        let minutes = self.recording_duration.as_secs() / 60;
        let seconds = self.recording_duration.as_secs() % 60;
        let time_text = format!("{minutes:02}:{seconds:02}");

        container(
            row![
                text("●")
                    .size(design::COMPACT_ICON_SIZE)
                    .style(iced::theme::Text::Color(colors.danger)),
                Space::with_width(Length::Fixed(design::small_space(self.scale_factor))),
                text(time_text)
                    .size(design::timer_text_size(self.scale_factor))
                    .font(Font {
                        family: iced::font::Family::Monospace,
                        weight: iced::font::Weight::Medium,
                        ..Default::default()
                    })
                    .style(iced::theme::Text::Color(colors.text)),
                Space::with_width(Length::Fixed(design::small_space(self.scale_factor))),
                button(text("⏹").size(design::COMPACT_ICON_SIZE))
                    .on_press(Message::StopRecording)
                    .padding(design::COMPACT_BUTTON_PADDING)
                    .style(iced::theme::Button::Custom(Box::new(theme::CompactButton(
                        colors
                    )))),
            ]
            .align_items(alignment::Alignment::Center),
        )
        .width(Length::Fill)
        .height(Length::Fill)
        .center_x()
        .center_y()
        .padding(design::COMPACT_BUTTON_PADDING)
        .style(iced::theme::Container::Custom(Box::new(
            theme::RecordingIndicator(colors),
        )))
        .into()
    }

    // Helper methods for window sizing and positioning
    fn get_settings_size(&self) -> Size {
        Size::new(
            (design::BASE_WINDOW_WIDTH * self.scale_factor)
                .clamp(design::MIN_WINDOW_WIDTH, design::MAX_WINDOW_WIDTH),
            (design::BASE_WINDOW_HEIGHT * self.scale_factor)
                .clamp(design::MIN_WINDOW_HEIGHT, design::MAX_WINDOW_HEIGHT),
        )
    }

    fn get_compact_size(&self) -> Size {
        // Scale compact window slightly for very high DPI displays
        let compact_scale = self.scale_factor.max(1.0);
        Size::new(
            design::COMPACT_WINDOW_WIDTH * compact_scale,
            design::COMPACT_WINDOW_HEIGHT * compact_scale,
        )
    }

    fn get_compact_position(&self) -> Point {
        let compact_size = self.get_compact_size();
        let padding = design::COMPACT_WINDOW_PADDING * self.scale_factor;

        // Ensure the window stays within screen bounds
        let x = (self.screen_size.width - compact_size.width - padding).max(0.0);
        let y = padding;

        Point::new(x, y)
    }

    fn get_center_position(&self, window_size: Size) -> Point {
        let x = ((self.screen_size.width - window_size.width) / 2.0).max(0.0);
        let y = ((self.screen_size.height - window_size.height) / 2.0).max(0.0);

        Point::new(x, y)
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
