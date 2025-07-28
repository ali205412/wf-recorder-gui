use gtk::prelude::*;
use gtk::{
    Box, Button, DropDown, FileChooserAction, FileChooserDialog, FlowBox, Image, Label,
    Orientation, ResponseType, StringList,
};
use std::cell::RefCell;
use std::rc::Rc;

use crate::recorder::{AudioSource, CaptureRegion, OutputFormat, RecordingConfig};

#[derive(Clone)]
pub struct SettingsView {
    container: Box,
    record_button: Button,
    options: Rc<RefCell<RecordingConfig>>,
}

impl SettingsView {
    pub fn new() -> Self {
        let container = Box::new(Orientation::Vertical, 0);
        container.add_css_class("settings-view");

        let options = Rc::new(RefCell::new(RecordingConfig::new_with_defaults()));

        // Main content box with padding
        let content = Box::new(Orientation::Vertical, 4);
        content.set_margin_start(6);
        content.set_margin_end(6);
        content.set_margin_top(6);
        content.set_margin_bottom(6);

        // Title and subtitle
        let title = Label::builder()
            .label("Normal")
            .css_classes(vec!["window-title"])
            .build();

        let subtitle = Label::builder()
            .label("MP4 • 30FPS")
            .css_classes(vec!["window-subtitle"])
            .build();

        content.append(&title);
        content.append(&subtitle);

        // FlowBox for options
        let flow_box = FlowBox::builder()
            .homogeneous(true)
            .row_spacing(2)
            .column_spacing(2)
            .selection_mode(gtk::SelectionMode::None)
            .halign(gtk::Align::Center)
            .hexpand(false)
            .css_classes(vec!["option-grid", "dynamic-grid"])
            .build();

        // Screen selection buttons
        let screen_box = Box::new(Orientation::Vertical, 0);
        let screen_icon = Image::from_icon_name("display-symbolic");
        let screen_label = Label::builder()
            .label("Screen")
            .css_classes(vec!["option-label"])
            .build();
        screen_box.append(&screen_icon);
        screen_box.append(&screen_label);

        let screen_btn = Button::builder()
            .css_classes(vec!["option-button", "active"])
            .tooltip_text("Full Screen")
            .child(&screen_box)
            .build();

        let region_box = Box::new(Orientation::Vertical, 0);
        let region_icon = Image::from_icon_name("selection-mode-symbolic");
        let region_label = Label::builder()
            .label("Region")
            .css_classes(vec!["option-label"])
            .build();
        region_box.append(&region_icon);
        region_box.append(&region_label);

        let region_btn = Button::builder()
            .css_classes(vec!["option-button"])
            .tooltip_text("Select Region")
            .child(&region_box)
            .build();

        flow_box.insert(&screen_btn, -1);
        flow_box.insert(&region_btn, -1);

        // Audio options
        let audio_box = Box::new(Orientation::Vertical, 0);
        let audio_icon = Image::from_icon_name("audio-volume-high-symbolic");
        let audio_label = Label::builder()
            .label("System")
            .css_classes(vec!["option-label"])
            .build();
        audio_box.append(&audio_icon);
        audio_box.append(&audio_label);

        let audio_btn = Button::builder()
            .css_classes(vec!["option-button"])
            .tooltip_text("System Audio")
            .child(&audio_box)
            .build();

        let mic_box = Box::new(Orientation::Vertical, 0);
        let mic_icon = Image::from_icon_name("audio-input-microphone-symbolic");
        let mic_label = Label::builder()
            .label("Mic")
            .css_classes(vec!["option-label"])
            .build();
        mic_box.append(&mic_icon);
        mic_box.append(&mic_label);

        let mic_btn = Button::builder()
            .css_classes(vec!["option-button"])
            .tooltip_text("Microphone")
            .child(&mic_box)
            .build();

        flow_box.insert(&audio_btn, -1);
        flow_box.insert(&mic_btn, -1);

        // Mute button
        let mute_box = Box::new(Orientation::Vertical, 0);
        let mute_icon = Image::from_icon_name("audio-volume-muted-symbolic");
        let mute_label = Label::builder()
            .label("Mute")
            .css_classes(vec!["option-label"])
            .build();
        mute_box.append(&mute_icon);
        mute_box.append(&mute_label);

        let mute_btn = Button::builder()
            .css_classes(vec!["option-button", "active"])
            .tooltip_text("No Audio")
            .child(&mute_box)
            .build();

        flow_box.insert(&mute_btn, -1);

        // Format selector
        let format_box = Box::new(Orientation::Horizontal, 4);
        format_box.set_halign(gtk::Align::Fill);
        format_box.set_margin_top(6);

        let format_model = StringList::new(&[]);
        for (_, desc) in OutputFormat::all() {
            format_model.append(desc);
        }

        let format_dropdown = DropDown::builder()
            .model(&format_model)
            .selected(1) // MP4 by default
            .css_classes(vec!["format-dropdown"])
            .build();

        let subtitle_clone = subtitle.clone();
        let options_clone = options.clone();
        format_dropdown.connect_selected_notify(move |dropdown| {
            let idx = dropdown.selected();
            let format = OutputFormat::all()[idx as usize].0;
            options_clone.borrow_mut().format = format;

            let format_text = match format {
                OutputFormat::WebM => "WebM • 30FPS",
                OutputFormat::Mp4 => "MP4 • 30FPS",
                OutputFormat::Mkv => "MKV • 30FPS",
            };
            subtitle_clone.set_text(format_text);
        });

        format_box.append(&format_dropdown);

        // display output section
        let output_available_box = Box::new(Orientation::Horizontal, 4);
        output_available_box.set_halign(gtk::Align::Fill);
        output_available_box.set_margin_top(6);
        output_available_box.set_size_request(-1, 40); // Fixed height
        output_available_box.set_visible(true); // Always visible

        // Store outputs for later use
        let available_outputs = options.borrow().get_available_outputs().clone();

        // Only create dropdown if we have multiple outputs
        let output_dropdown_option = if available_outputs.len() > 1 {
            let output_available_model = StringList::new(&[]);
            for output in &available_outputs {
                output_available_model.append(&output.display_name());
            }

            let output_available_dropdown = DropDown::builder()
                .model(&output_available_model)
                .selected(0)
                .css_classes(vec!["output-dropdown"])
                .build();

            let options_clone = options.clone();
            output_available_dropdown.connect_selected_notify(move |dropdown| {
                let idx = dropdown.selected() as usize;
                options_clone.borrow_mut().set_selected_output(idx);
            });

            Some(output_available_dropdown)
        } else {
            None
        };

        let update_output_visibility = {
            let output_box = output_available_box.clone();
            let options = options.clone();
            let dropdown = output_dropdown_option.clone();

            move || {
                // Clear the box first
                while let Some(child) = output_box.first_child() {
                    output_box.remove(&child);
                }

                let should_display = options.borrow().has_multiple_outputs()
                    && matches!(options.borrow().region, CaptureRegion::FullScreen);

                if should_display {
                    if let Some(ref dropdown) = dropdown {
                        output_box.append(dropdown);
                    }
                }
            }
        };
        // Set initial state
        update_output_visibility();

        // Output directory selector
        let path_box = Box::new(Orientation::Horizontal, 4);
        path_box.set_halign(gtk::Align::Fill);
        path_box.set_margin_top(6);
        path_box.set_spacing(4);

        let path_label = Label::builder()
            .label(options.borrow().output_dir.to_string_lossy().to_string())
            .halign(gtk::Align::Start)
            .hexpand(true)
            .css_classes(vec!["path-label"])
            .build();

        let path_btn = Button::builder()
            .label("Browse...")
            .css_classes(vec!["path-button"])
            .build();

        let options_clone = options.clone();
        let path_label_clone = path_label.clone();
        path_btn.connect_clicked(move |_| {
            let dialog = FileChooserDialog::new(
                Some("Choose Output Directory"),
                None::<&gtk::Window>,
                FileChooserAction::SelectFolder,
                &[
                    ("Cancel", ResponseType::Cancel),
                    ("Select", ResponseType::Accept),
                ],
            );

            let _ = dialog.set_current_folder(Some(&gio::File::for_path(
                &options_clone.borrow().output_dir,
            )));

            let options = options_clone.clone();
            let path_label = path_label_clone.clone();
            dialog.connect_response(move |dialog, response| {
                if response == ResponseType::Accept {
                    if let Some(folder) = dialog.file() {
                        if let Some(path) = folder.path() {
                            path_label.set_text(path.to_string_lossy().as_ref());
                            options.borrow_mut().output_dir = path.clone();
                        }
                    }
                    dialog.close();
                }
            });

            dialog.show();
        });

        path_box.append(&path_label);
        path_box.append(&path_btn);

        // Record button
        let record_button = Button::builder()
            .label("Record")
            .css_classes(vec!["record-button"])
            .margin_top(6)
            .build();

        // Connect signals
        {
            let screen_btn = screen_btn.clone();
            let region_btn = region_btn.clone();
            let options = options.clone();
            let update_visibility = update_output_visibility.clone();
            screen_btn.connect_clicked(move |btn| {
                btn.add_css_class("active");
                region_btn.remove_css_class("active");
                options.borrow_mut().region = CaptureRegion::FullScreen;
                update_visibility();
            });
        }

        {
            let screen_btn = screen_btn.clone();
            let region_btn = region_btn.clone();
            let options = options.clone();
            let update_visibility = update_output_visibility.clone();
            region_btn.connect_clicked(move |btn| {
                btn.add_css_class("active");
                screen_btn.remove_css_class("active");
                options.borrow_mut().region = CaptureRegion::Selection;
                update_visibility();
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
        content.append(&flow_box);
        content.append(&format_box);
        content.append(&output_available_box);
        content.append(&path_box);
        content.append(&record_button);

        container.append(&content);

        Self {
            container,
            record_button,
            options,
        }
    }

    pub fn widget(&self) -> &Box {
        &self.container
    }

    pub fn connect_record_clicked<F: Fn(RecordingConfig) + 'static>(&self, f: F) {
        let options = self.options.clone();
        self.record_button.connect_clicked(move |_| {
            let mut config = options.borrow().clone();
            config.ensure_output_selected();
            f(config);
        });
    }
}
