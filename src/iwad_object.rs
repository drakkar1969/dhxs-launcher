use std::cell::{Cell, RefCell};

use gtk::glib;
use gtk::subclass::prelude::*;
use gtk::prelude::ObjectExt;

//------------------------------------------------------------------------------
// FLAGS: IWADFlags
//------------------------------------------------------------------------------
#[glib::flags(name = "IWADFlags")]
pub enum IWADFlags {
    DOOM      = 0b0000_0001,
    HERETIC   = 0b0000_0010,
    HEXEN     = 0b0000_0100,
}

impl Default for IWADFlags {
    fn default() -> Self {
        Self::empty()
    }
}

//------------------------------------------------------------------------------
// MODULE: IWadObject
//------------------------------------------------------------------------------
mod imp {
    use super::*;

    //-----------------------------------
    // Private structure
    //-----------------------------------
    #[derive(Default, glib::Properties)]
    #[properties(wrapper_type = super::IWadObject)]
    pub struct IWadObject {
        #[property(get, set)]
        flag: Cell<IWADFlags>,
        #[property(get, set)]
        name: RefCell<String>,
        #[property(get, set)]
        iwad: RefCell<String>,
    }

    //-----------------------------------
    // Subclass
    //-----------------------------------
    #[glib::object_subclass]
    impl ObjectSubclass for IWadObject {
        const NAME: &'static str = "IWadObject";
        type Type = super::IWadObject;
    }

    #[glib::derived_properties]
    impl ObjectImpl for IWadObject {}
}

//------------------------------------------------------------------------------
// IMPLEMENTATION: IWadObject
//------------------------------------------------------------------------------
glib::wrapper! {
    pub struct IWadObject(ObjectSubclass<imp::IWadObject>);
}

impl IWadObject {
    //-----------------------------------
    // New function
    //-----------------------------------
    pub fn new(flag: IWADFlags, name: &str, iwad: &str) -> Self {
        // Build IWadObject
        glib::Object::builder()
            .property("flag", flag)
            .property("name", name)
            .property("iwad", iwad)
            .build()
    }
}
