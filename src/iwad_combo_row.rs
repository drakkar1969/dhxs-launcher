use std::collections::HashMap;

use gtk::{gio, glib};
use adw::subclass::prelude::*;
use adw::prelude::*;

use glob::{glob_with, MatchOptions};

use crate::iwad_object::IWadObject;
use crate::iwad_data::{IWAD_PATHS, IWAD_HASHMAP};
use crate::utils::crc32;

//------------------------------------------------------------------------------
// MODULE: IWadComboRow
//------------------------------------------------------------------------------
mod imp {
    use super::*;

    //-----------------------------------
    // Private structure
    //-----------------------------------
    #[derive(Default, gtk::CompositeTemplate)]
    #[template(resource = "/com/github/DHXS-Launcher/ui/iwad_combo_row.ui")]
    pub struct IWadComboRow {
        #[template_child]
        pub(super) model: TemplateChild<gio::ListStore>,
        #[template_child]
        pub(super) sort_model: TemplateChild<gtk::SortListModel>,
    }

    //-----------------------------------
    // Subclass
    //-----------------------------------
    #[glib::object_subclass]
    impl ObjectSubclass for IWadComboRow {
        const NAME: &'static str = "IWadComboRow";
        type Type = super::IWadComboRow;
        type ParentType = adw::ComboRow;

        fn class_init(klass: &mut Self::Class) {
            IWadObject::ensure_type();

            klass.bind_template();
        }

        fn instance_init(obj: &glib::subclass::InitializingObject<Self>) {
            obj.init_template();
        }
    }

    impl ObjectImpl for IWadComboRow {
        //-----------------------------------
        // Constructor
        //-----------------------------------
        fn constructed(&self) {
            self.parent_constructed();
        }
    }

    impl WidgetImpl for IWadComboRow {}
    impl ListBoxRowImpl for IWadComboRow {}
    impl PreferencesRowImpl for IWadComboRow {}
    impl ActionRowImpl for IWadComboRow {}
    impl ComboRowImpl for IWadComboRow {}
}

//------------------------------------------------------------------------------
// IMPLEMENTATION: IWadComboRow
//------------------------------------------------------------------------------
glib::wrapper! {
    pub struct IWadComboRow(ObjectSubclass<imp::IWadComboRow>)
        @extends adw::ComboRow, adw::ActionRow, adw::PreferencesRow, gtk::ListBoxRow, gtk::Widget,
        @implements gtk::Accessible, gtk::Actionable, gtk::Buildable, gtk::ConstraintTarget;
}

impl IWadComboRow {
    //-----------------------------------
    // Public init for folders function
    //-----------------------------------
    pub fn init_for_folders(&self, user_folder: &str) {
        let imp = self.imp();

        let options = MatchOptions {
            case_sensitive: false,
            require_literal_separator: false,
            require_literal_leading_dot: false
        };

        // Get list of IWADs in folders
        let hash_map = HashMap::from(IWAD_HASHMAP);

        let iwad_objects = IWAD_PATHS.iter().chain([&user_folder])
            .flat_map(|folder| glob_with(&format!("{folder}/*.wad"), options))
            .flat_map(|paths| {
                paths.into_iter()
                    .flatten()
                    .filter_map(|path| {
                        let filename = path.display().to_string();

                        crc32(&filename).ok()
                            .and_then(|hash| hash_map.get(&hash))
                            .map(|data| IWadObject::new(data, &filename))
                    })
            })
            .collect::<Vec<IWadObject>>();

        // Add IWADs to combo row
        imp.model.splice(0, imp.model.n_items(), &iwad_objects);
    }

    //-----------------------------------
    // Public selected iwad function
    //-----------------------------------
    pub fn selected_iwad(&self) -> Option<IWadObject> {
        self.selected_item()
            .and_downcast::<IWadObject>()
    }

    //-----------------------------------
    // Public set selected iwad file function
    //-----------------------------------
    pub fn set_selected_iwad_file(&self, filename: &str) {
        let index = self.imp().sort_model.iter::<glib::Object>()
            .flatten()
            .position(|obj| {
                let iwad = obj.downcast::<IWadObject>()
                    .expect("Must be a 'IWadObject'");

                iwad.filename() == filename
            });

        self.set_selected(index.unwrap_or_default() as u32);
    }
}

impl Default for IWadComboRow {
    //-----------------------------------
    // Default constructor
    //-----------------------------------
    fn default() -> Self {
        glib::Object::builder().build()
    }
}
