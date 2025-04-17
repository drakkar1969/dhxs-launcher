use std::cell::{Cell, RefCell};
use std::path::Path;

use adw::prelude::ActionRowExt;
use gtk::{gio, glib};
use adw::subclass::prelude::*;
use gtk::prelude::*;
use glib::clone;

use crate::utils::{env_expand, file_to_path, path_to_file};

//------------------------------------------------------------------------------
// MODULE: FolderSelectRow
//------------------------------------------------------------------------------
mod imp {
    use super::*;

    //-----------------------------------
    // Private structure
    //-----------------------------------
    #[derive(Default, gtk::CompositeTemplate, glib::Properties)]
    #[properties(wrapper_type = super::FolderSelectRow)]
    #[template(resource = "/com/github/D-Launcher/ui/folder_select_row.ui")]
    pub struct FolderSelectRow {
        #[template_child]
        pub(super) label: TemplateChild<gtk::Label>,
        #[template_child]
        pub(super) separator: TemplateChild<gtk::Separator>,
        #[template_child]
        pub(super) reset_button: TemplateChild<gtk::Button>,

        #[property(get, set = Self::set_show_reset_button, construct)]
        show_reset_button: Cell<bool>,

        #[property(get, set, default = "Files", construct)]
        dialog_title: RefCell<String>,

        #[property(get, set)]
        initial_folder: RefCell<String>,
        #[property(get, set)]
        default_folder: RefCell<String>,
        #[property(get, set)]
        folder: RefCell<String>,
    }

    //-----------------------------------
    // Subclass
    //-----------------------------------
    #[glib::object_subclass]
    impl ObjectSubclass for FolderSelectRow {
        const NAME: &'static str = "FolderSelectRow";
        type Type = super::FolderSelectRow;
        type ParentType = adw::ActionRow;

        fn class_init(klass: &mut Self::Class) {
            klass.bind_template();
        }

        fn instance_init(obj: &glib::subclass::InitializingObject<Self>) {
            obj.init_template();
        }
    }

    #[glib::derived_properties]
    impl ObjectImpl for FolderSelectRow {
        //-----------------------------------
        // Constructor
        //-----------------------------------
        fn constructed(&self) {
            self.parent_constructed();

            let obj = self.obj();

            obj.setup_signals();
        }
    }

    impl WidgetImpl for FolderSelectRow {}
    impl ListBoxRowImpl for FolderSelectRow {}
    impl PreferencesRowImpl for FolderSelectRow {}
    impl ActionRowImpl for FolderSelectRow {}
    impl FolderSelectRow {
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
// IMPLEMENTATION: FolderSelectRow
//------------------------------------------------------------------------------
glib::wrapper! {
    pub struct FolderSelectRow(ObjectSubclass<imp::FolderSelectRow>)
        @extends adw::ActionRow, adw::PreferencesRow, gtk::ListBoxRow, gtk::Widget,
        @implements gtk::Accessible, gtk::Actionable, gtk::Buildable, gtk::ConstraintTarget;
}

impl FolderSelectRow {
    //-----------------------------------
    // New function
    //-----------------------------------
    pub fn new() -> Self {
        glib::Object::builder().build()
    }

    //-----------------------------------
    // Setup signals
    //-----------------------------------
    fn setup_signals(&self) {
        let imp = self.imp();

        // Folder property notify signal
        self.connect_folder_notify(clone!(
            #[weak] imp,
            move |row| {
                let folder = env_expand(&row.folder());

                let label = Path::new(&folder).file_name()
                    .and_then(|filename| filename.to_str())
                    .filter(|filename| !filename.is_empty())
                    .unwrap_or("(None)");

                imp.label.set_label(label);

                imp.reset_button.set_sensitive(folder != env_expand(&row.default_folder()));
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

                // Set initial location for dialog
                let folder = row.folder();

                if !folder.is_empty() {
                    dialog.set_initial_file(path_to_file(&folder).as_ref());
                } else {
                    dialog.set_initial_folder(path_to_file(row.initial_folder().as_ref()).as_ref());
                }

                // Get root window
                let root = row.root()
                    .and_downcast::<gtk::Window>();

                // Show dialog
                dialog.select_folder(root.as_ref(), None::<&gio::Cancellable>, clone!(
                    #[weak] row,
                    move |result| {
                        if let Ok(folder) = result {
                            row.set_folder(file_to_path(&folder));
                        }
                    }
                ));
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
        self.set_folder(self.default_folder());
    }
}

impl Default for FolderSelectRow {
    //-----------------------------------
    // Default constructor
    //-----------------------------------
    fn default() -> Self {
        Self::new()
    }
}
