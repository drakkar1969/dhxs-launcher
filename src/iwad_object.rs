use std::cell::{Cell, RefCell};
use std::path::Path;

use gtk::glib;
use gtk::subclass::prelude::*;
use gtk::prelude::ObjectExt;

use crate::data::{IWADData, IWADFlags};

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
        version: RefCell<String>,
        #[property(get, set)]
        filename: RefCell<String>,

        #[property(get = Self::basename)]
        _basename: RefCell<String>,
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

    impl IWadObject {
        //-----------------------------------
        // Basename property getter
        //-----------------------------------
        fn basename(&self) -> String {
            let filename = self.filename.borrow();

            Path::new(filename.as_str()).file_name().unwrap().to_string_lossy().into_owned()
        }
    }
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
    pub fn new(data: &IWADData, filename: &str) -> Self {
        // Build IWadObject
        glib::Object::builder()
            .property("flag", data.flag())
            .property("name", data.name())
            .property("version", data.version())
            .property("filename", filename)
            .build()
    }
}
