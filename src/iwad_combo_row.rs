use std::cell::OnceCell;
use std::collections::HashMap;

use gtk::{gio, glib};
use adw::subclass::prelude::*;
use adw::prelude::*;

use glob::{glob_with, MatchOptions};

use crate::iwad_object::IWadObject;

//------------------------------------------------------------------------------
// MODULE: IWadComboRow
//------------------------------------------------------------------------------
mod imp {
    use super::*;

    //-----------------------------------
    // Private structure
    //-----------------------------------
    #[derive(Default, gtk::CompositeTemplate)]
    #[template(resource = "/com/github/D-Launcher/ui/iwad_combo_row.ui")]
    pub struct IWadComboRow {
        #[template_child]
        pub model: TemplateChild<gio::ListStore>,

        pub iwad_map: OnceCell<HashMap<&'static str, &'static str>>,
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

            self.obj().setup_data();
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
    // New function
    //-----------------------------------
    pub fn new() -> Self {
        glib::Object::builder().build()
    }

    //-----------------------------------
    // Setup data
    //-----------------------------------
    fn setup_data(&self) {
        let imp = self.imp();

        let iwad_map: HashMap<&str, &str> = [
            (
                "doom.wad",
                "The Ultimate Doom"
            ),
            (
                "doom2.wad",
                "Doom II: Hell on Earth"
            ),
            (
                "plutonia.wad",
                "Final Doom - The Plutonia Experiment"
            ),
            (
                "tnt.wad",
                "Final Doom - TNT: Evilution"
            ),
            (
                "freedoom1.wad",
                "Freedoom Phase 1"
            ),
            (
                "freedoom2.wad",
                "Freedoom Phase 2"
            ),
            (
                "heretic.wad",
                "Heretic"
            ),
            (
                "hexen.wad",
                "Hexen"
            )
        ]
        .into_iter()
        .collect();

        imp.iwad_map.set(iwad_map).unwrap();
    }

    //-----------------------------------
    // Public init from folder function
    //-----------------------------------
    pub fn init_from_folder(&self, folder: &str) {
        let imp = self.imp();

        let options = MatchOptions {
            case_sensitive: false,
            require_literal_separator: false,
            require_literal_leading_dot: false
        };

        if let Ok(entries) = glob_with(&format!("{folder}/*.wad"), options) {
            // Get list of IWADs in folder
            let iwad_map = imp.iwad_map.get().unwrap();

            let iwad_objects = entries.into_iter()
                .flatten()
                .filter_map(|entry| {
                    let filename = entry.file_name()
                        .and_then(|filename| filename.to_str())
                        .unwrap_or_default();

                    iwad_map.get(filename)
                        .map(|name| IWadObject::new(name, filename))
                })
                .collect::<Vec<_>>();

            // Add IWADs to combo row
            imp.model.splice(0, imp.model.n_items(), &iwad_objects);
        }
    }

    //-----------------------------------
    // Public selected iwad function
    //-----------------------------------
    pub fn selected_iwad(&self) -> Option<IWadObject> {
        self.selected_item()
            .and_downcast::<IWadObject>()
    }
}

impl Default for IWadComboRow {
    //-----------------------------------
    // Default constructor
    //-----------------------------------
    fn default() -> Self {
        Self::new()
    }
}
