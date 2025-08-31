use std::collections::HashMap;

use gtk::{gio, glib};
use adw::subclass::prelude::*;
use adw::prelude::*;

use glob::{glob_with, MatchOptions};

use crate::iwad_object::IWadObject;
use crate::iwad_data::{IWadData, IWAD_HASHMAP, IWAD_PATHS};
use crate::pwad_data::{PWadData, PWAD_HASHMAP};
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

        // Use case-insensitive search
        let options = MatchOptions::default();

        // Get list of WAD files in folders
        let iwad_hashmap = HashMap::from(IWAD_HASHMAP);
        let pwad_hashmap = HashMap::from(PWAD_HASHMAP);

        let mut iwad_list: Vec<(&IWadData, String)> = vec![];
        let mut pwad_list: Vec<(&PWadData, String)> = vec![];

        for path in IWAD_PATHS.iter().chain([&user_folder])
            .flat_map(|folder| glob_with(&format!("{folder}/*.wad"), options))
            .flat_map(|paths| paths.into_iter().flatten())
        {
            let filename = path.display().to_string();
            
            if let Ok(hash) = crc32(&filename) {
                if let Some(data) = iwad_hashmap.get(&hash) {
                    iwad_list.push((data, filename));
                } else if let Some(data) = pwad_hashmap.get(&hash) {
                    pwad_list.push((data, filename));
                }
            }
        }

        // Add IWADs to combo row
        let iwad_objects = iwad_list.into_iter()
            .map(|(iwad_data, filename)| {
                let (pwad_files, mut pwad_names): (Vec<&str>, Vec<&str>) = pwad_list.iter()
                    .filter(|(pwad_data, _)| pwad_data.id == iwad_data.id)
                    .map(|(pwad_data, filename)| (filename.as_str(), pwad_data.name))
                    .unzip();

                pwad_names.sort_unstable();
                pwad_names.dedup();

                IWadObject::new(iwad_data, &filename, &pwad_files, &pwad_names)
            })
            .collect::<Vec<IWadObject>>();

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
