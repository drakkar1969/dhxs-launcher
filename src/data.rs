use gtk::glib;

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
// STRUCT: IWADData
//------------------------------------------------------------------------------
#[derive(Debug)]
pub struct IWADData {
    flag: IWADFlags,
    name: &'static str,
    version: &'static str,
}

impl IWADData {
    pub fn flag(&self) -> IWADFlags {
        self.flag
    }

    pub fn name(&self) -> &str {
        self.name
    }

    pub fn version(&self) -> &str {
        self.version
    }
}

//------------------------------------------------------------------------------
// GLOBAL: Constants
//------------------------------------------------------------------------------
pub const IWAD_HASHMAP: [(u32, IWADData); 6] = [
    (
        0xbf0eaac0,
        IWADData { flag: IWADFlags::DOOM, name: "Doom - The Ultimate Doom", version: "v1.9ud" }
    ),
    (
        0xec8725db,
        IWADData { flag: IWADFlags::DOOM, name: "Doom2", version: "v1.9" }
    ),
    (
        0xd4bb05c0,
        IWADData { flag: IWADFlags::DOOM, name: "Final Doom - TNT: Evilution", version: "v1.9 Fix" }
    ),
    (
        0x15cd1448,
        IWADData { flag: IWADFlags::DOOM, name: "Final Doom - The Plutonia Experiment", version: "v1.9 Fix" }
    ),
    (
        0x5b16049e,
        IWADData { flag: IWADFlags::HERETIC, name: "Heretic", version: "v1.3" }
    ),
    (
        0xdca9114c,
        IWADData { flag: IWADFlags::HEXEN, name: "Hexen", version: "v1.1" }
    ),
];
