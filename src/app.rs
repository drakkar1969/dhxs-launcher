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
    pub struct LauncherApp;

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
            let window = application.active_window().map_or_else(|| {
                AppWindow::new(&application).upcast()
            }, |window| window);

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
    pub fn new(application_id: &str, flags: gio::ApplicationFlags) -> Self {
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
                    .application_icon("dhxs-launcher")
                    .developer_name("draKKar1969")
                    .version(env!("CARGO_PKG_VERSION"))
                    .website("https://github.com/drakkar1969/dhxs-launcher/")
                    .copyright("© 2025 draKKar1969")
                    .license_type(gtk::License::Gpl30)
                    .build();

                about_dialog.add_link("Doom Wiki", "https://doomwiki.org/wiki/Entryway");
                about_dialog.add_link("DoomWorld /idgames", "https://www.doomworld.com/idgames/");
                about_dialog.add_link("DoomWorld /Cacowards", "https://www.doomworld.com/cacowards/");
                about_dialog.add_link("Classic DOOM", "https://classicdoom.com/");

                about_dialog.present(Some(&window));
            })
            .build();

        self.add_action_entries([quit_action, about_action]);

        self.set_accels_for_action("app.quit-app", &["<ctrl>Q"]);
    }
}
