use crate::iwad_data::IWadID;

//------------------------------------------------------------------------------
// STRUCT: PWadData
//------------------------------------------------------------------------------
#[allow(dead_code)]
#[derive(Debug)]
pub struct PWadData<'a> {
    pub id: IWadID,
    pub name: &'a str,
    pub description: &'a str
}

//------------------------------------------------------------------------------
// PWAD DATA
//------------------------------------------------------------------------------
pub const PWAD_HASHMAP: [(u32, PWadData); 3] = [
    // SIGIL I+II --------------------------------------------------------------
    (
        0xf9216574,
        PWadData { id: IWadID::UDOOM, name: "SIGIL", description: "SIGIL main PWAD" }
    ),
    (
        0xb7679050,
        PWadData { id: IWadID::UDOOM, name: "SIGIL", description: "SIGIL compatibility PWAD" }
    ),
    (
        0xd210db36,
        PWadData { id: IWadID::UDOOM, name: "SIGIL II", description: "SIGIL II main PWAD" }
    ),
];
