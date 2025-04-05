use std::cell::OnceCell;

use gtk::{gio, glib, gdk};
use adw::subclass::prelude::*;
use adw::prelude::*;
use glib::clone;

use crate::APP_ID;
use crate::LauncherApplication;
use crate::engine_combo_row::EngineComboRow;
use crate::iwad_combo_row::IWadComboRow;
use crate::preferences_dialog::PreferencesDialog;
use crate::utils::{env_expand, gsetting_default_value};

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
        pub engine_row: TemplateChild<EngineComboRow>,
        #[template_child]
        pub iwad_row: TemplateChild<IWadComboRow>,

        #[template_child]
        pub prefs_dialog: TemplateChild<PreferencesDialog>,

        pub gsettings: OnceCell<gio::Settings>,
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

            obj.setup_widgets();

            obj.setup_signals();

            obj.load_gsettings();

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
    // Setup widgets
    //-----------------------------------
    fn setup_widgets(&self) {
        // Set initial focus on engine combo row
        self.imp().engine_row.get().grab_focus();
    }

    //-----------------------------------
    // Setup signals
    //-----------------------------------
    fn setup_signals(&self) {
        let imp = self.imp();

        // Preferences window IWAD folders property notify signal
        imp.prefs_dialog.connect_iwad_folder_notify(clone!(
            #[weak] imp,
            move |prefs_dialog| {
                imp.iwad_row.init_from_folder(&prefs_dialog.iwad_folder());
        
    //         imp.launch_button.set_sensitive(imp.iwad_comborow.selected_iwad().is_some());
            }
        ));
    }

    //-----------------------------------
    // Load gsettings
    //-----------------------------------
    fn load_gsettings(&self) {
        let imp = self.imp();

        // Create gsettings
        let gsettings = gio::Settings::new(APP_ID);

        // Init preferences window
        imp.prefs_dialog.set_iwad_folder(env_expand(&gsettings.string("iwad-folder")));

        imp.prefs_dialog.set_iwad_default_folder(env_expand(&gsetting_default_value(&gsettings,"iwad-folder")));

        // Store gsettings
        imp.gsettings.set(gsettings).unwrap();
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
