use gtk::prelude::*;
use gtk::{glib, Application, ApplicationWindow, Box, Orientation};
use std::cell::RefCell;
use std::rc::Rc;

use super::{RecordingState, settings_view::SettingsView, countdown_view::CountdownView, recording_view::RecordingView};
use crate::recorder::{Recorder, RecordingConfig};

#[derive(Clone)]
struct AppState {
    recorder: Option<Recorder>,
    recording_state: RecordingState,
}

pub fn build_ui(app: &Application) {
    // Create main window with modern styling
    let window = ApplicationWindow::builder()
        .application(app)
        .title("WF Recorder")
        .default_width(320)  // Slightly wider for better layout
        .default_height(520) // Taller to accommodate all settings
        .css_classes(vec!["main-window"])
        .build();

    // Main container
    let main_box = Box::new(Orientation::Vertical, 0);
    main_box.set_hexpand(true);
    main_box.set_vexpand(true);
    main_box.set_halign(gtk::Align::Fill);
    main_box.set_valign(gtk::Align::Fill);
    
    // Create all views
    let settings_view = SettingsView::new();
    let countdown_view = CountdownView::new();
    let recording_view = RecordingView::new();

    // Initially show settings view
    main_box.append(settings_view.widget());

    // State management
    let state = Rc::new(RefCell::new(AppState {
        recorder: None,
        recording_state: RecordingState::Settings,
    }));
    
    // Store main_box in Rc for sharing
    let main_box = Rc::new(main_box);

    // Handle state transitions
    {
        let state = state.clone();
        let main_box = main_box.clone();
        let settings_view = settings_view.clone();
        let countdown_view = countdown_view.clone();
        let recording_view = recording_view.clone();
        let window_clone = window.clone();

        settings_view.clone().connect_record_clicked(move |options| {
            let config = RecordingConfig {
                format: options.format,
                audio: options.audio,
                region: options.region,
                output_dir: options.output_dir,
            };

            state.borrow_mut().recorder = Some(Recorder::new(config));
            state.borrow_mut().recording_state = RecordingState::Countdown;

            let state_clone = state.clone();
            let main_box_clone = main_box.clone();
            let settings_view = settings_view.clone();
            let countdown_view = countdown_view.clone();
            let recording_view = recording_view.clone();
            let window = window_clone.clone();

            // Resize for countdown/recording
            window.set_default_size(200, 100);
            update_view(&main_box, &RecordingState::Countdown,
                       &settings_view, &countdown_view, &recording_view);

            let mut count = 3;
            countdown_view.set_countdown(count);

            glib::timeout_add_local(std::time::Duration::from_secs(1), move || {
                if count > 0 {
                    count -= 1;
                    countdown_view.set_countdown(count);
                    glib::ControlFlow::Continue
                } else {
                    let mut state = state_clone.borrow_mut();
                    if let Some(recorder) = state.recorder.as_mut() {
                        if let Err(e) = recorder.start() {
                            eprintln!("Failed to start recording: {}", e);
                            state.recorder = None;
                            state.recording_state = RecordingState::Settings;
                            window.set_default_size(320, 520);
                        } else {
                            state.recording_state = RecordingState::Recording;
                        }
                    }

                    update_view(&main_box_clone, &state.recording_state,
                              &settings_view, &countdown_view, &recording_view);

                    if state.recording_state == RecordingState::Recording {
                        start_recording_timer(&recording_view);
                    }
                    
                    glib::ControlFlow::Break
                }
            });
        });
    }

    {
        let state = state.clone();
        let main_box = main_box.clone();
        let settings_view = settings_view.clone();
        let countdown_view = countdown_view.clone();
        let recording_view = recording_view.clone();
        let window_clone = window.clone();

        countdown_view.clone().connect_cancel_clicked(move || {
            let mut state = state.borrow_mut();
            state.recorder = None;
            state.recording_state = RecordingState::Settings;
            window_clone.set_default_size(320, 520);
            update_view(&main_box, &state.recording_state,
                       &settings_view, &countdown_view, &recording_view);
        });
    }

    {
        let state = state.clone();
        let main_box = main_box.clone();
        let settings_view = settings_view.clone();
        let countdown_view = countdown_view.clone();
        let recording_view = recording_view.clone();
        let window_clone = window.clone();

        recording_view.clone().connect_stop_clicked(move || {
            let mut state = state.borrow_mut();
            if let Some(recorder) = state.recorder.as_mut() {
                let _ = recorder.stop();
            }
            state.recorder = None;
            state.recording_state = RecordingState::Settings;
            window_clone.set_default_size(320, 520);
            update_view(&main_box, &state.recording_state,
                       &settings_view, &countdown_view, &recording_view);
        });
    }

    // Add main container to window
    window.set_child(Some(main_box.as_ref()));

    // Load CSS styling
    load_css();
    
    // Set minimum window size
    window.set_size_request(200, 100);
    
    window.present();
}

fn update_view(
    container: &Box,
    state: &RecordingState,
    settings: &SettingsView,
    countdown: &CountdownView,
    recording: &RecordingView,
) {
    // Remove all children
    while let Some(child) = container.first_child() {
        container.remove(&child);
    }

    // Add appropriate view
    match state {
        RecordingState::Settings => container.append(settings.widget()),
        RecordingState::Countdown => container.append(countdown.widget()),
        RecordingState::Recording => container.append(recording.widget()),
    }
}

fn start_recording_timer(recording_view: &RecordingView) {
    let recording_view = recording_view.clone();
    let mut seconds = 0;
    
    glib::timeout_add_local(std::time::Duration::from_secs(1), move || {
        seconds += 1;
        recording_view.update_time(seconds);
        glib::ControlFlow::Continue
    });
}

fn load_css() {
    let provider = gtk::CssProvider::new();
    provider.load_from_data(include_str!("../../assets/style.css"));
    
    gtk::style_context_add_provider_for_display(
        &gtk::gdk::Display::default().expect("Could not connect to display"),
        &provider,
        gtk::STYLE_PROVIDER_PRIORITY_APPLICATION,
    );
}
