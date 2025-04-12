use std::path::Path;

use gtk::{gio, glib};
use adw::subclass::prelude::*;
use adw::prelude::*;
use glib::clone;

use crate::engine_data::ENGINE_ARRAY;
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
        pub(super) settings_button: TemplateChild<gtk::MenuButton>,
        #[template_child]
        pub(super) settings_hires_switch: TemplateChild<gtk::Switch>,

        #[template_child]
        pub(super) model: TemplateChild<gio::ListStore>,
        #[template_child]
        pub(super) filter: TemplateChild<gtk::CustomFilter>,
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

            let obj = self.obj();

            obj.setup_signals();
            obj.setup_engines();
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
    // Setup signals
    //-----------------------------------
    fn setup_signals(&self) {
        let imp = self.imp();

        // Selected item property signal
        self.connect_selected_item_notify(clone!(
            #[weak(rename_to = row)] self,
            #[weak] imp,
            move |_| {
                if let Some(engine) = row.selected_engine() {
                    let hires_capable = engine.hires_capable();
                    let hires_active = engine.settings_hires();

                    imp.settings_hires_switch.set_visible(hires_capable);
                    imp.settings_hires_switch.set_active(hires_capable && hires_active);
                    imp.settings_button.set_sensitive(hires_capable);
                } else {
                    imp.settings_button.set_sensitive(false);
                }
            }
        ));

        // Settings hires switch active property signal
        imp.settings_hires_switch.connect_active_notify(clone!(
            #[weak(rename_to = row)] self,
            move |switch| {
                if let Some(engine) = row.selected_engine() {
                    engine.set_settings_hires(switch.is_active());
                }
            }
        ));
    }

    //-----------------------------------
    // Setup engines function
    //-----------------------------------
    fn setup_engines(&self) {
        let imp = self.imp();

        // Get list of installed engines
        let mut engine_objects = ENGINE_ARRAY.into_iter()
            .filter(|data| Path::new(&data.path()).try_exists().unwrap_or_default())
            .map(|data| EngineObject::new(&data))
            .collect::<Vec<EngineObject>>();

        engine_objects.sort_unstable_by_key(|engine| engine.name().to_ascii_lowercase());

        imp.model.splice(0, imp.model.n_items(), &engine_objects);
    }

    //-----------------------------------
    // Public filter engines function
    //-----------------------------------
    pub fn filter_engines(&self, iwad_id: Option<IWadID>) {
        let imp = self.imp();

        if let Some(id) = iwad_id {
            imp.filter.set_filter_func(move |item| {
                item
                    .downcast_ref::<EngineObject>()
                    .expect("Could not downcast to 'EngineObject'")
                    .games()
                    .intersects(id)
            });
        }
    }

    //-----------------------------------
    // Public reset engine settings function
    //-----------------------------------
    pub fn reset_engine_settings(&self) {
        let imp = self.imp();

        imp.model.iter::<EngineObject>()
            .flatten()
            .for_each(|engine| engine.set_settings_hires(false));
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
