use serde::{Deserialize, Serialize};

// Logo paths are relative to the crate root: /assets/images/{Logo Name}

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Sponsor {
    pub name: String,
    pub description: Option<String>,
    pub link: Option<String>,
    pub logo_file_name: String,
    pub amount_donated: u32
}

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd)]
pub struct Sponsors {
    pub creator: Vec<Sponsor>,
    pub mechanic: Vec<Sponsor>,
    pub engineer: Vec<Sponsor>
}

impl Sponsors {
    
    #[must_use]
    pub fn new() -> Self {
        Sponsors { creator: Vec::new(), mechanic: Vec::new(), engineer: Vec::new() }
    }
}