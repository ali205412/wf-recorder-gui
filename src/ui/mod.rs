mod window;
mod settings_view;
mod countdown_view;
mod recording_view;

pub use window::build_ui;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum RecordingState {
    Settings,
    Countdown,
    Recording,
}
