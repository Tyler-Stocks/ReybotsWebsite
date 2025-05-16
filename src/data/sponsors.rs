use serde::{Deserialize, Serialize};

// Logo paths are relative to the crate root: /assets/images/{Logo Name}

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Serialize, Deserialize)]
pub enum Sponsor {
    CreatorSponsor {
        name: String,
        link: String,
        logo_path: String,
        low_resolution_logo_path: String,
    },
    MechanicSponsor {
        name: String,
        link: String,
        logo_path: String,
        low_resolution_logo_path: String
    },
    EngineerSponsor {
        name: String,
        description: String,
        link: String,
        logo_path: String,
        low_resolution_logo_path: String,
    }
}