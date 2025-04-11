use gtk::glib;

//------------------------------------------------------------------------------
// FLAGS: IWadID
//------------------------------------------------------------------------------
#[glib::flags(name = "IWadID")]
pub enum IWadID {
    DOOM          = 0b0000_0000_0000_0001,
    UDOOM         = 0b0000_0000_0000_0010,
    DOOM2         = 0b0000_0000_0000_0100,
    TNT           = 0b0000_0000_0000_1000,
    PLUTONIA      = 0b0000_0000_0001_0000,
    FREEDOOM1     = 0b0000_0000_0010_0000,
    FREEDOOM2     = 0b0000_0000_0100_0000,
    HERETIC       = 0b0000_0000_1000_0000,
    HEXEN         = 0b0000_0001_0000_0000,
    STRIFE        = 0b0000_0010_0000_0000,

    DOOM_ONLY     = Self::DOOM.bits() | Self::UDOOM.bits() | Self::DOOM2.bits() | Self::TNT.bits() | Self::PLUTONIA.bits() | Self::FREEDOOM1.bits() | Self::FREEDOOM2.bits(),
    ALL_NO_STRIFE = Self::DOOM_ONLY.bits() | Self::HERETIC.bits() | Self::HEXEN.bits(),
    ALL           = Self::ALL_NO_STRIFE.bits() | Self::STRIFE.bits(),
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
    id: IWadID,
    name: &'static str,
    version: &'static str,
}

impl IWadData {
    pub fn id(&self) -> IWadID {
        self.id
    }

    pub fn name(&self) -> &str {
        self.name
    }

    pub fn version(&self) -> &str {
        self.version
    }
}

//------------------------------------------------------------------------------
// IWAD DATA
//------------------------------------------------------------------------------
pub const IWAD_HASHMAP: [(u32, IWadData); 9] = [
    (
        0xbf0eaac0,
        IWadData { id: IWadID::UDOOM, name: "Doom - The Ultimate Doom", version: "v1.9ud" }
    ),
    (
        0xec8725db,
        IWadData { id: IWadID::DOOM2, name: "Doom2", version: "v1.9" }
    ),
    (
        0xd4bb05c0,
        IWadData { id: IWadID::TNT, name: "Final Doom - TNT: Evilution", version: "v1.9 Fix" }
    ),
    (
        0x15cd1448,
        IWadData { id: IWadID::PLUTONIA, name: "Final Doom - The Plutonia Experiment", version: "v1.9 Fix" }
    ),
    (
        0xde6ddb27,
        IWadData { id: IWadID::FREEDOOM1, name: "FreeDoom: Phase 1", version: "v0.12.1" }
    ),
    (
        0x212e1cf9,
        IWadData { id: IWadID::FREEDOOM2, name: "FreeDoom: Phase 2", version: "v0.12.1" }
    ),
    (
        0x5b16049e,
        IWadData { id: IWadID::HERETIC, name: "Heretic", version: "v1.3" }
    ),
    (
        0xdca9114c,
        IWadData { id: IWadID::HEXEN, name: "Hexen", version: "v1.1" }
    ),
    (
        0x4234ace5,
        IWadData { id: IWadID::STRIFE, name: "Strife", version: "v1.2-1.31" }
    ),
];
