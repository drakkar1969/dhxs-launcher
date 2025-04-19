use gtk::{gio, glib};
use adw::prelude::*;
use adw::subclass::prelude::*;

use crate::window::AppWindow;

//------------------------------------------------------------------------------
// MODULE: LauncherApp
//------------------------------------------------------------------------------
mod imp {
    use super::*;

    //-----------------------------------
    // Private structure
    //-----------------------------------
    #[derive(Default)]
    pub struct LauncherApp {}

    //-----------------------------------
    // Subclass
    //-----------------------------------
    #[glib::object_subclass]
    impl ObjectSubclass for LauncherApp {
        const NAME: &'static str = "LauncherApp";
        type Type = super::LauncherApp;
        type ParentType = adw::Application;
    }

    impl ObjectImpl for LauncherApp {
        //-----------------------------------
        // Constructor
        //-----------------------------------
        fn constructed(&self) {
            self.parent_constructed();

            self.obj().setup_actions();
        }
    }

    impl ApplicationImpl for LauncherApp {
        //-----------------------------------
        // Activate handler
        //-----------------------------------
        fn activate(&self) {
            let application = self.obj();

            // Show main window
            let window = if let Some(window) = application.active_window() {
                window
            } else {
                let window = AppWindow::new(&application);
                window.upcast()
            };

            window.present();
        }
    }

    impl GtkApplicationImpl for LauncherApp {}
    impl AdwApplicationImpl for LauncherApp {}
}

//------------------------------------------------------------------------------
// IMPLEMENTATION: LauncherApp
//------------------------------------------------------------------------------
glib::wrapper! {
    pub struct LauncherApp(ObjectSubclass<imp::LauncherApp>)
        @extends gio::Application, gtk::Application, adw::Application,
        @implements gio::ActionGroup, gio::ActionMap;
}

impl LauncherApp {
    //-----------------------------------
    // New function
    //-----------------------------------
    pub fn new(application_id: &str, flags: &gio::ApplicationFlags) -> Self {
        glib::Object::builder()
            .property("application-id", application_id)
            .property("flags", flags)
            .build()
    }

    //-----------------------------------
    // Setup actions
    //-----------------------------------
    fn setup_actions(&self) {
        let quit_action = gio::ActionEntry::builder("quit-app")
            .activate(move |app: &Self, _, _| app.quit())
            .build();

        let about_action = gio::ActionEntry::builder("show-about")
            .activate(move |app: &Self, _, _| {
                let window = app.active_window()
                    .expect("Could not retrieve active window");

                let about_dialog = adw::AboutDialog::builder()
                    .application_name("DHXS-Launcher")
                    // .application_icon("zandronum")
                    .developer_name("draKKar1969")
                    .version(env!("CARGO_PKG_VERSION"))
                    // .website("https://github.com/drakkar1969/zandronum-launcher/")
                    .developers(vec!["draKKar1969"])
                    .designers(vec!["draKKar1969"])
                    .copyright("© 2025 draKKar1969")
                    .license_type(gtk::License::Gpl30)
                    .build();

                about_dialog.present(Some(&window));
            })
            .build();

        self.add_action_entries([quit_action, about_action]);

        self.set_accels_for_action("app.quit-app", &["<ctrl>Q"]);
    }
}
