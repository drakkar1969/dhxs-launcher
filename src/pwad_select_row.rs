use std::cell::{Cell, RefCell};
use std::path::Path;

use adw::prelude::ActionRowExt;
use gtk::{gio, glib};
use adw::subclass::prelude::*;
use gtk::prelude::*;
use glib::clone;

use crate::utils::{file_to_path, path_to_file};

//------------------------------------------------------------------------------
// MODULE: PWadSelectRow
//------------------------------------------------------------------------------
mod imp {
    use super::*;

    //-----------------------------------
    // Private structure
    //-----------------------------------
    #[derive(Default, gtk::CompositeTemplate, glib::Properties)]
    #[properties(wrapper_type = super::PWadSelectRow)]
    #[template(resource = "/com/github/D-Launcher/ui/pwad_select_row.ui")]
    pub struct PWadSelectRow {
        #[template_child]
        pub(super) label: TemplateChild<gtk::Label>,
        #[template_child]
        pub(super) image: TemplateChild<gtk::Image>,
        #[template_child]
        pub(super) separator: TemplateChild<gtk::Separator>,
        #[template_child]
        pub(super) reset_button: TemplateChild<gtk::Button>,

        #[property(get, set = Self::set_icon, nullable, construct)]
        icon: RefCell<Option<String>>,
        #[property(get, set = Self::set_show_reset_button, construct)]
        show_reset_button: Cell<bool>,

        #[property(get, set)]
        initial_folder: RefCell<String>,
        #[property(get, set)]
        files: RefCell<Vec<String>>,
    }

    //-----------------------------------
    // Subclass
    //-----------------------------------
    #[glib::object_subclass]
    impl ObjectSubclass for PWadSelectRow {
        const NAME: &'static str = "PWadSelectRow";
        type Type = super::PWadSelectRow;
        type ParentType = adw::ActionRow;

        fn class_init(klass: &mut Self::Class) {
            klass.bind_template();
        }

        fn instance_init(obj: &glib::subclass::InitializingObject<Self>) {
            obj.init_template();
        }
    }

    #[glib::derived_properties]
    impl ObjectImpl for PWadSelectRow {
        //-----------------------------------
        // Constructor
        //-----------------------------------
        fn constructed(&self) {
            self.parent_constructed();

            let obj = self.obj();

            obj.setup_signals();
        }
    }

    impl WidgetImpl for PWadSelectRow {}
    impl ListBoxRowImpl for PWadSelectRow {}
    impl PreferencesRowImpl for PWadSelectRow {}
    impl ActionRowImpl for PWadSelectRow {}
    impl PWadSelectRow {
        //-----------------------------------
        // Icon property setter
        //-----------------------------------
        fn set_icon(&self, icon: Option<&str>) {
            if icon.is_some() {
                self.image.set_icon_name(icon);
            } else {
                self.image.set_icon_name(Some("media-zip-symbolic"));
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
// IMPLEMENTATION: PWadSelectRow
//------------------------------------------------------------------------------
glib::wrapper! {
    pub struct PWadSelectRow(ObjectSubclass<imp::PWadSelectRow>)
        @extends adw::ActionRow, adw::PreferencesRow, gtk::ListBoxRow, gtk::Widget,
        @implements gtk::Accessible, gtk::Actionable, gtk::Buildable, gtk::ConstraintTarget;
}

impl PWadSelectRow {
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
                            .unwrap_or("(None)")
                    },
                    _ => {
                        &format!("({n_files} files)")
                    }
                };

                imp.label.set_label(label);

                imp.reset_button.set_sensitive(n_files > 0);
            }
        ));

        // Select button clicked signal
        self.connect_activated(clone!(
            move |row| {
                // Create dialog
                let dialog = gtk::FileDialog::builder()
                    .title("PWAD Files")
                    .modal(true)
                    .accept_label("Select")
                    .build();

                // Set filters for dialog
                let all_filter = gtk::FileFilter::new();
                all_filter.set_name(Some("All Files"));
                all_filter.add_pattern("*");

                let pwad_filter = gtk::FileFilter::new();
                pwad_filter.set_name(Some("WAD/PK3/PK7 files"));
                pwad_filter.add_mime_type("application/x-doom-wad");
                pwad_filter.add_mime_type("application/zip");
                pwad_filter.add_mime_type("application/x-7z-compressed");

                dialog.set_default_filter(Some(&pwad_filter));
                dialog.set_filters(Some(&gio::ListStore::from_iter([pwad_filter, all_filter])));

                // Set initial location for dialog
                let files = row.files();

                if !files.is_empty() {
                    dialog.set_initial_file(path_to_file(&files[0]).as_ref());
                } else {
                    dialog.set_initial_folder(path_to_file(row.initial_folder().as_ref()).as_ref());
                }

                // Get root window
                let root = row.root()
                    .and_downcast::<gtk::Window>();

                // Show dialog
                dialog.open_multiple(root.as_ref(), None::<&gio::Cancellable>, clone!(
                    #[weak] row,
                    move |result| {
                        if let Ok(file_list) = result {
                            row.set_files(file_list.iter::<gio::File>()
                                .flatten()
                                .map(|file| file_to_path(&file))
                                .collect::<Vec<String>>()
                            )
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
        self.set_files(vec![]);
    }
}

impl Default for PWadSelectRow {
    //-----------------------------------
    // Default constructor
    //-----------------------------------
    fn default() -> Self {
        Self::new()
    }
}
