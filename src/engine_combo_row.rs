use std::path::Path;

use gtk::{gio, glib};
use adw::subclass::prelude::*;
use adw::prelude::*;

use crate::engine_object::EngineObject;

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
        pub model: TemplateChild<gio::ListStore>,
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

            self.obj().setup_data();
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
    // Setup data
    //-----------------------------------
    fn setup_data(&self) {
        let imp = self.imp();

        let engines: Vec<EngineObject> = vec![
            EngineObject::new(
                "PrBoom+",
                "An advanced, Vanilla-compatible Doom engine based on PrBoom",
                ""
            ),
            EngineObject::new(
                "Woof!",
                "Woof! is a continuation of Lee Killough's Doom source port MBF targeted at modern systems",
                ""
            ),
            EngineObject::new(
                "Nugget Doom",
                "Fork of Woof! with additional features",
                ""
            ),
            EngineObject::new(
                "GZDoom",
                "Feature centric port for all Doom engine games",
                "/usr/bin/gzdoom"
            ),
            EngineObject::new(
                "Chocolate Doom",
                "Historically-accurate Doom, Heretic, Hexen, and Strife port",
                ""
            ),
        ];

        imp.model.splice(0, imp.model.n_items(), &engines.into_iter().filter(|engine| {
            Path::new(&engine.path()).try_exists().unwrap_or_default()
        }).collect::<Vec<EngineObject>>());
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
