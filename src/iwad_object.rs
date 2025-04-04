use std::cell::RefCell;

use gtk::glib;
use gtk::subclass::prelude::*;
use gtk::prelude::ObjectExt;

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
    pub fn new(name: &str, iwad: &str) -> Self {
        // Build IWadObject
        glib::Object::builder()
            .property("name", name)
            .property("iwad", iwad)
            .build()
    }
}
