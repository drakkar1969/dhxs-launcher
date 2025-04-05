use std::borrow::Cow;

use gtk::gio;
use gio::prelude::{FileExt, SettingsExt, SettingsExtManual};
use gio::glib::variant::{FromVariant, ToVariant};

//------------------------------------------------------------------------------
// GLOBAL: Functions
//------------------------------------------------------------------------------
//---------------------------------------
// Env expand function
//---------------------------------------
pub fn env_expand(path: &str) -> String {
    shellexpand::full(path)
        .unwrap_or(Cow::Borrowed(path))
        .to_string()
}

//-----------------------------------
// Path to file function
//-----------------------------------
pub fn path_to_file(path: &str) -> Option<gio::File> {
    (!path.is_empty()).then_some(gio::File::for_path(env_expand(path)))
}

//---------------------------------------
// File to path function
//---------------------------------------
pub fn file_to_path(file: &gio::File) -> String {
    file.path()
        .map(|path| path.display().to_string())
        .unwrap_or_default()
}

//-----------------------------------
// Gsetting default value function
//-----------------------------------
pub fn gsetting_default_value(gsettings: &gio::Settings, key: &str) -> String {
    gsettings.default_value(key).unwrap().to_string().replace('\'', "")
}

//---------------------------------------
// Set gsetting helper function
//---------------------------------------
pub fn set_gsetting<T: FromVariant + ToVariant + PartialEq>(gsettings: &gio::Settings, key: &str, value: &T) {
    let default: T = gsettings.default_value(key)
        .expect("Could not get gsettings default value")
        .get::<T>()
        .expect("Could not retrieve value from variant");

    if !(default == *value && default == gsettings.get(key)) {
        gsettings.set(key, value.to_variant()).unwrap();
    }
}
