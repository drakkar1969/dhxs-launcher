use std::cell::{Cell, RefCell};

use gtk::glib;
use gtk::subclass::prelude::*;
use gtk::prelude::ObjectExt;

use crate::iwad_data::{IWadData, IWadID};

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
        id: Cell<IWadID>,
        #[property(get, set)]
        name: RefCell<String>,
        #[property(get, set)]
        version: RefCell<String>,
        #[property(get, set)]
        filename: RefCell<String>,
        #[property(get, set)]
        pwads: RefCell<String>,
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

    impl IWadObject {}
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
    pub fn new(data: &IWadData, filename: &str, pwads: &str) -> Self {
        // Build IWadObject
        glib::Object::builder()
            .property("id", data.id)
            .property("name", data.name)
            .property("version", data.version)
            .property("filename", filename)
            .property("pwads", pwads)
            .build()
    }
}
