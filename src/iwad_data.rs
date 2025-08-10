use gtk::glib;

//------------------------------------------------------------------------------
// CONST VARIABLES
//------------------------------------------------------------------------------
pub const IWAD_PATHS: [&str; 4] = [
    "/usr/share/games/doom",
    "/usr/share/games/heretic",
    "/usr/share/games/hexen",
    "/usr/share/games/strife",
];

//------------------------------------------------------------------------------
// FLAGS: IWadID
//------------------------------------------------------------------------------
#[glib::flags(name = "IWadID")]
pub enum IWadID {
    #[flags_value(name = "Doom")]
    DOOM          = 0b0000_0000_0000_0001,
    #[flags_value(name = "Doom")]
    UDOOM         = 0b0000_0000_0000_0010,
    #[flags_value(name = "Doom II")]
    DOOM2         = 0b0000_0000_0000_0100,
    #[flags_value(name = "Final Doom")]
    PLUTONIA      = 0b0000_0000_0000_1000,
    #[flags_value(name = "Final Doom")]
    TNT           = 0b0000_0000_0001_0000,
    #[flags_value(name = "FreeDoom")]
    FREEDOOM1     = 0b0000_0000_0010_0000,
    #[flags_value(name = "FreeDoom")]
    FREEDOOM2     = 0b0000_0000_0100_0000,
    #[flags_value(name = "Heretic")]
    HERETIC       = 0b0000_0000_1000_0000,
    #[flags_value(name = "Hexen")]
    HEXEN         = 0b0000_0001_0000_0000,
    #[flags_value(name = "Strife")]
    STRIFE        = 0b0000_0010_0000_0000,

    #[flags_value(skip)]
    DOOM_ONLY     = Self::DOOM.bits() | Self::UDOOM.bits() | Self::DOOM2.bits() | Self::PLUTONIA.bits() | Self::TNT.bits() | Self::FREEDOOM1.bits() | Self::FREEDOOM2.bits(),
    #[flags_value(skip)]
    ALL_NO_STRIFE = Self::DOOM_ONLY.bits() | Self::HERETIC.bits() | Self::HEXEN.bits(),
    #[flags_value(skip)]
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
pub struct IWadData<'a> {
    pub id: IWadID,
    pub name: &'a str,
    pub version: &'a str,
}

//------------------------------------------------------------------------------
// IWAD DATA
//------------------------------------------------------------------------------
pub const IWAD_HASHMAP: [(u32, IWadData); 39] = [
    // DOOM / ULTIMATE DOOM ----------------------------------------------------
    (
        0x66457ab9,
        IWadData { id: IWadID::DOOM, name: "Doom", version: "v1.1" }
    ),
    (
        0xa5da8930,
        IWadData { id: IWadID::DOOM, name: "Doom", version: "v1.2" }
    ),
    (
        0xf756aab5,
        IWadData { id: IWadID::DOOM, name: "Doom", version: "v1.666" }
    ),
    (
        0x8d242df9,
        IWadData { id: IWadID::DOOM, name: "Doom", version: "v1.8" }
    ),
    (
        0x723e60f9,
        IWadData { id: IWadID::DOOM, name: "Doom", version: "v1.9" }
    ),
    (
        0xbf0eaac0,
        IWadData { id: IWadID::UDOOM, name: "Doom - The Ultimate Doom", version: "v1.9ud" }
    ),
    (
        0x5efa677e,
        IWadData { id: IWadID::UDOOM, name: "Doom - The Ultimate Doom", version: "v1.9ud (BFG Edition)" }
    ),
    (
        0x75c3b7bf,
        IWadData { id: IWadID::UDOOM, name: "Doom - The Ultimate Doom", version: "v1.9ud (Doom I Enhanced)" }
    ),
    (
        0xcff03d9f,
        IWadData { id: IWadID::UDOOM, name: "Doom - The Ultimate Doom", version: "Doom + Doom II" }
    ),
    (
        0xd5f8c089,
        IWadData { id: IWadID::UDOOM, name: "Doom - The Ultimate Doom", version: "Doom + Doom II" }
    ),
    // DOOM2 -------------------------------------------------------------------
    (
        0xc08005f7,
        IWadData { id: IWadID::DOOM2, name: "Doom II", version: "v1.666 (German)" }
    ),
    (
        0xe2a683bd,
        IWadData { id: IWadID::DOOM2, name: "Doom II", version: "v1.666" }
    ),
    (
        0x47daeb2e,
        IWadData { id: IWadID::DOOM2, name: "Doom II", version: "v1.7" }
    ),
    (
        0x952f6baa,
        IWadData { id: IWadID::DOOM2, name: "Doom II", version: "v1.7a" }
    ),
    (
        0x27eaae69,
        IWadData { id: IWadID::DOOM2, name: "Doom II", version: "v1.8 (French)" }
    ),
    (
        0x31bd3bc0,
        IWadData { id: IWadID::DOOM2, name: "Doom II", version: "v1.8" }
    ),
    (
        0xec8725db,
        IWadData { id: IWadID::DOOM2, name: "Doom II", version: "v1.9" }
    ),
    (
        0xdbaa4a2b,
        IWadData { id: IWadID::DOOM2, name: "Doom II", version: "v1.9 (PC-98)" }
    ),
    (
        0x927a778a,
        IWadData { id: IWadID::DOOM2, name: "Doom II", version: "v1.9 (BFG Edition)" }
    ),
    (
        0xf1d1ad55,
        IWadData { id: IWadID::DOOM2, name: "Doom II", version: "v1.9 (Doom II Enhanced)" }
    ),
    (
        0x09b8a6ae,
        IWadData { id: IWadID::DOOM2, name: "Doom II", version: "Doom + Doom II" }
    ),
    (
        0x151b8a96,
        IWadData { id: IWadID::DOOM2, name: "Doom II", version: "Doom + Doom II" }
    ),
    // FINAL DOOM --------------------------------------------------------------
    (
        0x48d1453c,
        IWadData { id: IWadID::PLUTONIA, name: "Final Doom - The Plutonia Experiment", version: "v1.9" }
    ),
    (
        0x15cd1448,
        IWadData { id: IWadID::PLUTONIA, name: "Final Doom - The Plutonia Experiment", version: "v1.9 (Fixed)" }
    ),
    (
        0x903dcc27,
        IWadData { id: IWadID::TNT, name: "Final Doom - TNT: Evilution", version: "v1.9" }
    ),
    (
        0xd4bb05c0,
        IWadData { id: IWadID::TNT, name: "Final Doom - TNT: Evilution", version: "v1.9 (Fixed)" }
    ),
    // FREEDOOM ----------------------------------------------------------------
    (
        0x070682b7,
        IWadData { id: IWadID::FREEDOOM1, name: "FreeDoom: Phase 1", version: "v0.12." }
    ),
    (
        0xde6ddb27,
        IWadData { id: IWadID::FREEDOOM1, name: "FreeDoom: Phase 1", version: "v0.12.1" }
    ),
    (
        0xe42df22f,
        IWadData { id: IWadID::FREEDOOM1, name: "FreeDoom: Phase 1", version: "v0.13.0" }
    ),
    (
        0xb66d9e8d,
        IWadData { id: IWadID::FREEDOOM2, name: "FreeDoom: Phase 2", version: "v0.12.0" }
    ),
    (
        0x212e1cf9,
        IWadData { id: IWadID::FREEDOOM2, name: "FreeDoom: Phase 2", version: "v0.12.1" }
    ),
    (
        0xa0bfeb53,
        IWadData { id: IWadID::FREEDOOM2, name: "FreeDoom: Phase 2", version: "v0.13.0" }
    ),
    // HERETIC -----------------------------------------------------------------
    (
        0x77482d1e,
        IWadData { id: IWadID::HERETIC, name: "Heretic", version: "v1.0" }
    ),
    (
        0x54759180,
        IWadData { id: IWadID::HERETIC, name: "Heretic", version: "v1.2" }
    ),
    (
        0x5b16049e,
        IWadData { id: IWadID::HERETIC, name: "Heretic: Shadow of the Serpent Riders", version: "v1.3" }
    ),
    // HEXEN -------------------------------------------------------------------
    (
        0xeece0236,
        IWadData { id: IWadID::HEXEN, name: "HeXen: Beyond Heretic", version: "v1.0" }
    ),
    (
        0xdca9114c,
        IWadData { id: IWadID::HEXEN, name: "HeXen: Beyond Heretic", version: "v1.1" }
    ),
    // STRIFE ------------------------------------------------------------------
    (
        0xb7581abd,
        IWadData { id: IWadID::STRIFE, name: "Strife", version: "v1.1" }
    ),
    (
        0x4234ace5,
        IWadData { id: IWadID::STRIFE, name: "Strife", version: "v1.2-1.31" }
    ),
];
