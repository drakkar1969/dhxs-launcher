use gtk::glib;

//------------------------------------------------------------------------------
// FLAGS: IWadID
//------------------------------------------------------------------------------
#[glib::flags(name = "IWadID")]
pub enum IWadID {
    DOOM      = 0b0000_0001,
    HERETIC   = 0b0000_0010,
    HEXEN     = 0b0000_0100,
}

impl Default for IWadID {
    fn default() -> Self {
        Self::empty()
    }
}

//------------------------------------------------------------------------------
// STRUCT: IWadData
//------------------------------------------------------------------------------
#[derive(Debug)]
pub struct IWadData {
    flag: IWadID,
    name: &'static str,
    version: &'static str,
}

impl IWadData {
    pub fn flag(&self) -> IWadID {
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
pub const IWAD_HASHMAP: [(u32, IWadData); 6] = [
    (
        0xbf0eaac0,
        IWadData { flag: IWadID::DOOM, name: "Doom - The Ultimate Doom", version: "v1.9ud" }
    ),
    (
        0xec8725db,
        IWadData { flag: IWadID::DOOM, name: "Doom2", version: "v1.9" }
    ),
    (
        0xd4bb05c0,
        IWadData { flag: IWadID::DOOM, name: "Final Doom - TNT: Evilution", version: "v1.9 Fix" }
    ),
    (
        0x15cd1448,
        IWadData { flag: IWadID::DOOM, name: "Final Doom - The Plutonia Experiment", version: "v1.9 Fix" }
    ),
    (
        0x5b16049e,
        IWadData { flag: IWadID::HERETIC, name: "Heretic", version: "v1.3" }
    ),
    (
        0xdca9114c,
        IWadData { flag: IWadID::HEXEN, name: "Hexen", version: "v1.1" }
    ),
];
