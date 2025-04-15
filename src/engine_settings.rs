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
        #[property(get, set, default = true, construct)]
        fullscreen: Cell<bool>,
        #[property(get, set, default = false, construct)]
        hires: Cell<bool>,
        #[property(get, set, default = false, construct)]
        lights: Cell<bool>,
        #[property(get, set, default = false, construct)]
        brightmaps: Cell<bool>,
        #[property(get, set, default = true, construct)]
        widescreen: Cell<bool>,
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
        self.set_fullscreen(true);
        self.set_hires(false);
        self.set_lights(false);
        self.set_brightmaps(false);
        self.set_widescreen(true);
    }
}

impl Default for EngineSettings {
    //-----------------------------------
    // Default costructor
    //-----------------------------------
    fn default() -> Self {
        // Build IWadObject
        glib::Object::builder()
            .build()
    }
}
