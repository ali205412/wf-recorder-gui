use gtk::prelude::*;
use gtk::{Box, Button, Label, Orientation};

#[derive(Clone)]
pub struct CountdownView {
    container: Box,
    countdown_label: Label,
    cancel_button: Button,
}

impl CountdownView {
    pub fn new() -> Self {
        let container = Box::new(Orientation::Horizontal, 8);
        container.set_margin_start(8);
        container.set_margin_end(8);
        container.set_margin_top(8);
        container.set_margin_bottom(8);
        container.add_css_class("countdown-view");
        container.set_valign(gtk::Align::Center);
        container.set_halign(gtk::Align::Center);

        // Countdown number
        let countdown_label = Label::builder()
            .label("3")
            .css_classes(vec!["countdown-number"])
            .build();

        // Cancel button
        let cancel_button = Button::builder()
            .label("Cancel")
            .css_classes(vec!["destructive-action", "cancel-button"])
            .build();

        // Add elements with proper spacing
        container.append(&countdown_label);
        container.append(&cancel_button);

        Self {
            container,
            countdown_label,
            cancel_button,
        }
    }

    pub fn widget(&self) -> &Box {
        &self.container
    }

    pub fn set_countdown(&self, count: i32) {
        self.countdown_label.set_text(&count.to_string());
    }

    pub fn connect_cancel_clicked<F: Fn() + 'static>(&self, f: F) {
        self.cancel_button.connect_clicked(move |_| {
            f();
        });
    }
}
