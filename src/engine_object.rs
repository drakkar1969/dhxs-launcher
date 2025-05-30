use std::cell::{Cell, RefCell};

use gtk::glib;
use gtk::subclass::prelude::*;
use gtk::prelude::ObjectExt;

use crate::iwad_data::IWadID;
use crate::engine_data::{EngineData, EngineSource};
use crate::engine_settings::EngineSettings;

//------------------------------------------------------------------------------
// MODULE: EngineObject
//------------------------------------------------------------------------------
mod imp {
    use super::*;

    //-----------------------------------
    // Private structure
    //-----------------------------------
    #[derive(Default, glib::Properties)]
    #[properties(wrapper_type = super::EngineObject)]
    pub struct EngineObject {
        #[property(get, set)]
        name: RefCell<String>,
        #[property(get, set)]
        description: RefCell<String>,
        #[property(get, set)]
        games: Cell<IWadID>,
        #[property(get, set)]
        doom_path: RefCell<String>,
        #[property(get, set, nullable)]
        heretic_path: RefCell<Option<String>>,
        #[property(get, set, nullable)]
        hexen_path: RefCell<Option<String>>,
        #[property(get, set, nullable)]
        strife_path: RefCell<Option<String>>,
        #[property(get, set)]
        config_folder: RefCell<String>,
        #[property(get, set, builder(EngineSource::default()))]
        source: Cell<EngineSource>,

        #[property(get)]
        settings: RefCell<EngineSettings>,
    }

    //-----------------------------------
    // Subclass
    //-----------------------------------
    #[glib::object_subclass]
    impl ObjectSubclass for EngineObject {
        const NAME: &'static str = "EngineObject";
        type Type = super::EngineObject;
    }

    #[glib::derived_properties]
    impl ObjectImpl for EngineObject {}
}

//------------------------------------------------------------------------------
// IMPLEMENTATION: EngineObject
//------------------------------------------------------------------------------
glib::wrapper! {
    pub struct EngineObject(ObjectSubclass<imp::EngineObject>);
}

impl EngineObject {
    //-----------------------------------
    // New function
    //-----------------------------------
    pub fn new(data: &EngineData) -> Self {
        // Build EngineObject
        glib::Object::builder()
            .property("name", data.name)
            .property("description", data.description)
            .property("games", data.games)
            .property("doom-path", data.doom_path)
            .property("heretic-path", data.heretic_path)
            .property("hexen-path", data.hexen_path)
            .property("strife-path", data.strife_path)
            .property("config-folder", data.config_folder)
            .property("source", data.source)
            .build()
    }
}
