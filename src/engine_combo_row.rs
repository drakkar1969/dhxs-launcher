use std::path::Path;

use gtk::{gio, glib};
use adw::subclass::prelude::*;
use adw::prelude::*;

use crate::engine_object::EngineObject;
use crate::iwad_data::IWadID;

//------------------------------------------------------------------------------
// MODULE: EngineComboRow
//------------------------------------------------------------------------------
mod imp {
    use super::*;

    //-----------------------------------
    // Private structure
    //-----------------------------------
    #[derive(Default, gtk::CompositeTemplate)]
    #[template(resource = "/com/github/D-Launcher/ui/engine_combo_row.ui")]
    pub struct EngineComboRow {
        #[template_child]
        pub(super) model: TemplateChild<gio::ListStore>,
    }

    //-----------------------------------
    // Subclass
    //-----------------------------------
    #[glib::object_subclass]
    impl ObjectSubclass for EngineComboRow {
        const NAME: &'static str = "EngineComboRow";
        type Type = super::EngineComboRow;
        type ParentType = adw::ComboRow;

        fn class_init(klass: &mut Self::Class) {
            EngineObject::ensure_type();

            klass.bind_template();
        }

        fn instance_init(obj: &glib::subclass::InitializingObject<Self>) {
            obj.init_template();
        }
    }

    impl ObjectImpl for EngineComboRow {
        //-----------------------------------
        // Constructor
        //-----------------------------------
        fn constructed(&self) {
            self.parent_constructed();
        }
    }

    impl WidgetImpl for EngineComboRow {}
    impl ListBoxRowImpl for EngineComboRow {}
    impl PreferencesRowImpl for EngineComboRow {}
    impl ActionRowImpl for EngineComboRow {}
    impl ComboRowImpl for EngineComboRow {}
}

//------------------------------------------------------------------------------
// IMPLEMENTATION: EngineComboRow
//------------------------------------------------------------------------------
glib::wrapper! {
    pub struct EngineComboRow(ObjectSubclass<imp::EngineComboRow>)
        @extends adw::ComboRow, adw::ActionRow, adw::PreferencesRow, gtk::ListBoxRow, gtk::Widget,
        @implements gtk::Accessible, gtk::Actionable, gtk::Buildable, gtk::ConstraintTarget;
}

impl EngineComboRow {
    //-----------------------------------
    // New function
    //-----------------------------------
    pub fn new() -> Self {
        glib::Object::builder().build()
    }

    //-----------------------------------
    // Public init for IWAD function
    //-----------------------------------
    pub fn init_for_iwad(&self, engine_list: &[EngineObject], iwad_id: IWadID) {
        let imp = self.imp();

        // Get list of installed engines compatible with IWAD
        let mut engine_objects = engine_list.iter()
            .filter(|engine| {
                Path::new(&engine.path()).try_exists().unwrap_or_default() &&
                engine.games().contains(iwad_id)
            }
        )
        .cloned()
        .collect::<Vec<EngineObject>>();

        engine_objects.sort_unstable_by_key(|engine| engine.name());

        imp.model.splice(0, imp.model.n_items(), &engine_objects);
    }

    //-----------------------------------
    // Public selected engine function
    //-----------------------------------
    pub fn selected_engine(&self) -> Option<EngineObject> {
        self.selected_item()
            .and_downcast::<EngineObject>()
    }

    //-----------------------------------
    // Public set selected engine name function
    //-----------------------------------
    pub fn set_selected_engine_name(&self, name: &str) {
        let index = self.imp().model.find_with_equal_func(|engine| {
            let engine = engine.downcast_ref::<EngineObject>()
                .expect("Must be a 'IWadObject'");

            engine.name() == name
        });

        self.set_selected(index.unwrap_or_default());
    }
}

impl Default for EngineComboRow {
    //-----------------------------------
    // Default constructor
    //-----------------------------------
    fn default() -> Self {
        Self::new()
    }
}
