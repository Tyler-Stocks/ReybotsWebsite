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