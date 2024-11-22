mod countdown_view;
mod recording_view;
mod settings_view;
mod window;

pub use window::build_ui;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum RecordingState {
    Settings,
    Countdown,
    Recording,
}
