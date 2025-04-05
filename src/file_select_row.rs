use std::cell::{Cell, RefCell};
use std::borrow::Cow;
use std::path::Path;

use adw::prelude::ActionRowExt;
use gtk::{gio, glib};
use adw::subclass::prelude::*;
use gtk::prelude::*;
use glib::clone;

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

        #[property(get, set)]
        initial_folder: RefCell<String>,
        #[property(get, set)]
        default_file: RefCell<String>,
        #[property(get, set)]
        files: RefCell<Vec<String>>,
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
        // Constructor
        //-----------------------------------
        fn constructed(&self) {
            self.parent_constructed();

            let obj = self.obj();

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
        // Show reset button property setter
        //-----------------------------------
        fn set_show_reset_button(&self, can_reset: bool) {
            self.reset_button.set_visible(can_reset);
            self.separator.set_visible(can_reset);

            self.show_reset_button.replace(can_reset);
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
    // Path to file helper function
    //-----------------------------------
    fn path_to_file(&self, path: &str) -> Option<gio::File> {
        if path.is_empty() {
            None
        } else {
            let path = shellexpand::full(path)
                .unwrap_or(Cow::Borrowed(path))
                .to_string();

            Some(gio::File::for_path(path))
        }
    }

    //---------------------------------------
    // File to path function
    //---------------------------------------
    fn file_to_path(&self, file: &gio::File) -> String {
        file.path()
            .map(|path| path.display().to_string())
            .unwrap_or_default()
    }

    //-----------------------------------
    // Setup signals
    //-----------------------------------
    fn setup_signals(&self) {
        let imp = self.imp();

        // Files property notify signal
        self.connect_files_notify(clone!(
            #[weak] imp,
            move |row| {
                let files = row.files();

                let n_files = files.len();

                let label = match n_files {
                    0 => {
                        "(None)"
                    },
                    1 => {
                        Path::new(&files[0]).file_name()
                            .and_then(|filename| filename.to_str())
                            .or_else(|| Some("(None)"))
                            .unwrap()
                    },
                    _ => {
                        &format!("({n_files} files)")
                    }
                };

                imp.select_label.set_label(&label);

                imp.reset_button.set_sensitive(n_files > 0);
            }
        ));

        // Select button clicked signal
        self.connect_activated(clone!(
            #[weak(rename_to = row)] self,
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
                let files = row.files();

                if files.len() > 0 {
                    dialog.set_initial_file(row.path_to_file(&files[0]).as_ref());
                } else {
                    dialog.set_initial_folder(row.path_to_file(row.initial_folder().as_ref()).as_ref());
                }

                // Get root window
                let root = row.root()
                    .and_downcast::<gtk::Window>()
                    .expect("Must be a 'Window'");

                // Show dialog
                match row.select_mode() {
                    SelectMode::File => {
                        dialog.open(Some(&root), None::<&gio::Cancellable>, clone!(
                            #[weak] row,
                            move |result| {
                                if let Ok(file) = result {
                                    row.set_files(vec![row.file_to_path(&file)]);
                                }
                            }
                        ));
                    },
                    SelectMode::Multiple => {
                        dialog.open_multiple(Some(&root), None::<&gio::Cancellable>, clone!(
                            #[weak] row,
                            move |result| {
                                if let Ok(file_list) = result {
                                    row.set_files(file_list.iter::<gio::File>()
                                        .flatten()
                                        .map(|file| row.file_to_path(&file))
                                        .collect::<Vec<String>>()
                                    )
                                }
                            }
                        ));
                    },
                    SelectMode::Folder => {
                        dialog.select_folder(Some(&root), None::<&gio::Cancellable>, clone!(
                            #[weak] row,
                            move |result| {
                                if let Ok(folder) = result {
                                    row.set_files(vec![row.file_to_path(&folder)]);
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
        let default_file = self.default_file();

        if default_file.is_empty() {
            self.set_files(vec![]);
        } else {
            self.set_files(vec![default_file]);
        }
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
