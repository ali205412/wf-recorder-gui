mod ui;
mod recorder;
mod config;
mod audio;

use anyhow::Result;
use gtk::prelude::*;
use gtk::Application;

const APP_ID: &str = "org.wf.recorder.gui";

fn main() -> Result<()> {
    // Initialize GTK application
    let app = Application::builder()
        .application_id(APP_ID)
        .build();

    app.connect_activate(ui::build_ui);
    app.run();

    Ok(())
}
