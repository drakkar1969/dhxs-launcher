use crate::iwad_data::IWadID;

//------------------------------------------------------------------------------
// CONST VARIABLES
//------------------------------------------------------------------------------
pub const GRAPHICS_PATH: &str = "/usr/share/d-launcher/graphics/";

//------------------------------------------------------------------------------
// GRAPHICS DATA
//------------------------------------------------------------------------------
pub const GRAPHICS_MAP: [(IWadID, &[&str]); 7] = [
    (
        IWadID::UDOOM,
        &["hires-doom-a.pk3", "hires-doom-b.pk3", "objects.pk3", "monsters.pk3", "jfo-udoom.pk3", "hud-stuff.pk3"]
    ),
    (
        IWadID::DOOM,
        &["hires-doom-a.pk3", "hires-doom-b.pk3", "objects.pk3", "monsters.pk3", "jfo-doom.pk3", "hud-stuff.pk3"]
    ),
    (
        IWadID::DOOM2,
        &["hires-doom-a.pk3", "hires-doom-b.pk3", "hires-doom2.pk3", "objects.pk3", "monsters.pk3", "jfo-doom2.pk3", "hud-stuff.pk3"]
    ),
    (
        IWadID::PLUTONIA,
        &["hires-doom-a.pk3", "hires-doom-b.pk3", "hires-doom2.pk3", "hires-plut.pk3", "objects.pk3", "monsters.pk3", "jfo-plut.pk3", "hud-stuff.pk3"]
    ),
    (
        IWadID::TNT,
        &["hires-doom-a.pk3", "hires-doom-b.pk3", "hires-doom2.pk3", "hires-tnt.pk3", "objects.pk3", "monsters.pk3", "jfo-tnt.pk3", "hud-stuff.pk3"]
    ),
    (
        IWadID::HERETIC,
        &["hires-heretic.pk3"]
    ),
    (
        IWadID::HEXEN,
        &["hires-hexen.pk3"]
    ),
];
