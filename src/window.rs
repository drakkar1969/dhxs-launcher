use std::cell::{Cell, OnceCell};
use std::path::Path;
use std::process::Command;
use std::collections::HashMap;

use gtk::{gio, glib, gdk, pango};
use adw::subclass::prelude::*;
use adw::prelude::*;
use glib::clone;

use crate::APP_ID;
use crate::LauncherApplication;
use crate::engine_combo_row::EngineComboRow;
use crate::engine_object::EngineObject;
use crate::iwad_combo_row::IWadComboRow;
use crate::pwad_select_row::PWadSelectRow;
use crate::preferences_dialog::PreferencesDialog;
use crate::utils::env_expand;
use crate::iwad_data::{IWadID, IWadData, IWAD_HASHMAP};
use crate::engine_data::ENGINE_ARRAY;
use crate::graphics_data::GRAPHICS_MAP;

//------------------------------------------------------------------------------
// CONST VARIABLES
//------------------------------------------------------------------------------
const GRAPHICS_PATH: &str = "/usr/share/d-launcher/graphics/";

const IWAD_PATHS: [&str; 1] = ["/usr/share/d-launcher/iwads"];

//------------------------------------------------------------------------------
// ENUM: LaunchResult
//------------------------------------------------------------------------------
enum LaunchResult {
    Success,
    Error(String)
}

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
        pub(super) engine_row: TemplateChild<EngineComboRow>,
        #[template_child]
        pub(super) graphics_row: TemplateChild<adw::SwitchRow>,

        #[template_child]
        pub(super) iwad_row: TemplateChild<IWadComboRow>,
        #[template_child]
        pub(super) pwad_row: TemplateChild<PWadSelectRow>,
        #[template_child]
        pub(super) switches_row: TemplateChild<adw::EntryRow>,

        #[template_child]
        pub(super) switches_grid: TemplateChild<gtk::Grid>,

        #[template_child]
        pub(super) launch_button: TemplateChild<gtk::Button>,

        #[template_child]
        pub(super) prefs_dialog: TemplateChild<PreferencesDialog>,

        pub gsettings: OnceCell<gio::Settings>,

        pub iwad_hashmap: OnceCell<HashMap<u32, IWadData>>,
        pub engine_vec: OnceCell<Vec<EngineObject>>,
        pub graphics_map: OnceCell<HashMap<IWadID, Vec<String>>>,

        pub graphics_installed: Cell<bool>,
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

            // Add launch Doom shortcuts
            klass.add_binding_action(gdk::Key::Return, gdk::ModifierType::CONTROL_MASK, "win.launch-doom");
            klass.add_binding_action(gdk::Key::KP_Enter, gdk::ModifierType::CONTROL_MASK, "win.launch-doom");
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

            obj.setup_data();

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
    // Setup data
    //-----------------------------------
    fn setup_data(&self) {
        let imp = self.imp();

        // Init IWAD data
        imp.iwad_hashmap.set(HashMap::from(IWAD_HASHMAP)).unwrap();

        // Init engine data
        imp.engine_vec.set(
            ENGINE_ARRAY.into_iter()
                .map(|data| EngineObject::new(&data))
                .collect::<Vec<EngineObject>>()
        ).unwrap();

        // Init graphics data
        imp.graphics_map.set(
            GRAPHICS_MAP.into_iter()
                .map(|(id, files)| (id, files.split(' ').map(String::from).collect()))
                .collect::<HashMap<IWadID, Vec<String>>>()
        ).unwrap();
    }

    //-----------------------------------
    // Label helper functions
    //-----------------------------------
    fn key_label(&self, key: &str) -> gtk::Label {
        let label = gtk::Label::new(Some(key));
        label.set_vexpand(true);
        label.set_xalign(0.0);
        label.set_yalign(0.0);
        label.set_can_focus(false);
        label.set_selectable(true);
        label.set_css_classes(&["heading"]);

        label
    }

    fn value_label(&self, value: &str) -> gtk::Label {
        let label = gtk::Label::new(Some(value));

        label.set_valign(gtk::Align::Center);
        label.set_xalign(0.0);
        label.set_yalign(0.0);
        label.set_can_focus(false);
        label.set_wrap_mode(pango::WrapMode::Word);
        label.set_wrap(true);
        label.set_width_chars(45);
        label.set_max_width_chars(45);

        label
    }

    //-----------------------------------
    // Setup widgets
    //-----------------------------------
    fn setup_widgets(&self) {
        let imp = self.imp();

        // Populate switches popover
        [
            ("-fast", "Increase the speed and attack rate of monsters, requires the -warp parameter"),
            ("-nomonsters", "Disable spawning of monsters, requires the -warp parameter."),
            ("-nomusic", "Disable background music"),
            ("-nosfx", "Disable sound effects"),
            ("-nosound", "Disable music and sound effects"),
            ("-respawn", "Monsters return a few seconds after being killed, requires the -warp parameter"),
            ("-skill s", "Select difficulty level s (1 to 5), will warp to the first level of the game (if no other -warp parameter is specified)"),
            ("-warp m\n-warp e m", "Start the game on level m (1 to 32) (Doom2)\nStart the game on episode e (1 to 4) map m (1 to 9) (Doom1)"),
            ("-width x -height y", "Specify the desired screen resolution")
        ]
        .iter()
        .enumerate()
        .for_each(|(i, (key, value))| {
            imp.switches_grid.attach(&self.key_label(key), 0, i as i32, 1, 1);
            imp.switches_grid.attach(&self.value_label(value), 1, i as i32, 1, 1);
        });

        // Set graphics package installed variable if 'dlauncher-hires-graphics' package is installed
        if Path::new(GRAPHICS_PATH).try_exists().unwrap_or_default() {
            imp.graphics_installed.set(true);
        }

        // Set initial focus on engine combo row
        imp.engine_row.get().grab_focus();
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
                let hash_map = imp.iwad_hashmap.get().unwrap();

                imp.iwad_row.init(hash_map, &IWAD_PATHS, &env_expand(&prefs_dialog.iwad_folder()));
        
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

        // Engine combo selected item property signal
        imp.engine_row.connect_selected_item_notify(clone!(
            #[weak] imp,
            move |engine_row| {
                let graphics_map = imp.graphics_map.get().unwrap();

                let engine_hires = engine_row.selected_engine()
                    .map(|engine| engine.hires())
                    .unwrap_or_default();

                let iwad_id = imp.iwad_row.selected_iwad()
                    .map(|iwad| iwad.id())
                    .unwrap_or_default();

                imp.graphics_row.set_sensitive(
                    imp.graphics_installed.get() &&
                    engine_hires &&
                    graphics_map.get(&iwad_id).is_some()
                );
            }
        ));

        // IWAD combo selected item property notify signal
        imp.iwad_row.connect_selected_item_notify(clone!(
            #[weak] imp,
            move |iwad_row| {
                let graphics_map = imp.graphics_map.get().unwrap();

                let engine = imp.engine_row.selected_engine();
                let engine_hires = imp.engine_row.selected_engine()
                    .map(|engine| engine.hires())
                    .unwrap_or_default();

                let iwad = iwad_row.selected_iwad();
                let iwad_id = iwad_row.selected_iwad()
                    .map(|iwad| iwad.id())
                    .unwrap_or_default();

                imp.graphics_row.set_sensitive(
                    imp.graphics_installed.get() &&
                    engine_hires &&
                    graphics_map.get(&iwad_id).is_some()
                );

                imp.launch_button.set_sensitive(engine.is_some() && iwad.is_some());

                if let Some(iwad) = iwad {
                    let engines = imp.engine_vec.get().unwrap();

                    imp.engine_row.init_for_iwad(engines, iwad.id());
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
        imp.switches_row.set_text(&gsettings.string("extra-switches"));
        imp.graphics_row.set_active(gsettings.boolean("hires-graphics"));

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
            .map_or("".to_string(), |iwad| iwad.filename());

        // Save main window settings
        Self::set_gsetting(gsettings, "selected-engine", &selected_engine);
        Self::set_gsetting(gsettings, "selected-iwad", &selected_iwad);
        Self::set_gsetting(gsettings, "pwad-files", &imp.pwad_row.files());
        Self::set_gsetting(gsettings, "extra-switches", &imp.switches_row.text().to_string());
        Self::set_gsetting(gsettings, "hires-graphics", &imp.graphics_row.is_active());

        // Save preferences window settings
        Self::set_gsetting(gsettings, "iwad-folder", &imp.prefs_dialog.iwad_folder());
        Self::set_gsetting(gsettings, "pwad-folder", &imp.prefs_dialog.pwad_folder());
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
                                    imp.switches_row.set_text("");
                                    imp.graphics_row.set_active(false);
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

        // Add launch Doom action
        let launch_action = gio::ActionEntry::builder("launch-doom")
            .activate(clone!(
                #[weak(rename_to = window)] self,
                move |_, _, _| {
                    window.set_sensitive(false);

                    match window.launch_doom() {
                        LaunchResult::Error(error_msg) => {
                            window.set_sensitive(true);

                            let error_dialog = adw::AlertDialog::builder()
                                .heading("Error")
                                .body(error_msg)
                                .body_use_markup(true)
                                .build();

                            error_dialog.add_responses(&[("ok", "_Ok")]);

                            error_dialog.present(Some(&window));
                        },
                        LaunchResult::Success => {
                            window.close();
                        }
                    }
                }
            ))
            .build();

        // Add actions to window
        self.add_action_entries([reset_action, prefs_action, launch_action]);
    }

    //-----------------------------------
    // Launch Doom function
    //-----------------------------------
    fn launch_doom(&self) -> LaunchResult {
        let imp = self.imp();

        // Return with error if no engine selected
        let Some(engine) = imp.engine_row.selected_engine() else {
            return LaunchResult::Error("Doom Engine not specified.".to_string())
        };

        // Return with error if no game (IWAD file) selected
        let Some(iwad) = imp.iwad_row.selected_iwad() else {
            return LaunchResult::Error("Game not specified.".to_string())
        };

        // Get executable file
        let exec_file = env_expand(&match iwad.id() {
            IWadID::UDOOM | IWadID::DOOM | IWadID::DOOM2 | IWadID::PLUTONIA | IWadID::TNT | IWadID::FREEDOOM1 | IWadID::FREEDOOM2 => {
                engine.path()
            },
            IWadID::HERETIC => {
                engine.heretic_path().unwrap_or(engine.path())
            },
            IWadID::HEXEN => {
                engine.hexen_path().unwrap_or(engine.path())
            },
            _ => unreachable!()
        });

        // Return with error if executable file does not exist
        if !Path::new(&exec_file).try_exists().unwrap_or_default() {
            return LaunchResult::Error(format!("Executable file <b>{}</b> not found.", exec_file))
        }

        // Return with error if IWAD file does not exist
        let iwad_file = env_expand(&iwad.filename());

        if !Path::new(&iwad_file).try_exists().unwrap_or_default() {
            return LaunchResult::Error(format!("IWAD file <b>{}</b> not found.", iwad_file))
        }

        // Get optional PWAD files
        let pwad_files = imp.pwad_row.files().join(" ");

        // Get hires graphics files if enabled
        let graphics_files = imp.graphics_map.get().unwrap().get(&iwad.id());

        let engine_hires = imp.engine_row.selected_engine()
            .map(|engine| engine.hires())
            .unwrap_or_default();

        let load_graphics = imp.graphics_installed.get() && graphics_files.is_some() && engine_hires && imp.graphics_row.is_active();

        let graphics_files = graphics_files
            .filter(|_| load_graphics)
            .map(|files| {
                files.into_iter()
                    .map(|file| Path::new(GRAPHICS_PATH).join(file).display().to_string())
                    .collect::<Vec<_>>()
                    .join(" ")
            })
            .unwrap_or_default();

        // Get extra switches
        let extra_switches = imp.switches_row.text();

        // Build Doom command line
        let mut cmd_line = format!("{exec_file} -iwad {iwad_file}");

        if !pwad_files.is_empty() {
            cmd_line = format!("{cmd_line} -file {pwad_files}");
        }

        if !graphics_files.is_empty() {
            cmd_line = format!("{cmd_line} -file {graphics_files}");
        }

        if !extra_switches.is_empty() {
            cmd_line = format!("{cmd_line} {extra_switches}");
        }

        // Launch Doom
        if let Some(params) = shlex::split(&cmd_line).filter(|params| !params.is_empty()) {
            Command::new(&params[0])
                .args(&params[1..])
                .spawn()
                .unwrap();
        }

        LaunchResult::Success
    }
}
