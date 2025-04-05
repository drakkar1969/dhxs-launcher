use std::borrow::Cow;

use gtk::gio;
use gio::prelude::FileExt;

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
