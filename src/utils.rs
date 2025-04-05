use std::borrow::Cow;

use gtk::gio;
use gtk::gio::prelude::SettingsExt;

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
// Gsetting default value function
//-----------------------------------
pub fn gsetting_default_value(gsettings: &gio::Settings, key: &str) -> String {
    gsettings.default_value(key).unwrap().to_string().replace('\'', "")
}
