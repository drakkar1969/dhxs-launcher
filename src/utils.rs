use std::borrow::Cow;
use std::{fs, io};
use std::io::Read;

use gtk::gio;
use gio::prelude::FileExt;

use crc32fast::Hasher;

//------------------------------------------------------------------------------
// GLOBAL: Functions
//------------------------------------------------------------------------------
//---------------------------------------
// CRC-32 function
//---------------------------------------
pub fn crc32(file: &str) -> io::Result<u32> {
    let file = fs::File::open(file)?;

    let mut buffer = [0; 4096]; // buffer size: 4KB
    let mut reader = io::BufReader::new(file);

    let mut hasher = Hasher::new();
    
    loop {
        let bytes_read = reader.read(&mut buffer)?;

        if bytes_read == 0 {
            break;
        }

        hasher.update(&buffer[..bytes_read]);
    }

    let result = hasher.finalize();

    Ok(result)
}

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
