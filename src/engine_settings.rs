use std::cell::Cell;

use gtk::glib;
use gtk::subclass::prelude::*;
use gtk::prelude::ObjectExt;

//------------------------------------------------------------------------------
// MODULE: EngineSettings
//------------------------------------------------------------------------------
mod imp {
    use super::*;

    //-----------------------------------
    // Private structure
    //-----------------------------------
    #[derive(Default, glib::Properties)]
    #[properties(wrapper_type = super::EngineSettings)]
    pub struct EngineSettings {
        #[property(get, set, default = false, construct)]
        hires: Cell<bool>,
    }

    //-----------------------------------
    // Subclass
    //-----------------------------------
    #[glib::object_subclass]
    impl ObjectSubclass for EngineSettings {
        const NAME: &'static str = "EngineSettings";
        type Type = super::EngineSettings;
    }

    #[glib::derived_properties]
    impl ObjectImpl for EngineSettings {}
}

//------------------------------------------------------------------------------
// IMPLEMENTATION: EngineSettings
//------------------------------------------------------------------------------
glib::wrapper! {
    pub struct EngineSettings(ObjectSubclass<imp::EngineSettings>);
}

impl EngineSettings {
    //-----------------------------------
    // Public reset function
    //-----------------------------------
    pub fn reset(&self) {
        self.set_hires(false);
    }
}

impl Default for EngineSettings {
    //-----------------------------------
    // Default costructor
    //-----------------------------------
    fn default() -> Self {
        // Build EngineSettings
        glib::Object::builder()
            .build()
    }
}
