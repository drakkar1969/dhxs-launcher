use std::cell::{Cell, RefCell};

use gtk::glib;
use gtk::subclass::prelude::*;
use gtk::prelude::ObjectExt;

use crate::data::IWadFlags;

//------------------------------------------------------------------------------
// ENUM: EngineID
//------------------------------------------------------------------------------
#[derive(Default, Debug, Eq, PartialEq, Clone, Copy, glib::Enum)]
#[repr(u32)]
#[enum_type(name = "EngineID")]
pub enum EngineID {
    #[default]
    ChocolateDoom,
    CrispyDoom,
    DSDADoom,
    GZDoom,
    NuggetDoom,
    VKDoom,
    Woof,
}

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
        games: Cell<IWadFlags>,
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
    pub fn new(id: EngineID, name: &str, description: &str, games: IWadFlags, compatibility: u32, path: &str, heretic_path: Option<&str>, hexen_path: Option<&str>) -> Self {
        // Build IWadObject
        glib::Object::builder()
            .property("id", id)
            .property("name", name)
            .property("description", description)
            .property("games", games)
            .property("compatibility", compatibility)
            .property("path", path)
            .property("heretic-path", heretic_path)
            .property("hexen-path", hexen_path)
            .build()
    }
}
