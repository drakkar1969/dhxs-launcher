use std::cell::{Cell, RefCell, OnceCell};
use std::sync::OnceLock;

use adw::prelude::ActionRowExt;
use gtk::{gio, glib};
use adw::subclass::prelude::*;
use gtk::prelude::*;
use glib::clone;
use glib::subclass::Signal;

//------------------------------------------------------------------------------
// ENUM: SelectMode
//------------------------------------------------------------------------------
#[derive(Default, Debug, Eq, PartialEq, Clone, Copy, glib::Enum)]
#[repr(u32)]
#[enum_type(name = "SelectMode")]
pub enum SelectMode {
    #[default]
    File = 0,
    Multiple = 1,
    Folder = 2,
}

//------------------------------------------------------------------------------
// MODULE: FileSelectRow
//------------------------------------------------------------------------------
mod imp {
    use super::*;

    //-----------------------------------
    // Private structure
    //-----------------------------------
    #[derive(Default, gtk::CompositeTemplate, glib::Properties)]
    #[properties(wrapper_type = super::FileSelectRow)]
    #[template(resource = "/com/github/D-Launcher/ui/file_select_row.ui")]
    pub struct FileSelectRow {
        #[template_child]
        pub select_label: TemplateChild<gtk::Label>,
        #[template_child]
        pub select_image: TemplateChild<gtk::Image>,
        #[template_child]
        pub separator: TemplateChild<gtk::Separator>,
        #[template_child]
        pub reset_button: TemplateChild<gtk::Button>,

        #[property(get, set = Self::set_select, construct, builder(SelectMode::default()))]
        select_mode: Cell<SelectMode>,
        #[property(get, set = Self::set_icon, nullable, construct)]
        icon: RefCell<Option<String>>,
        #[property(get, set = Self::set_show_reset_button, construct)]
        show_reset_button: Cell<bool>,

        #[property(get, set, default = "Files", construct)]
        dialog_title: RefCell<String>,
        #[property(get, set, nullable, construct)]
        filter: RefCell<Option<gtk::FileFilter>>,

        #[property(name = "initial-path", type = String, get = Self::initial_path, set = Self::set_initial_path)]
        #[property(get, set, nullable)]
        initial_folder: RefCell<Option<gio::File>>,

        #[property(name = "default-path", type = String, get = Self::default_path, set = Self::set_default_path)]
        #[property(get, set, nullable)]
        default_file: RefCell<Option<gio::File>>,

        #[property(name = "paths", type = Vec<String>, get = Self::paths, set = Self::set_paths)]
        #[property(name = "path", type = String, get = Self::path, set = Self::set_path)]
        pub(super) files: OnceCell<gio::ListStore>,
    }

    //-----------------------------------
    // Subclass
    //-----------------------------------
    #[glib::object_subclass]
    impl ObjectSubclass for FileSelectRow {
        const NAME: &'static str = "FileSelectRow";
        type Type = super::FileSelectRow;
        type ParentType = adw::ActionRow;

        fn class_init(klass: &mut Self::Class) {
            klass.bind_template();
        }

        fn instance_init(obj: &glib::subclass::InitializingObject<Self>) {
            obj.init_template();
        }
    }

    #[glib::derived_properties]
    impl ObjectImpl for FileSelectRow {
        //-----------------------------------
        // Custom signals
        //-----------------------------------
        fn signals() -> &'static [Signal] {
            static SIGNALS: OnceLock<Vec<Signal>> = OnceLock::new();
            SIGNALS.get_or_init(|| {
                vec![
                    Signal::builder("changed")
                        .build(),
                ]
            })
        }

        //-----------------------------------
        // Constructor
        //-----------------------------------
        fn constructed(&self) {
            self.parent_constructed();

            let obj = self.obj();

            obj.setup_widgets();
            obj.setup_signals();
        }
    }

    impl WidgetImpl for FileSelectRow {}
    impl ListBoxRowImpl for FileSelectRow {}
    impl PreferencesRowImpl for FileSelectRow {}
    impl ActionRowImpl for FileSelectRow {}
    impl FileSelectRow {
        //-----------------------------------
        // Select property setter
        //-----------------------------------
        fn set_select(&self, select: SelectMode) {
            if self.icon.borrow().is_none() {
                if select == SelectMode::Folder {
                    self.select_image.set_icon_name(Some("folder-symbolic"));
                } else {
                    self.select_image.set_icon_name(Some("document-open-symbolic"));
                }
            }

            self.select_mode.replace(select);
        }

        //-----------------------------------
        // Icon property setter
        //-----------------------------------
        fn set_icon(&self, icon: Option<&str>) {
            if icon.is_some() {
                self.select_image.set_icon_name(icon);
            } else if self.select_mode.get() == SelectMode::Folder {
                self.select_image.set_icon_name(Some("folder-symbolic"));
            } else {
                self.select_image.set_icon_name(Some("document-open-symbolic"));
            }

            self.icon.replace(icon.map(|icon| icon.to_string()));
        }

        //-----------------------------------
        // Show reset buttob property setter
        //-----------------------------------
        fn set_show_reset_button(&self, can_reset: bool) {
            self.reset_button.set_visible(can_reset);
            self.separator.set_visible(can_reset);

            self.show_reset_button.replace(can_reset);
        }

        //-----------------------------------
        // Initial path property getter/setter
        //-----------------------------------
        fn initial_path(&self) -> String {
            let initial_folder = self.initial_folder.borrow();

            initial_folder.as_ref()
                .and_then(|folder| folder.path())
                .map(|path| path.display().to_string())
                .unwrap_or_default()
        }

        fn set_initial_path(&self, path: &str) {
            self.initial_folder.replace(self.path_to_file(path));
        }

        //-----------------------------------
        // Default path property getter/setter
        //-----------------------------------
        fn default_path(&self) -> String {
            let default_file = self.default_file.borrow();

            default_file.as_ref()
                .and_then(|folder| folder.path())
                .map(|path| path.display().to_string())
                .unwrap_or_default()
        }

        fn set_default_path(&self, path: &str) {
            self.default_file.replace(self.path_to_file(path));
        }

        //-----------------------------------
        // Path property getter/setter
        //-----------------------------------
        fn path(&self) -> String {
            let files = self.files.get().unwrap();

            files.item(0)
                .and_downcast::<gio::File>()
                .and_then(|file| file.path())
                .map(|path| path.display().to_string())
                .unwrap_or_default()
        }

        fn set_path(&self, path: &str) {
            let files = self.files.get().unwrap();

            if let Some(path) = self.path_to_file(path) {
                files.splice(0, files.n_items(), &[path])
            } else {
                files.remove_all();
            }

            self.set_state();
        }

        //-----------------------------------
        // Paths property getter/setter
        //-----------------------------------
        fn paths(&self) -> Vec<String> {
            let files = self.files.get().unwrap();

            files.iter::<gio::File>()
                .filter_map(|file| {
                    file.ok()
                        .and_then(|file| file.path())
                        .map(|path| path.display().to_string())
                })
                .collect::<Vec<String>>()
        }

        fn set_paths(&self, paths: Vec<String>) {
            let files = self.files.get().unwrap();

            files.splice(0, files.n_items(), &paths.iter()
                .filter_map(|path| self.path_to_file(path.as_str()))
                .collect::<Vec<gio::File>>()
            );

            self.set_state();
        }

        //-----------------------------------
        // Path to file helper function
        //-----------------------------------
        fn path_to_file(&self, path: &str) -> Option<gio::File> {
            if path.is_empty() {
                return None
            }

            let file = if let Ok(path_exp) = shellexpand::full(&path) {
                gio::File::for_path(path_exp.to_string())
            } else {
                gio::File::for_path(path)
            };

            file.query_exists(None::<&gio::Cancellable>).then_some(file)
        }

        //-----------------------------------
        // Set state helper function
        //-----------------------------------
        pub(super) fn set_state(&self) {
            let files = self.files.get().unwrap();

            let n_files = files.n_items();

            let label = match n_files {
                0 => {
                    "(None)".to_string()
                },
                1 => {
                    let file = files.item(0).and_downcast::<gio::File>();
                    file
                        .and_then(|file| file.basename())
                        .map(|name| name.display().to_string())
                        .or_else(|| Some("(None)".to_string()))
                        .unwrap()
                },
                _ => {
                    format!("({n_files} files)")
                }
            };

            self.select_label.set_label(&label);

            self.reset_button.set_sensitive(n_files > 0);

            self.obj().emit_by_name::<()>("changed", &[]);
        }
    }
}

//------------------------------------------------------------------------------
// IMPLEMENTATION: FileSelectRow
//------------------------------------------------------------------------------
glib::wrapper! {
    pub struct FileSelectRow(ObjectSubclass<imp::FileSelectRow>)
        @extends adw::ActionRow, adw::PreferencesRow, gtk::ListBoxRow, gtk::Widget,
        @implements gtk::Accessible, gtk::Actionable, gtk::Buildable, gtk::ConstraintTarget;
}

impl FileSelectRow {
    //-----------------------------------
    // New function
    //-----------------------------------
    pub fn new() -> Self {
        glib::Object::builder().build()
    }

    //-----------------------------------
    // Setup widgets
    //-----------------------------------
    fn setup_widgets(&self) {
        let model = gio::ListStore::new::<gio::File>();

        self.imp().files.set(model).unwrap();
    }

    //-----------------------------------
    // Setup signals
    //-----------------------------------
    fn setup_signals(&self) {
        let imp = self.imp();

        // Select button clicked signal
        self.connect_activated(clone!(
            #[weak(rename_to = row)] self,
            #[weak] imp,
            move |_| {
                // Create dialog
                let dialog = gtk::FileDialog::builder()
                    .title(row.dialog_title())
                    .modal(true)
                    .accept_label("Select")
                    .build();

                // Set filters for dialog
                if row.select_mode() != SelectMode::Folder {
                    let all_filter = gtk::FileFilter::new();
                    all_filter.set_name(Some("All Files"));
                    all_filter.add_pattern("*");

                    let dialog_filters = gio::ListStore::from_iter([all_filter]);

                    if let Some(filter) = row.filter() {
                        dialog_filters.append(&filter);
                    }

                    dialog.set_filters(Some(&dialog_filters));
                    dialog.set_default_filter(row.filter().as_ref());
                }

                // Set initial location for dialog
                let files = imp.files.get().unwrap();

                if files.n_items() > 0 {
                    dialog.set_initial_file(files.item(0).and_downcast::<gio::File>().as_ref());
                } else {
                    dialog.set_initial_folder(row.initial_folder().as_ref());
                }

                // Get root window
                let root = row.root()
                    .and_downcast::<gtk::Window>()
                    .expect("Must be a 'Window'");

                // Show dialog
                match row.select_mode() {
                    SelectMode::File => {
                        dialog.open(Some(&root), None::<&gio::Cancellable>, clone!(
                            #[weak] imp,
                            move |result| {
                                if let Ok(file) = result {
                                    let files = imp.files.get().unwrap();

                                    files.splice(0, files.n_items(), &[file]);

                                    imp.set_state();
                                }
                            }
                        ));
                    },
                    SelectMode::Multiple => {
                        dialog.open_multiple(Some(&root), None::<&gio::Cancellable>, clone!(
                            #[weak] imp,
                            move |result| {
                                if let Ok(file_list) = result {
                                    let files = imp.files.get().unwrap();

                                    files.splice(0, files.n_items(), &file_list.iter::<gio::File>().flatten().collect::<Vec<gio::File>>());

                                    imp.set_state();
                                }
                            }
                        ));
                    },
                    SelectMode::Folder => {
                        dialog.select_folder(Some(&root), None::<&gio::Cancellable>, clone!(
                            #[weak] imp,
                            move |result| {
                                if let Ok(folder) = result {
                                    let files = imp.files.get().unwrap();

                                    files.splice(0, files.n_items(), &[folder]);

                                    imp.set_state();
                                }
                            }
                        ));
                    }
                }
            }
        ));

        // Reset button clicked signal
        imp.reset_button.connect_clicked(clone!(
            #[weak(rename_to = row)] self,
            move |_| {
                row.reset_to_default();
            }
        ));
    }

    //-----------------------------------
    // Public reset_to_default function
    //-----------------------------------
    pub fn reset_to_default(&self) {
        let imp = self.imp();

        let files = imp.files.get().unwrap();

        if let Some(default_file) = self.default_file() {
            files.splice(0, files.n_items(), &[default_file]);
        } else {
            files.remove_all();
        }

        imp.set_state();
    }
}

impl Default for FileSelectRow {
    //-----------------------------------
    // Default constructor
    //-----------------------------------
    fn default() -> Self {
        Self::new()
    }
}
