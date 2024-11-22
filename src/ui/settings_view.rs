use gtk::prelude::*;
use gtk::{Box, Button, Image, Orientation, FileChooserDialog, FileChooserAction, ResponseType};
use gtk::{DropDown, StringList, Label};
use std::cell::RefCell;
use std::rc::Rc;
use std::path::PathBuf;

use crate::recorder::{OutputFormat, AudioSource, CaptureRegion};

#[derive(Clone)]
pub struct RecordingOptions {
    pub format: OutputFormat,
    pub audio: AudioSource,
    pub region: CaptureRegion,
    pub output_dir: PathBuf,
}

impl Default for RecordingOptions {
    fn default() -> Self {
        Self {
            format: OutputFormat::WebM,
            audio: AudioSource::None,
            region: CaptureRegion::FullScreen,
            output_dir: std::env::temp_dir(),
        }
    }
}

#[derive(Clone)]
pub struct SettingsView {
    container: Box,
    record_button: Button,
    options: Rc<RefCell<RecordingOptions>>,
    dir_label: Label,
}

impl SettingsView {
    pub fn new() -> Self {
        let container = Box::new(Orientation::Vertical, 0);
        container.add_css_class("settings-view");

        let options = Rc::new(RefCell::new(RecordingOptions::default()));

        // Main content box with padding
        let content = Box::new(Orientation::Vertical, 20);
        content.set_margin_start(24);
        content.set_margin_end(24);
        content.set_margin_top(24);
        content.set_margin_bottom(24);

        // Format selection dropdown
        let format_box = Box::new(Orientation::Vertical, 8);
        format_box.set_halign(gtk::Align::Fill);

        let format_label = Label::builder()
            .label("Format")
            .halign(gtk::Align::Start)
            .css_classes(vec!["setting-label"])
            .build();

        let format_model = StringList::new(&[]);
        for (_, desc) in OutputFormat::all() {
            format_model.append(desc);
        }

        let format_dropdown = DropDown::builder()
            .model(&format_model)
            .css_classes(vec!["format-dropdown"])
            .build();

        let options_clone = options.clone();
        format_dropdown.connect_selected_notify(move |dropdown| {
            let idx = dropdown.selected();
            let format = OutputFormat::all()[idx as usize].0;
            options_clone.borrow_mut().format = format;
        });

        format_box.append(&format_label);
        format_box.append(&format_dropdown);

        // Screen selection buttons
        let screen_box = Box::new(Orientation::Vertical, 8);
        screen_box.set_halign(gtk::Align::Fill);

        let screen_label = Label::builder()
            .label("Capture Mode")
            .halign(gtk::Align::Start)
            .css_classes(vec!["setting-label"])
            .build();

        let screen_buttons = Box::new(Orientation::Horizontal, 12);
        screen_buttons.set_halign(gtk::Align::Fill);

        let screen_btn = Rc::new(create_option_button("display-symbolic", "Full Screen", true));
        let region_btn = Rc::new(create_option_button("selection-mode-symbolic", "Select Region", false));

        screen_buttons.append(screen_btn.as_ref());
        screen_buttons.append(region_btn.as_ref());

        screen_box.append(&screen_label);
        screen_box.append(&screen_buttons);

        // Audio options
        let audio_box = Box::new(Orientation::Vertical, 8);
        audio_box.set_halign(gtk::Align::Fill);

        let audio_label = Label::builder()
            .label("Audio Source")
            .halign(gtk::Align::Start)
            .css_classes(vec!["setting-label"])
            .build();

        let audio_buttons = Box::new(Orientation::Horizontal, 12);
        audio_buttons.set_halign(gtk::Align::Fill);

        let audio_btn = Rc::new(create_option_button("audio-volume-high-symbolic", "System Audio", false));
        let mic_btn = Rc::new(create_option_button("audio-input-microphone-symbolic", "Microphone", false));
        let mute_btn = Rc::new(create_option_button("audio-volume-muted-symbolic", "No Audio", true));

        audio_buttons.append(audio_btn.as_ref());
        audio_buttons.append(mic_btn.as_ref());
        audio_buttons.append(mute_btn.as_ref());

        audio_box.append(&audio_label);
        audio_box.append(&audio_buttons);

        // Output directory selection
        let dir_box = Box::new(Orientation::Vertical, 8);
        dir_box.set_halign(gtk::Align::Fill);

        let dir_label = Label::builder()
            .label("Save Location")
            .halign(gtk::Align::Start)
            .css_classes(vec!["setting-label"])
            .build();

        let dir_path_box = Box::new(Orientation::Horizontal, 8);
        dir_path_box.set_halign(gtk::Align::Fill);

        let dir_path_label = Label::builder()
            .label(&*options.borrow().output_dir.to_string_lossy())
            .halign(gtk::Align::Start)
            .hexpand(true)
            .css_classes(vec!["path-label"])
            .build();

        let dir_btn = Button::builder()
            .label("Choose")
            .css_classes(vec!["folder-button"])
            .build();

        let options_clone = options.clone();
        let dir_path_label_clone = dir_path_label.clone();
        dir_btn.connect_clicked(move |_| {
            let dialog = FileChooserDialog::new(
                Some("Choose Save Location"),
                None::<&gtk::Window>,
                FileChooserAction::SelectFolder,
                &[("Cancel", ResponseType::Cancel), ("Select", ResponseType::Accept)],
            );

            let options = options_clone.clone();
            let dir_path_label = dir_path_label_clone.clone();
            dialog.connect_response(move |dialog, response| {
                if response == ResponseType::Accept {
                    if let Some(file) = dialog.file() {
                        if let Some(path) = file.path() {
                            options.borrow_mut().output_dir = path.clone();
                            dir_path_label.set_label(&path.to_string_lossy());
                        }
                    }
                }
                dialog.close();
            });

            dialog.show();
        });

        dir_path_box.append(&dir_path_label);
        dir_path_box.append(&dir_btn);

        dir_box.append(&dir_label);
        dir_box.append(&dir_path_box);

        // Record button
        let record_button = Button::builder()
            .label("Record")
            .css_classes(vec!["record-button"])
            .margin_top(32)
            .build();

        // Connect signals
        {
            let screen_btn = screen_btn.clone();
            let region_btn = region_btn.clone();
            let options = options.clone();
            screen_btn.connect_clicked(move |btn| {
                btn.add_css_class("active");
                region_btn.remove_css_class("active");
                options.borrow_mut().region = CaptureRegion::FullScreen;
            });
        }

        {
            let screen_btn = screen_btn.clone();
            let region_btn = region_btn.clone();
            let options = options.clone();
            region_btn.connect_clicked(move |btn| {
                btn.add_css_class("active");
                screen_btn.remove_css_class("active");
                options.borrow_mut().region = CaptureRegion::Selection;
            });
        }

        {
            let audio_btn = audio_btn.clone();
            let mic_btn = mic_btn.clone();
            let mute_btn = mute_btn.clone();
            let options = options.clone();
            audio_btn.connect_clicked(move |btn| {
                btn.add_css_class("active");
                mic_btn.remove_css_class("active");
                mute_btn.remove_css_class("active");
                options.borrow_mut().audio = AudioSource::System;
            });
        }

        {
            let audio_btn = audio_btn.clone();
            let mic_btn = mic_btn.clone();
            let mute_btn = mute_btn.clone();
            let options = options.clone();
            mic_btn.connect_clicked(move |btn| {
                btn.add_css_class("active");
                audio_btn.remove_css_class("active");
                mute_btn.remove_css_class("active");
                options.borrow_mut().audio = AudioSource::Microphone;
            });
        }

        {
            let audio_btn = audio_btn;
            let mic_btn = mic_btn;
            let mute_btn = mute_btn;
            let options = options.clone();
            mute_btn.connect_clicked(move |btn| {
                btn.add_css_class("active");
                audio_btn.remove_css_class("active");
                mic_btn.remove_css_class("active");
                options.borrow_mut().audio = AudioSource::None;
            });
        }

        // Add all sections
        content.append(&format_box);
        content.append(&screen_box);
        content.append(&audio_box);
        content.append(&dir_box);
        content.append(&record_button);

        container.append(&content);

        Self {
            container,
            record_button,
            options,
            dir_label: dir_path_label,
        }
    }

    pub fn widget(&self) -> &Box {
        &self.container
    }

    pub fn connect_record_clicked<F: Fn(RecordingOptions) + 'static>(&self, f: F) {
        let options = self.options.clone();
        self.record_button.connect_clicked(move |_| {
            f(options.borrow().clone());
        });
    }
}

fn create_option_button(icon_name: &str, tooltip: &str, active: bool) -> Button {
    let btn = Button::builder()
        .css_classes(vec!["option-button"])
        .tooltip_text(tooltip)
        .hexpand(true)
        .build();
    
    let icon = Image::from_icon_name(icon_name);
    btn.set_child(Some(&icon));
    
    if active {
        btn.add_css_class("active");
    }
    
    btn
}
