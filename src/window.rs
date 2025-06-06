use std::path::Path;
use std::process::Command;
use std::collections::HashMap;
use std::fmt::Write as _;

use gtk::{gio, glib, gdk, pango};
use adw::subclass::prelude::*;
use adw::prelude::*;
use glib::{clone, closure_local};

use crate::APP_ID;
use crate::LauncherApp;
use crate::engine_data::EngineSource;
use crate::engine_combo_row::EngineComboRow;
use crate::engine_object::EngineObject;
use crate::iwad_combo_row::IWadComboRow;
use crate::pwad_select_row::PWadSelectRow;
use crate::preferences_dialog::PreferencesDialog;
use crate::utils::env_expand;
use crate::iwad_data::IWadID;
use crate::graphics_data::{GRAPHICS_PATH, GRAPHICS_MAP};

//------------------------------------------------------------------------------
// ENUM: LaunchResult
//------------------------------------------------------------------------------
enum LaunchResult {
    Success,
    Error(String)
}

//------------------------------------------------------------------------------
// MODULE: AppWindow
//------------------------------------------------------------------------------
mod imp {
    use super::*;

    //-----------------------------------
    // Private structure
    //-----------------------------------
    #[derive(Default, gtk::CompositeTemplate)]
    #[template(resource = "/com/github/DHXS-Launcher/ui/window.ui")]
    pub struct AppWindow {
        #[template_child]
        pub(super) split_view: TemplateChild<adw::OverlaySplitView>,

        #[template_child]
        pub(super) engine_row: TemplateChild<EngineComboRow>,

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
        pub(super) settings_title: TemplateChild<adw::WindowTitle>,
        #[template_child]
        pub(super) settings_zdoom_group: TemplateChild<adw::PreferencesGroup>,
        #[template_child]
        pub(super) settings_hires_row: TemplateChild<adw::SwitchRow>,
        #[template_child]
        pub(super) settings_config_row: TemplateChild<adw::ActionRow>,

        #[template_child]
        pub(super) prefs_dialog: TemplateChild<PreferencesDialog>,
    }

    //-----------------------------------
    // Subclass
    //-----------------------------------
    #[glib::object_subclass]
    impl ObjectSubclass for AppWindow {
        const NAME: &'static str = "AppWindow";
        type Type = super::AppWindow;
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

    impl ObjectImpl for AppWindow {
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

    impl WidgetImpl for AppWindow {}
    impl WindowImpl for AppWindow {
        //-----------------------------------
        // Window close handler
        //-----------------------------------
        fn close_request(&self) -> glib::Propagation {
            self.obj().save_gsettings();

            glib::Propagation::Proceed
        }
    }
    impl ApplicationWindowImpl for AppWindow {}
    impl AdwApplicationWindowImpl for AppWindow {}
}

//------------------------------------------------------------------------------
// IMPLEMENTATION: AppWindow
//------------------------------------------------------------------------------
glib::wrapper! {
    pub struct AppWindow(ObjectSubclass<imp::AppWindow>)
        @extends gtk::Widget, gtk::Window, gtk::ApplicationWindow, adw::ApplicationWindow,
        @implements gio::ActionGroup, gio::ActionMap, gtk::Accessible, gtk::Buildable,
                    gtk::ConstraintTarget, gtk::Native, gtk::Root, gtk::ShortcutManager;
}

impl AppWindow {
    //-----------------------------------
    // New function
    //-----------------------------------
    pub fn new(app: &LauncherApp) -> Self {
        glib::Object::builder().property("application", app).build()
    }

    //-----------------------------------
    // Label helper functions
    //-----------------------------------
    fn key_label(key: &str) -> gtk::Label {
        let label = gtk::Label::new(Some(key));
        label.set_vexpand(true);
        label.set_xalign(0.0);
        label.set_yalign(0.0);
        label.set_can_focus(false);
        label.set_selectable(true);
        label.set_css_classes(&["heading"]);

        label
    }

    fn value_label(value: &str) -> gtk::Label {
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

        // Create user config dirs if they do not exist
        if let Ok(xdg_dirs) = xdg::BaseDirectories::new() {
            let _ = xdg_dirs.create_config_directory("dhxs-launcher/iwads");
            let _ = xdg_dirs.create_config_directory("dhxs-launcher/pwads");
        }

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
            imp.switches_grid.attach(&Self::key_label(key), 0, i as i32, 1, 1);
            imp.switches_grid.attach(&Self::value_label(value), 1, i as i32, 1, 1);
        });

        // Set initial focus on engine combo row
        imp.engine_row.get().grab_focus();
    }

    //-----------------------------------
    // Set launch button state helper function
    //-----------------------------------
    fn set_launch_button_state(&self) {
        let imp = self.imp();

        imp.launch_button.set_sensitive(imp.engine_row.selected_item().is_some() && imp.iwad_row.selected_iwad().is_some());
    }

    //-----------------------------------
    // Setup signals
    //-----------------------------------
    fn setup_signals(&self) {
        let imp = self.imp();

        // Preferences window IWAD folder property notify signal
        imp.prefs_dialog.connect_iwad_folder_notify(clone!(
            #[weak(rename_to = window)] self,
            #[weak] imp,
            move |prefs_dialog| {
                imp.iwad_row.init_for_folders(&env_expand(&prefs_dialog.iwad_folder()));

                window.set_launch_button_state();
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
            #[weak(rename_to = window)] self,
            move |_| {
                window.set_launch_button_state();
            }
        ));

        // Engine combo settings clicked signal
        imp.engine_row.connect_closure("settings-clicked", false, closure_local!(
            #[watch(rename_to = window)] self,
            move |engine_row: EngineComboRow| {
                let imp = window.imp();

                if let Some(engine) = engine_row.selected_engine() {
                    imp.settings_title.set_title(&format!("{} Settings", engine.name()));

                    let is_zdoom = engine.source() == EngineSource::ZDoom;

                    imp.settings_zdoom_group.set_visible(is_zdoom);

                    if is_zdoom {
                        imp.settings_hires_row.set_active(engine.settings().hires());
                    }

                    imp.split_view.set_show_sidebar(true);
                }
            }
        ));

        // Settings hires row active property signal
        imp.settings_hires_row.connect_active_notify(clone!(
            #[weak] imp,
            move |row| {
                if let Some(engine) = imp.engine_row.selected_engine() {
                    engine.settings().set_hires(row.is_active());
                }
            }
        ));

        // Settings config row actived signal
        imp.settings_config_row.connect_activated(clone!(
            #[weak] imp,
            move |_| {
                if let Some(engine) = imp.engine_row.selected_engine() {
                    let uri = format!("file://{}", env_expand(&engine.config_folder()));

                    if let Some(desktop) = gio::AppInfo::default_for_type("inode/directory", true) {
                        let _res = desktop.launch_uris(&[&uri], None::<&gio::AppLaunchContext>);
                    }
                }
            }
        ));

        // IWAD combo selected item property notify signal
        imp.iwad_row.connect_selected_item_notify(clone!(
            #[weak(rename_to = window)] self,
            #[weak] imp,
            move |iwad_row| {
                imp.engine_row.filter_engines(iwad_row.selected_iwad().map(|iwad| iwad.id()));

                window.set_launch_button_state();
            }
        ));
    }

    //-----------------------------------
    // Gsetting default value helper function
    //-----------------------------------
    fn gsetting_default_value(gsettings: &gio::Settings, key: &str) -> String {
        gsettings.default_value(key).and_then(|value| value.get::<String>()).unwrap()
    }

    //---------------------------------------
    // Set gsetting helper function
    //---------------------------------------
    fn set_gsetting<T: FromVariant + ToVariant + PartialEq>(gsettings: &gio::Settings, key: &str, value: &T) {
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
        imp.prefs_dialog.set_iwad_default_folder(Self::gsetting_default_value(&gsettings,"iwad-folder"));
        imp.prefs_dialog.set_pwad_default_folder(Self::gsetting_default_value(&gsettings,"pwad-folder"));

        imp.prefs_dialog.set_iwad_folder(gsettings.string("iwad-folder"));
        imp.prefs_dialog.set_pwad_folder(gsettings.string("pwad-folder"));

        // Init main window
        imp.engine_row.set_selected_engine_name(&gsettings.string("selected-engine"));
        imp.iwad_row.set_selected_iwad_file(&gsettings.string("selected-iwad"));
        imp.pwad_row.set_files(gsettings.strv("pwad-files").into_iter().map(String::from).collect::<Vec<String>>());
        imp.switches_row.set_text(&gsettings.string("extra-switches"));

        // Init engine settings
        for engine in imp.engine_row.engines().iter::<EngineObject>().flatten() {
            if engine.source() == EngineSource::ZDoom {
                let gsettings = gio::Settings::new(&format!("{}.{}", APP_ID, engine.name()));

                engine.settings().set_hires(gsettings.boolean("hires"));
            }
        }
    }

    //-----------------------------------
    // Save gsettings
    //-----------------------------------
    fn save_gsettings(&self) {
        let imp = self.imp();

        // Create gsettings
        let gsettings = gio::Settings::new(APP_ID);

        // Get selected engine
        let selected_engine = imp.engine_row.selected_engine()
            .map_or(String::new(), |engine| engine.name());

        // Get selected IWAD
        let selected_iwad = imp.iwad_row.selected_iwad()
            .map_or(String::new(), |iwad| iwad.filename());

        // Save main window settings
        Self::set_gsetting(&gsettings, "selected-engine", &selected_engine);
        Self::set_gsetting(&gsettings, "selected-iwad", &selected_iwad);
        Self::set_gsetting(&gsettings, "pwad-files", &imp.pwad_row.files());
        Self::set_gsetting(&gsettings, "extra-switches", &imp.switches_row.text().to_string());

        // Save preferences window settings
        Self::set_gsetting(&gsettings, "iwad-folder", &imp.prefs_dialog.iwad_folder());
        Self::set_gsetting(&gsettings, "pwad-folder", &imp.prefs_dialog.pwad_folder());

        // Save engine settings
        for engine in imp.engine_row.engines().iter::<EngineObject>().flatten() {
            if engine.source() == EngineSource::ZDoom {
                let gsettings = gio::Settings::new(&format!("{}.{}", APP_ID, engine.name()));

                Self::set_gsetting(&gsettings, "hires", &engine.settings().hires());
            }
        }
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
                                    imp.engine_row.reset_engine_settings();
                                    imp.iwad_row.set_selected(0);
                                    imp.pwad_row.reset_to_default();
                                    imp.switches_row.set_text("");
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
    #[allow(clippy::zombie_processes)]
    fn launch_doom(&self) -> LaunchResult {
        let imp = self.imp();

        // Return with error if no engine selected
        let Some(engine) = imp.engine_row.selected_engine() else {
            return LaunchResult::Error(String::from("Doom Engine not specified."))
        };

        // Return with error if no game (IWAD file) selected
        let Some(iwad) = imp.iwad_row.selected_iwad() else {
            return LaunchResult::Error(String::from("Game not specified."))
        };

        // Get executable file
        let exec_file = env_expand(&match iwad.id() {
            IWadID::DOOM | IWadID::UDOOM | IWadID::DOOM2 | IWadID::PLUTONIA | IWadID::TNT | IWadID::FREEDOOM1 | IWadID::FREEDOOM2 => {
                engine.doom_path()
            },
            IWadID::HERETIC => {
                engine.heretic_path().unwrap_or_else(|| engine.doom_path())
            },
            IWadID::HEXEN => {
                engine.hexen_path().unwrap_or_else(|| engine.doom_path())
            },
            IWadID::STRIFE => {
                engine.strife_path().unwrap_or_else(|| engine.doom_path())
            },
            _ => unreachable!()
        });

        // Return with error if executable file does not exist
        if !Path::new(&exec_file).try_exists().unwrap_or_default() {
            return LaunchResult::Error(format!("Executable file <b>{exec_file}</b> not found."))
        }

        // Return with error if IWAD file does not exist
        let iwad_file = env_expand(&iwad.filename());

        if !Path::new(&iwad_file).try_exists().unwrap_or_default() {
            return LaunchResult::Error(format!("IWAD file <b>{iwad_file}</b> not found."))
        }

        // Init Doom command line with exec file and IWAD
        let mut cmd_line = format!("{exec_file} -iwad {iwad_file}");

        // Get optional PWAD files
        let pwad_files = imp.pwad_row.files().join(" ");

        if !pwad_files.is_empty() {
            write!(cmd_line, " -file {pwad_files}").unwrap();
        }

        // Get extra switches
        let extra_switches = imp.switches_row.text();

        if !extra_switches.is_empty() {
            write!(cmd_line, " {extra_switches}").unwrap();
        }

        // Get hires graphics files if enabled
        let load_graphics = Path::new(GRAPHICS_PATH).try_exists().unwrap_or_default() &&
            (engine.source() == EngineSource::ZDoom) &&
            engine.settings().hires();

        let graphics_files = if load_graphics {
            let graphics_map = HashMap::from(GRAPHICS_MAP);

            graphics_map.get(&iwad.id())
                .filter(|_| load_graphics)
                .map(|files| {
                    files.iter()
                        .map(|file| Path::new(GRAPHICS_PATH).join(file).display().to_string())
                        .collect::<Vec<String>>()
                        .join(" ")
                })
                .unwrap_or_default()
        } else {
            String::new()
        };

        if !graphics_files.is_empty() {
            write!(cmd_line, " -file {graphics_files}").unwrap();
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
