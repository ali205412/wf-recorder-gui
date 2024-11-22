use gtk::prelude::*;
use gtk::{Box, Button, Label, Orientation};

#[derive(Clone)]
pub struct RecordingView {
    container: Box,
    time_label: Label,
    stop_button: Button,
}

impl RecordingView {
    pub fn new() -> Self {
        let container = Box::new(Orientation::Horizontal, 8);
        container.set_margin_start(8);
        container.set_margin_end(8);
        container.set_margin_top(8);
        container.set_margin_bottom(8);
        container.add_css_class("recording-view");
        container.set_valign(gtk::Align::Center);
        container.set_halign(gtk::Align::Center);

        // Time display (MM:SS format)
        let time_label = Label::builder()
            .label("00:00")
            .css_classes(vec!["time-label"])
            .build();

        // Stop button
        let stop_button = Button::builder()
            .label("Stop")
            .css_classes(vec!["destructive-action", "stop-button"])
            .build();

        // Add elements with proper spacing
        container.append(&time_label);
        container.append(&stop_button);

        Self {
            container,
            time_label,
            stop_button,
        }
    }

    pub fn widget(&self) -> &Box {
        &self.container
    }

    pub fn update_time(&self, seconds: u32) {
        let minutes = seconds / 60;
        let seconds = seconds % 60;
        self.time_label
            .set_text(&format!("{:02}:{:02}", minutes, seconds));
    }

    pub fn connect_stop_clicked<F: Fn() + 'static>(&self, f: F) {
        self.stop_button.connect_clicked(move |_| {
            f();
        });
    }
}
