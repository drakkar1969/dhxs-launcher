mod app;
mod window;
mod engine_combo_row;
mod engine_object;
mod engine_settings;
mod iwad_combo_row;
mod iwad_object;
mod pwad_select_row;
mod preferences_dialog;
mod folder_select_row;
mod utils;
mod iwad_data;
mod engine_data;
mod graphics_data;

use gtk::{gio, glib};
use gtk::prelude::*;

use app::LauncherApp;

const APP_ID: &str = "com.github.DHXS-Launcher";

fn main() -> glib::ExitCode {
    // Register and include resources
    gio::resources_register_include!("resources.gresource")
        .expect("Failed to register resources");

    // Run app
    let app = LauncherApp::new(APP_ID, gio::ApplicationFlags::empty());

    app.run()
}
