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
        let container = Box::new(Orientation::Vertical, 2);
        container.set_margin_start(6);
        container.set_margin_end(6);
        container.set_margin_top(4);
        container.set_margin_bottom(4);
        container.set_halign(gtk::Align::Fill);
        container.set_valign(gtk::Align::Center);
        container.add_css_class("recording-view");

        // Title
        let title = Label::builder()
            .label("Recording")
            .css_classes(vec!["recording-title"])
            .halign(gtk::Align::Center)
            .margin_bottom(2)
            .build();

        // Time label
        let time_label = Label::builder()
            .label("00:00")
            .css_classes(vec!["time-label"])
            .halign(gtk::Align::Center)
            .margin_bottom(2)
            .build();

        // Stop button
        let stop_button = Button::builder()
            .label("Stop")
            .css_classes(vec!["stop-button"])
            .halign(gtk::Align::Fill)
            .build();

        container.append(&title);
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
            .set_text(&format!("{minutes:02}:{seconds:02}"));
    }

    pub fn reset_time(&self) {
        self.time_label.set_text("00:00");
    }

    pub fn connect_stop_clicked<F: Fn() + 'static>(&self, f: F) {
        let this = self.clone();
        self.stop_button.connect_clicked(move |_| {
            this.reset_time();
            f();
        });
    }
}
