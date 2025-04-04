use gtk::{gio, glib, gdk};
use adw::subclass::prelude::*;
use adw::prelude::*;
use glib::clone;

use crate::LauncherApplication;
use crate::engine_combo_row::EngineComboRow;
use crate::iwad_combo_row::IWadComboRow;
use crate::preferences_dialog::PreferencesDialog;

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
        #[template_child]
        pub engine_comborow: TemplateChild<EngineComboRow>,
        #[template_child]
        pub iwad_comborow: TemplateChild<IWadComboRow>,

        #[template_child]
        pub prefs_dialog: TemplateChild<PreferencesDialog>,
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

            // Add show preferences shortcut
            klass.add_binding_action(gdk::Key::comma, gdk::ModifierType::CONTROL_MASK, "win.show-preferences");
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

            let obj = self.obj();

            obj.setup_actions();
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

    //-----------------------------------
    // Setup actions
    //-----------------------------------
    fn setup_actions(&self) {
        let imp = self.imp();

        // Add show preferences action
        let prefs_action = gio::ActionEntry::builder("show-preferences")
            .activate(clone!(
                #[weak(rename_to = window)] self,
                #[weak] imp,
                move |_, _, _| {
                    imp.prefs_dialog.present(Some(&window));
                }
            ))
            .build();
        // Add actions to window
        self.add_action_entries([prefs_action]);
    }
}
