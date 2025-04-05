use std::cell::{Cell, RefCell};

use gtk::glib;
use gtk::subclass::prelude::*;
use gtk::prelude::ObjectExt;

use crate::iwad_object::IWADFlags;

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
        games: Cell<IWADFlags>,
        #[property(get, set)]
        path: RefCell<String>,
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
    pub fn new(name: &str, description: &str, games: IWADFlags, path: &str) -> Self {
        // Build IWadObject
        glib::Object::builder()
            .property("name", name)
            .property("description", description)
            .property("games", games)
            .property("path", path)
            .build()
    }
}
