use std::cell::OnceCell;
use std::path::Path;

use gtk::{gio, glib};
use adw::subclass::prelude::*;
use adw::prelude::*;

use crate::engine_object::EngineObject;
use crate::iwad_object::IWADFlags;

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

        pub engine_list: OnceCell<Vec<EngineObject>>,
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

        let engine_list: Vec<EngineObject> = vec![
            EngineObject::new(
                "Chocolate Doom",
                "Historically-accurate Doom, Heretic, Hexen, and Strife port",
                IWADFlags::DOOM | IWADFlags::HERETIC | IWADFlags::HEXEN,
                "/usr/bin/chocolate-doom"
            ),
            EngineObject::new(
                "Crispy Doom",
                "Vanilla-compatible enhanced Doom engine",
                IWADFlags::DOOM | IWADFlags::HERETIC | IWADFlags::HEXEN,
                "/usr/bin/crispy-doom"
            ),
            EngineObject::new(
                "DSDA-Doom",
                "Fork of PrBoom+ with extra tooling for demo recording and playback, with a focus on speedrunning",
                IWADFlags::DOOM | IWADFlags::HERETIC | IWADFlags::HEXEN,
                "/usr/bin/dsda-doom"
            ),
            EngineObject::new(
                "GZDoom",
                "Feature centric port for all Doom engine games",
                IWADFlags::DOOM | IWADFlags::HERETIC | IWADFlags::HEXEN,
                "/usr/bin/gzdoom"
            ),
            EngineObject::new(
                "Nugget Doom",
                "Fork of Woof! with additional features",
                IWADFlags::DOOM,
                "/usr/bin/nugget-doom"
            ),
            EngineObject::new(
                "VKDoom",
                "VKDoom is a source port based on the DOOM engine with a focus on Vulkan and modern computers",
                IWADFlags::DOOM | IWADFlags::HERETIC | IWADFlags::HEXEN,
                "/usr/bin/vkdoom"
            ),
            EngineObject::new(
                "Woof!",
                "Woof! is a continuation of Lee Killough's Doom source port MBF targeted at modern systems",
                IWADFlags::DOOM,
                "/usr/bin/woof"
            ),
        ];

        imp.engine_list.set(engine_list).unwrap();
    }

    //-----------------------------------
    // Public init for IWAD function
    //-----------------------------------
    pub fn init_for_iwad(&self, iwad_flag: IWADFlags) {
        let imp = self.imp();

        // Get list of installed engines compatible with IWAD
        let engine_list = imp.engine_list.get().unwrap();

        let mut engine_objects = engine_list.clone().into_iter()
            .filter(|engine| {
                Path::new(&engine.path()).try_exists().unwrap_or_default() &&
                engine.games().contains(iwad_flag)
            }
        ).collect::<Vec<EngineObject>>();

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
