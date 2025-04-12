use gtk::glib;

use crate::iwad_data::IWadID;

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
    hexen_path: Option<&'static str>,
    strife_path: Option<&'static str>,
    hires_capable: bool,
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

    pub fn strife_path(&self) -> Option<&str> {
        self.strife_path
    }

    pub fn hires_capable(&self) -> bool {
        self.hires_capable
    }
}

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
        hexen_path: Some("/usr/bin/chocolate-hexen"),
        strife_path: Some("/usr/bin/chocolate-strife"),
        hires_capable: false
    },
    EngineData {
        id: EngineID::CrispyDoom,
        name: "Crispy Doom",
        description: "Vanilla-compatible enhanced Doom engine",
        games: IWadID::ALL,
        compatibility: 5,
        path: "/usr/bin/crispy-doom",
        heretic_path: Some("/usr/bin/crispy-heretic"),
        hexen_path: Some("/usr/bin/crispy-hexen"),
        strife_path: Some("/usr/bin/crispy-strife"),
        hires_capable: false
    },
    EngineData {
        id: EngineID::DSDADoom,
        name: "DSDA-Doom",
        description: "Fork of PrBoom+ with extra tooling for demo recording and playback, with a focus on speedrunning",
        games: IWadID::ALL_NO_STRIFE,
        compatibility: 5,
        path: "/usr/bin/dsda-doom",
        heretic_path: None,
        hexen_path: None,
        strife_path: None,
        hires_capable: false
    },
    EngineData {
        id: EngineID::GZDoom,
        name: "GZDoom",
        description: "Feature centric port for all Doom engine games",
        games: IWadID::ALL,
        compatibility: 2,
        path: "/usr/bin/gzdoom",
        heretic_path: None,
        hexen_path: None,
        strife_path: None,
        hires_capable: true
    },
    EngineData {
        id: EngineID::NuggetDoom,
        name: "Nugget Doom",
        description: "Fork of Woof! with additional features",
        games: IWadID::DOOM_ONLY,
        compatibility: 5,
        path: "/usr/bin/nugget-doom",
        heretic_path: None,
        hexen_path: None,
        strife_path: None,
        hires_capable: false
    },
    EngineData {
        id: EngineID::VKDoom,
        name: "VKDoom",
        description: "VKDoom is a source port based on the DOOM engine with a focus on Vulkan and modern computers",
        games: IWadID::ALL,
        compatibility: 2,
        path: "/usr/bin/vkdoom",
        heretic_path: None,
        hexen_path: None,
        strife_path: None,
        hires_capable: true
    },
    EngineData {
        id: EngineID::Woof,
        name: "Woof!",
        description: "Woof! is a continuation of Lee Killough's Doom source port MBF targeted at modern systems",
        games: IWadID::DOOM_ONLY,
        compatibility: 5,
        path: "/usr/bin/woof",
        heretic_path: None,
        hexen_path: None,
        strife_path: None,
        hires_capable: false
    },
];
