use gtk::{gio, glib};
use adw::subclass::prelude::*;
use adw::prelude::*;

use crate::LauncherApplication;

//------------------------------------------------------------------------------
// MODULE: LauncherWindow
//------------------------------------------------------------------------------
mod imp {
    use super::*;

    //-----------------------------------
    // Private structure
    //-----------------------------------
    #[derive(Default, gtk::CompositeTemplate)]
    #[template(resource = "/com/github/D-Launcher/ui/window.ui")]
    pub struct LauncherWindow {
    }

    //-----------------------------------
    // Subclass
    //-----------------------------------
    #[glib::object_subclass]
    impl ObjectSubclass for LauncherWindow {
        const NAME: &'static str = "LauncherWindow";
        type Type = super::LauncherWindow;
        type ParentType = adw::ApplicationWindow;

        fn class_init(klass: &mut Self::Class) {
            klass.bind_template();
        }

        fn instance_init(obj: &glib::subclass::InitializingObject<Self>) {
            obj.init_template();
        }
    }

    impl ObjectImpl for LauncherWindow {
        //-----------------------------------
        // Constructor
        //-----------------------------------
        fn constructed(&self) {
            self.parent_constructed();
        }
    }

    impl WidgetImpl for LauncherWindow {}
    impl WindowImpl for LauncherWindow {}
    impl ApplicationWindowImpl for LauncherWindow {}
    impl AdwApplicationWindowImpl for LauncherWindow {}
}

//------------------------------------------------------------------------------
// IMPLEMENTATION: LauncherWindow
//------------------------------------------------------------------------------
glib::wrapper! {
    pub struct LauncherWindow(ObjectSubclass<imp::LauncherWindow>)
        @extends gtk::Widget, gtk::Window, gtk::ApplicationWindow, adw::ApplicationWindow,
        @implements gio::ActionGroup, gio::ActionMap, gtk::Accessible, gtk::Buildable,
                    gtk::ConstraintTarget, gtk::Native, gtk::Root, gtk::ShortcutManager;
}

impl LauncherWindow {
    //-----------------------------------
    // New function
    //-----------------------------------
    pub fn new(app: &LauncherApplication) -> Self {
        glib::Object::builder().property("application", app).build()
    }
}
