use dioxus::prelude::Asset;
use serde::{Deserialize, Serialize};

// Logo paths are relative to the crate root: /assets/images/{Logo Name}

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreatorSponsor {
    pub name: String,
    pub link: Option<String>,
    pub logo_file_name: String,
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MechanicSponsor {
    pub name: String,
    pub link: Option<String>,
    pub logo_file_name: String,
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EngineerSponsor {
    pub name: String,
    pub link: Option<String>,
    pub logo_file_name: String,
    pub description: String
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Sponsors {
    pub creator: Vec<CreatorSponsor>,
    pub mechanic: Vec<MechanicSponsor>,
    pub engineer: Vec<EngineerSponsor>
}

impl Sponsors {

    fn strip_file_hash_and_path(file_path: String) -> String {
        let path_segments: Vec<&str> = file_path.split('/').collect();
        let file_name_with_hash: &str = path_segments[path_segments.len() - 1];
        let extension: &str = file_name_with_hash.split('.').last().unwrap();
        let file_name: &str = file_name_with_hash.split('-').collect::<Vec<_>>()[0];
        format!("{}.{}", file_name, extension).to_string()
    }
    
    pub fn inject_logo_paths(&mut self, logos: Box<[Asset]>) {
        self.creator.iter_mut()
            .for_each(
                |sponsor| {
                    if let Some(logo) = logos.iter().find(|logo| { Self::strip_file_hash_and_path(logo.to_string()) == sponsor.logo_file_name }) {
                        sponsor.logo_file_name = logo.to_string();
                    }
                }
            );
        
        self.mechanic.iter_mut()
            .for_each(
                |sponsor| {
                    if let Some(logo) = logos.iter().find(|logo| { Self::strip_file_hash_and_path(logo.to_string()) == sponsor.logo_file_name }) {
                        sponsor.logo_file_name = logo.to_string();
                    }
                }
            );

        self.engineer.iter_mut()
            .for_each(
                |sponsor| {
                    if let Some(logo) = logos.iter().find(|logo| { Self::strip_file_hash_and_path(logo.to_string()) == sponsor.logo_file_name }) {
                        sponsor.logo_file_name = logo.to_string();
                    }
                }
            );
    }
}