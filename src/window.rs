use std::cell::OnceCell;

use gtk::{gio, glib, gdk};
use adw::subclass::prelude::*;
use adw::prelude::*;
use glib::clone;

use crate::APP_ID;
use crate::LauncherApplication;
use crate::engine_combo_row::EngineComboRow;
use crate::iwad_combo_row::IWadComboRow;
use crate::file_select_row::FileSelectRow;
use crate::preferences_dialog::PreferencesDialog;
use crate::utils::env_expand;

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
        pub pwad_row: TemplateChild<FileSelectRow>,

        #[template_child]
        pub launch_button: TemplateChild<gtk::Button>,

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

            // Add reset widgets shortcut
            klass.add_binding_action(gdk::Key::R, gdk::ModifierType::CONTROL_MASK, "win.reset-widgets");

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
    impl WindowImpl for LauncherWindow {
        //-----------------------------------
        // Window close handler
        //-----------------------------------
        fn close_request(&self) -> glib::Propagation {
            self.obj().save_gsettings();

            glib::Propagation::Proceed
        }
    }
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

        // Preferences window IWAD folder property notify signal
        imp.prefs_dialog.connect_iwad_folder_notify(clone!(
            #[weak] imp,
            move |prefs_dialog| {
                imp.iwad_row.init_from_folder(&env_expand(&prefs_dialog.iwad_folder()));
        
                imp.launch_button.set_sensitive(imp.engine_row.selected_item().is_some() && imp.iwad_row.selected_iwad().is_some());
            }
        ));

        // Preferences window PWAD folder property notify signal
        imp.prefs_dialog.connect_pwad_folder_notify(clone!(
            #[weak] imp,
            move |prefs_dialog| {
                imp.pwad_row.set_initial_folder(prefs_dialog.pwad_folder());
            }
        ));

        // IWAD combo selected item property notify signal
        imp.iwad_row.connect_selected_item_notify(clone!(
            #[weak] imp,
            move |iwad_row| {
                if let Some(selected_iwad) = iwad_row.selected_iwad() {
                    imp.engine_row.init_for_iwad(selected_iwad.flag());

                    imp.launch_button.set_sensitive(imp.engine_row.selected_item().is_some() && imp.iwad_row.selected_iwad().is_some());
                }
            }
        ));
    }

    //-----------------------------------
    // Gsetting default value helper function
    //-----------------------------------
    pub fn gsetting_default_value(gsettings: &gio::Settings, key: &str) -> String {
        gsettings.default_value(key).unwrap().to_string().replace('\'', "")
    }

    //---------------------------------------
    // Set gsetting helper function
    //---------------------------------------
    pub fn set_gsetting<T: FromVariant + ToVariant + PartialEq>(gsettings: &gio::Settings, key: &str, value: &T) {
        let default: T = gsettings.default_value(key)
            .expect("Could not get gsettings default value")
            .get::<T>()
            .expect("Could not retrieve value from variant");

        if !(default == *value && default == gsettings.get(key)) {
            gsettings.set(key, value.to_variant()).unwrap();
        }
    }

    //-----------------------------------
    // Load gsettings
    //-----------------------------------
    fn load_gsettings(&self) {
        let imp = self.imp();

        // Create gsettings
        let gsettings = gio::Settings::new(APP_ID);

        // Init preferences window
        imp.prefs_dialog.set_iwad_folder(gsettings.string("iwad-folder"));
        imp.prefs_dialog.set_pwad_folder(gsettings.string("pwad-folder"));

        imp.prefs_dialog.set_iwad_default_folder(Self::gsetting_default_value(&gsettings,"iwad-folder"));
        imp.prefs_dialog.set_pwad_default_folder(Self::gsetting_default_value(&gsettings,"pwad-folder"));

        // Init main window
        imp.engine_row.set_selected_engine_name(&gsettings.string("selected-engine"));
        imp.iwad_row.set_selected_iwad_file(&gsettings.string("selected-iwad"));
        imp.pwad_row.set_files(gsettings.strv("pwad-files").into_iter().map(String::from).collect::<Vec<String>>());

        // Store gsettings
        imp.gsettings.set(gsettings).unwrap();
    }

    //-----------------------------------
    // Save gsettings
    //-----------------------------------
    fn save_gsettings(&self) {
        let imp = self.imp();

        let gsettings = imp.gsettings.get().unwrap();

        // Get selected engine
        let selected_engine = imp.engine_row.selected_engine()
            .map_or("".to_string(), |engine| engine.name());

        // Get selected IWAD
        let selected_iwad = imp.iwad_row.selected_iwad()
            .map_or("".to_string(), |iwad| iwad.iwad());

        // Save main window settings
        Self::set_gsetting(gsettings, "selected-engine", &selected_engine);
        Self::set_gsetting(gsettings, "selected-iwad", &selected_iwad);
        Self::set_gsetting(gsettings, "pwad-files", &imp.pwad_row.files());

        // Save preferences window settings
        let prefs = imp.prefs_dialog.imp();

        Self::set_gsetting(gsettings, "iwad-folder", &prefs.iwad_row.files().get(0).cloned().unwrap_or_default());
        Self::set_gsetting(gsettings, "pwad-folder", &prefs.pwad_row.files().get(0).cloned().unwrap_or_default());
    }

    //-----------------------------------
    // Setup actions
    //-----------------------------------
    fn setup_actions(&self) {
        let imp = self.imp();

        // Add reset widgets action
        let reset_action = gio::ActionEntry::builder("reset-widgets")
            .activate(clone!(
                #[weak(rename_to = window)] self,
                #[weak] imp,
                move |_, _, _| {
                    let reset_dialog = adw::AlertDialog::builder()
                        .heading("Reset Parameters?")
                        .body("Reset all parameters to their default values.")
                        .default_response("reset")
                        .build();

                    reset_dialog.add_responses(&[("cancel", "_Cancel"), ("reset", "_Reset")]);
                    reset_dialog.set_response_appearance("reset", adw::ResponseAppearance::Destructive);
        
                    reset_dialog.choose(
                        &window,
                        None::<&gio::Cancellable>,
                        clone!(
                            #[weak] imp,
                            move |response| {
                                if response == "reset" {
                                    imp.engine_row.set_selected(0);
                                    imp.iwad_row.set_selected(0);
                                    imp.pwad_row.reset_to_default();
                                }
                            }
                        )
                    );
                }
            ))
            .build();

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
        self.add_action_entries([reset_action, prefs_action]);
    }
}
