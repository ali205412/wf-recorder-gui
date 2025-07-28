use gtk::prelude::*;
use gtk::{glib, Application, ApplicationWindow, Box, Orientation};
use std::cell::{Cell, RefCell};
use std::rc::Rc;
use std::time::Duration;

use super::{
    countdown_view::CountdownView, recording_view::RecordingView, settings_view::SettingsView,
    RecordingState,
};
use crate::recorder::{CaptureRegion, Recorder};

struct AppState {
    recorder: Option<Recorder>,
    timer_running: Rc<Cell<bool>>,
    recording_state: RecordingState,
}

pub fn build_ui(app: &Application) {
    let window = ApplicationWindow::builder()
        .application(app)
        .title("WF Recorder")
        .resizable(false)
        .css_classes(vec!["main-window"])
        .default_width(320)
        .default_height(320)
        .build();

    let main_box = Box::new(Orientation::Vertical, 0);
    main_box.set_hexpand(false);
    main_box.set_vexpand(false);
    main_box.set_halign(gtk::Align::Fill);
    main_box.set_valign(gtk::Align::Fill);

    let settings_view = SettingsView::new();
    let countdown_view = CountdownView::new();
    let recording_view = RecordingView::new();

    main_box.append(settings_view.widget());

    let state = Rc::new(RefCell::new(AppState {
        recorder: None,
        timer_running: Rc::new(Cell::new(false)),
        recording_state: RecordingState::Settings,
    }));

    let timer_id = Rc::new(RefCell::new(None::<glib::SourceId>));
    let main_box = Rc::new(main_box);

    {
        let state = state.clone();
        let main_box = main_box.clone();
        let settings_view = settings_view.clone();
        let countdown_view = countdown_view.clone();
        let recording_view = recording_view.clone();
        let window_clone = window.clone();
        let timer_id = timer_id.clone();

        settings_view.clone().connect_record_clicked(move |config| {
            {
                let mut state = state.borrow_mut();
                state.recorder = Some(Recorder::new(config.clone()));
                state.recording_state = RecordingState::Countdown;
                if let Some(id) = timer_id.borrow_mut().take() {
                    id.remove();
                }
            }

            recording_view.reset_time();

            let state_clone = state.clone();
            let main_box_clone = main_box.clone();
            let settings_view = settings_view.clone();
            let countdown_view = countdown_view.clone();
            let recording_view = recording_view.clone();
            let timer_id = timer_id.clone();
            let window = window_clone.clone();

            window.set_default_size(180, 120);
            update_view(
                &main_box,
                &RecordingState::Countdown,
                &settings_view,
                &countdown_view,
                &recording_view,
            );

            let mut count = 3;
            countdown_view.set_countdown(count);

            // For region selection, we start recording immediately after countdown
            // For fullscreen, we use the countdown
            let delay = match config.region {
                CaptureRegion::Selection => Duration::from_millis(100),
                CaptureRegion::FullScreen => Duration::from_secs(1),
            };

            glib::timeout_add_local(delay, move || {
                if count > 0 && matches!(config.region, CaptureRegion::FullScreen) {
                    count -= 1;
                    countdown_view.set_countdown(count);
                    glib::ControlFlow::Continue
                } else {
                    let mut state = state_clone.borrow_mut();
                    if let Some(recorder) = state.recorder.as_mut() {
                        if let Err(e) = recorder.start() {
                            eprintln!("Failed to start recording: {e}");
                            state.recorder = None;
                            state.recording_state = RecordingState::Settings;
                            window.set_default_size(180, 320);
                        } else {
                            state.recording_state = RecordingState::Recording;
                            state.timer_running.set(true);
                        }
                    }

                    update_view(
                        &main_box_clone,
                        &state.recording_state,
                        &settings_view,
                        &countdown_view,
                        &recording_view,
                    );

                    if state.recording_state == RecordingState::Recording {
                        // Start new timer
                        let id =
                            start_recording_timer(&recording_view, state.timer_running.clone());
                        *timer_id.borrow_mut() = Some(id);
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
        let timer_id = timer_id.clone();

        countdown_view.clone().connect_cancel_clicked(move || {
            let mut state = state.borrow_mut();
            state.recorder = None;
            state.recording_state = RecordingState::Settings;
            if let Some(id) = timer_id.borrow_mut().take() {
                id.remove();
            }
            recording_view.reset_time();
            window_clone.set_default_size(180, 320);
            update_view(
                &main_box,
                &state.recording_state,
                &settings_view,
                &countdown_view,
                &recording_view,
            );
        });
    }

    {
        let state = state.clone();
        let main_box = main_box.clone();
        let settings_view = settings_view.clone();
        let countdown_view = countdown_view.clone();
        let recording_view = recording_view.clone();
        let window_clone = window.clone();
        let timer_id = timer_id.clone();

        recording_view.clone().connect_stop_clicked(move || {
            let mut state = state.borrow_mut();
            if let Some(recorder) = state.recorder.as_mut() {
                let _ = recorder.stop();
            }
            state.recorder = None;
            state.timer_running.set(false);
            state.recording_state = RecordingState::Settings;
            if let Some(id) = timer_id.borrow_mut().take() {
                id.remove();
            }
            recording_view.reset_time();
            window_clone.set_default_size(180, 320);
            update_view(
                &main_box,
                &state.recording_state,
                &settings_view,
                &countdown_view,
                &recording_view,
            );
        });
    }

    window.set_child(Some(main_box.as_ref()));
    load_css();
    window.present();
}

fn update_view(
    container: &Box,
    state: &RecordingState,
    settings: &SettingsView,
    countdown: &CountdownView,
    recording: &RecordingView,
) {
    while let Some(child) = container.first_child() {
        container.remove(&child);
    }

    match state {
        RecordingState::Settings => container.append(settings.widget()),
        RecordingState::Countdown => container.append(countdown.widget()),
        RecordingState::Recording => container.append(recording.widget()),
    }
}

fn start_recording_timer(
    recording_view: &RecordingView,
    is_running: Rc<Cell<bool>>,
) -> glib::SourceId {
    let recording_view = recording_view.clone();
    let seconds = Rc::new(RefCell::new(0u32));

    glib::timeout_add_local(Duration::from_secs(1), move || {
        if is_running.get() {
            let mut seconds = seconds.borrow_mut();
            *seconds += 1;
            recording_view.update_time(*seconds);
            glib::ControlFlow::Continue
        } else {
            glib::ControlFlow::Break
        }
    })
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
