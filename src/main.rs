mod app;
mod window;

use gtk::{gio, glib};
use gtk::prelude::*;

use app::LauncherApplication;

const APP_ID: &str = "com.github.D-Launcher";

fn main() -> glib::ExitCode {
    // Register and include resources
    gio::resources_register_include!("resources.gresource")
        .expect("Failed to register resources");

    // Run app
    let app = LauncherApplication::new(APP_ID, &gio::ApplicationFlags::empty());

    app.run()
}
