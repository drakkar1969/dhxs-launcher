use gtk::glib;

//------------------------------------------------------------------------------
// FLAGS: IWadID
//------------------------------------------------------------------------------
#[glib::flags(name = "IWadID")]
pub enum IWadID {
    DOOM      = 0b0000_0000_0000_0001,
    DOOM2     = 0b0000_0000_0000_0010,
    TNT       = 0b0000_0000_0000_0100,
    PLUTONIA  = 0b0000_0000_0000_1000,
    HERETIC   = 0b0000_0000_0001_0000,
    HEXEN     = 0b0000_0000_0010_0000,
    FREEDOOM1 = 0b0000_0000_0100_0000,
    FREEDOOM2 = 0b0000_0000_1000_0000,

    DOOMONLY  = 0b0000_0000_0000_1111,
    ALL       = 0b0000_0000_1111_1111,
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
// ENUM: EngineID
//------------------------------------------------------------------------------
#[derive(Default, Debug, Eq, PartialEq, Clone, Copy, glib::Enum)]
#[repr(u32)]
#[enum_type(name = "EngineID")]
pub enum EngineID {
    #[default]
    ChocolateDoom,
    CrispyDoom,
    DSDADoom,
    GZDoom,
    NuggetDoom,
    VKDoom,
    Woof,
}

//------------------------------------------------------------------------------
// STRUCT: EngineData
//------------------------------------------------------------------------------
#[derive(Debug)]
pub struct EngineData {
    id: EngineID,
    name: &'static str,
    description: &'static str,
    games: IWadID,
    compatibility: u32,
    path: &'static str,
    heretic_path: Option<&'static str>,
    hexen_path: Option<&'static str>
}

impl EngineData {
    pub fn id(&self) -> EngineID {
        self.id
    }

    pub fn name(&self) -> &str {
        self.name
    }

    pub fn description(&self) -> &str {
        self.description
    }

    pub fn games(&self) -> IWadID {
        self.games
    }

    pub fn compatibility(&self) -> u32 {
        self.compatibility
    }

    pub fn path(&self) -> &str {
        self.path
    }

    pub fn heretic_path(&self) -> Option<&str> {
        self.heretic_path
    }

    pub fn hexen_path(&self) -> Option<&str> {
        self.hexen_path
    }
}

//------------------------------------------------------------------------------
// IWAD DATA
//------------------------------------------------------------------------------
pub const IWAD_HASHMAP: [(u32, IWadData); 6] = [
    (
        0xbf0eaac0,
        IWadData { id: IWadID::DOOM, name: "Doom - The Ultimate Doom", version: "v1.9ud" }
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
        0x5b16049e,
        IWadData { id: IWadID::HERETIC, name: "Heretic", version: "v1.3" }
    ),
    (
        0xdca9114c,
        IWadData { id: IWadID::HEXEN, name: "Hexen", version: "v1.1" }
    ),
];

//------------------------------------------------------------------------------
// ENGINE DATA
//------------------------------------------------------------------------------
pub const ENGINE_ARRAY: [EngineData; 7] = [
    EngineData {
        id: EngineID::ChocolateDoom,
        name: "Chocolate Doom",
        description: "Historically-accurate Doom, Heretic, Hexen, and Strife port",
        games: IWadID::ALL,
        compatibility: 5,
        path: "/usr/bin/chocolate-doom",
        heretic_path: Some("/usr/bin/chocolate-heretic"),
        hexen_path: Some("/usr/bin/chocolate-hexen")
    },
    EngineData {
        id: EngineID::CrispyDoom,
        name: "Crispy Doom",
        description: "Vanilla-compatible enhanced Doom engine",
        games: IWadID::ALL,
        compatibility: 5,
        path: "/usr/bin/crispy-doom",
        heretic_path: Some("/usr/bin/crispy-heretic"),
        hexen_path: Some("/usr/bin/crispy-hexen")
    },
    EngineData {
        id: EngineID::DSDADoom,
        name: "DSDA-Doom",
        description: "Fork of PrBoom+ with extra tooling for demo recording and playback, with a focus on speedrunning",
        games: IWadID::ALL,
        compatibility: 5,
        path: "/usr/bin/dsda-doom",
        heretic_path: None,
        hexen_path: None
    },
    EngineData {
        id: EngineID::GZDoom,
        name: "GZDoom",
        description: "Feature centric port for all Doom engine games",
        games: IWadID::ALL,
        compatibility: 2,
        path: "/usr/bin/gzdoom",
        heretic_path: None,
        hexen_path: None
    },
    EngineData {
        id: EngineID::NuggetDoom,
        name: "Nugget Doom",
        description: "Fork of Woof! with additional features",
        games: IWadID::DOOMONLY,
        compatibility: 5,
        path: "/usr/bin/nugget-doom",
        heretic_path: None,
        hexen_path: None
    },
    EngineData {
        id: EngineID::VKDoom,
        name: "VKDoom",
        description: "VKDoom is a source port based on the DOOM engine with a focus on Vulkan and modern computers",
        games: IWadID::ALL,
        compatibility: 2,
        path: "/usr/bin/vkdoom",
        heretic_path: None,
        hexen_path: None
    },
    EngineData {
        id: EngineID::Woof,
        name: "Woof!",
        description: "Woof! is a continuation of Lee Killough's Doom source port MBF targeted at modern systems",
        games: IWadID::DOOMONLY,
        compatibility: 5,
        path: "/usr/bin/woof",
        heretic_path: None,
        hexen_path: None
    },
];
