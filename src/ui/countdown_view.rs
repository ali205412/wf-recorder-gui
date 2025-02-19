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
        let container = Box::new(Orientation::Vertical, 2);
        container.set_margin_start(6);
        container.set_margin_end(6);
        container.set_margin_top(4);
        container.set_margin_bottom(4);
        container.set_halign(gtk::Align::Fill);
        container.set_valign(gtk::Align::Center);
        container.add_css_class("countdown-view");

        // Title
        let title = Label::builder()
            .label("Recording in...")
            .css_classes(vec!["countdown-title"])
            .halign(gtk::Align::Center)
            .margin_bottom(2)
            .build();

        // Countdown number
        let countdown_label = Label::builder()
            .label("3")
            .css_classes(vec!["countdown-number"])
            .halign(gtk::Align::Center)
            .margin_bottom(2)
            .build();

        // Cancel button
        let cancel_button = Button::builder()
            .label("Cancel")
            .css_classes(vec!["cancel-button"])
            .halign(gtk::Align::Fill)
            .build();

        container.append(&title);
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
