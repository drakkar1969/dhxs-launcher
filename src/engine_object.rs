use std::cell::{Cell, RefCell};

use gtk::glib;
use gtk::subclass::prelude::*;
use gtk::prelude::ObjectExt;

use crate::iwad_data::IWadID;
use crate::engine_data::{EngineData, EngineID};

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
        #[property(get, set, builder(EngineID::default()))]
        id: Cell<EngineID>,
        #[property(get, set)]
        name: RefCell<String>,
        #[property(get, set)]
        description: RefCell<String>,
        #[property(get, set)]
        games: Cell<IWadID>,
        #[property(get, set)]
        compatibility: Cell<u32>,
        #[property(get, set)]
        path: RefCell<String>,
        #[property(get, set, nullable)]
        heretic_path: RefCell<Option<String>>,
        #[property(get, set, nullable)]
        hexen_path: RefCell<Option<String>>,
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
        // Build IWadObject
        glib::Object::builder()
            .property("id", data.id())
            .property("name", data.name())
            .property("description", data.description())
            .property("games", data.games())
            .property("compatibility", data.compatibility())
            .property("path", data.path())
            .property("heretic-path", data.heretic_path())
            .property("hexen-path", data.hexen_path())
            .build()
    }
}
