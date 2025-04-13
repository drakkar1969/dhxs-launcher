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
// ENUM: EngineSource
//------------------------------------------------------------------------------
#[derive(Default, Debug, Eq, PartialEq, Clone, Copy, glib::Enum)]
#[repr(u32)]
#[enum_type(name = "EngineSource")]
pub enum EngineSource {
    #[default]
    Chocolate,
    PrBoom,
    WinMBF,
    ZDoom,
}

//------------------------------------------------------------------------------
// STRUCT: EngineData
//------------------------------------------------------------------------------
#[derive(Debug)]
pub struct EngineData<'a> {
    pub id: EngineID,
    pub name: &'a str,
    pub description: &'a str,
    pub games: IWadID,
    pub path: &'a str,
    pub heretic_path: Option<&'a str>,
    pub hexen_path: Option<&'a str>,
    pub strife_path: Option<&'a str>,
    pub source: EngineSource,
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
        path: "/usr/bin/chocolate-doom",
        heretic_path: Some("/usr/bin/chocolate-heretic"),
        hexen_path: Some("/usr/bin/chocolate-hexen"),
        strife_path: Some("/usr/bin/chocolate-strife"),
        source: EngineSource::Chocolate,
    },
    EngineData {
        id: EngineID::CrispyDoom,
        name: "Crispy Doom",
        description: "Vanilla-compatible enhanced Doom engine",
        games: IWadID::ALL,
        path: "/usr/bin/crispy-doom",
        heretic_path: Some("/usr/bin/crispy-heretic"),
        hexen_path: Some("/usr/bin/crispy-hexen"),
        strife_path: Some("/usr/bin/crispy-strife"),
        source: EngineSource::Chocolate,
    },
    EngineData {
        id: EngineID::DSDADoom,
        name: "DSDA-Doom",
        description: "Fork of PrBoom+ with extra tooling for demo recording and playback, with a focus on speedrunning",
        games: IWadID::ALL_NO_STRIFE,
        path: "/usr/bin/dsda-doom",
        heretic_path: None,
        hexen_path: None,
        strife_path: None,
        source: EngineSource::PrBoom,
    },
    EngineData {
        id: EngineID::GZDoom,
        name: "GZDoom",
        description: "Feature centric port for all Doom engine games",
        games: IWadID::ALL,
        path: "/usr/bin/gzdoom",
        heretic_path: None,
        hexen_path: None,
        strife_path: None,
        source: EngineSource::ZDoom,
    },
    EngineData {
        id: EngineID::NuggetDoom,
        name: "Nugget Doom",
        description: "Fork of Woof! with additional features",
        games: IWadID::DOOM_ONLY,
        path: "/usr/bin/nugget-doom",
        heretic_path: None,
        hexen_path: None,
        strife_path: None,
        source: EngineSource::WinMBF,
    },
    EngineData {
        id: EngineID::VKDoom,
        name: "VKDoom",
        description: "VKDoom is a source port based on the DOOM engine with a focus on Vulkan and modern computers",
        games: IWadID::ALL,
        path: "/usr/bin/vkdoom",
        heretic_path: None,
        hexen_path: None,
        strife_path: None,
        source: EngineSource::ZDoom,
    },
    EngineData {
        id: EngineID::Woof,
        name: "Woof!",
        description: "Woof! is a continuation of Lee Killough's Doom source port MBF targeted at modern systems",
        games: IWadID::DOOM_ONLY,
        path: "/usr/bin/woof",
        heretic_path: None,
        hexen_path: None,
        strife_path: None,
        source: EngineSource::WinMBF,
    },
];
