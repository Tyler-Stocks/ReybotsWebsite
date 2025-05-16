use serde::{Deserialize, Serialize};

// Logo paths are relative to the crate root: /assets/images/{Logo Name}

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Sponsor {
    name: String,
    description: String,
    link: String,
    logo_file_name: String,
    amount_donated: u32
}