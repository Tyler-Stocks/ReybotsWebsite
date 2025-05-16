use std::fs::File;
use dioxus::logger::tracing;
use dioxus::prelude::*;
use crate::data_components::Sponsor;
use crate::views::Footer;

const CSS: Asset = asset!("/assets/styles/sponsors.css");

// Other photos
const TEAM_PHOTO: Asset = asset!("/assets/images/sponsor_us.avif");

const SPONSOR_LOGOS: [Asset; 3] = [
  asset!("/assets/images/Howell Data Systems Logo.avif", ImageAssetOptions::new().with_preload(true)),
  asset!("/assets/images/Country Grocer Logo.avif", ImageAssetOptions::new().with_preload(true)),
  asset!("/assets/images/Bramley House Enterprises Logo.avif", ImageAssetOptions::new().with_preload(true))
];

fn strip_file_hash_and_path(file_path: &String) -> String {
  let path_segments: Vec<&str> = file_path.split('/').collect();
  let file_name_with_hash: &str = path_segments[path_segments.len() - 1];
  let extension: &str = file_name_with_hash.split('.').last().unwrap();
  let file_name: &str = file_name_with_hash.split('-').collect::<Vec<_>>()[0];
  format!("{}.{}", file_name, extension).to_string()
}

#[component]
fn EngineerSponsor(
  name: String,
  description: String,
  image: String,
  link: String,
) -> Element {
  rsx! {
    div {
      class: "EngineerSponsor",

      if link.is_empty() {
         img { src: image}
      } else {
         a { href: link , target: "_blank", rel: "noreferer", img { src: image} }
      }

      div {
        class: "EngineerSponsorDescription",
        h3 { "{name}" },
        p { "{description}" }
      }
    }
  }
}

#[component]
pub fn Sponsors() -> Element {
  let show_menu = use_context::<Signal<bool>>();

  let sponsors_data = include_str!("../../data/sponsors.json");
  let sponsors: Vec<Sponsor> = serde_json::from_str(sponsors_data)?;

  let mut engineer_sponsors: Vec<Sponsor> = Vec::new();
  let mut mechanic_sponsors: Vec<Sponsor> = Vec::new();
  let mut creator_sponsors: Vec<Sponsor> = Vec::new();
  
  for mut sponsor in sponsors {
    for photo in SPONSOR_LOGOS {
      let raw_photo_path = photo.to_string();

      if strip_file_hash_and_path(&raw_photo_path) == sponsor.logo_file_name  {
         sponsor.logo_file_name = raw_photo_path; 
      }
    }  
    
    match sponsor.amount_donated {
      0..=499 => creator_sponsors.push(sponsor),
      500..=999 => mechanic_sponsors.push(sponsor),
      1000.. => engineer_sponsors.push(sponsor),
    }
  }

  rsx! {
    document::Stylesheet { href: CSS }

    main {
      class: "Sponsors",
      style: if *show_menu.read() { "overflow: hidden" } else { "overflow: initial"},
      section {
        class: "SponsorsHeader",
        div {
          class: "ThankYouContainer",
          h1 { "Thank You" }
        
          p {
            r#"Lorem ipsum dolor sit amet, consectetuer adipiscing elit. Aenean commodo ligula eget dolor. Aenean massa. Cum sociis natoque penatibus et magnis dis parturient montes, nascetur ridiculus mus.
            Donec quam felis, ultricies nec, pellentesque eu, pretium quis, sem. Nulla consequat massa quis enim. Donec pede justo, fringilla vel, aliquet nec, vulputate eget, arcu.
            In enim justo, rhoncus ut, imperdiet a, venenatis vitae, justo. Nullam dictum felis eu pede mollis pretium. Integer tincidunt. Cras dapibus. Vivamus elementum semper nisi. Aenean vulputate eleifend tellus.
            "#
          }
        }
        div {
          class: "SponsorsHeaderImageContainer",
          figure {
            img { src: TEAM_PHOTO, alt: "Team Image" }
            figcaption { "Team Photo At Houston World Championship 2025" }
          }
        }
      }
      section {
        class: "SponsorTier",
        div {
          class: "SponsorTierHeader",
          h2 { "Engineer" }
          p { "Anyone who has donated more than $1000!" },
        }

        div {
          class: "EngineerSponsors",

          for sponsor in engineer_sponsors {
            EngineerSponsor {
              name: sponsor.name,
              description: sponsor.description.expect("Engineer Sponsors Must Have A Description"),
              image: sponsor.logo_file_name,
              link: sponsor.link.unwrap_or(String::new()),
            }
          }
        }
      }
      section {
        class: "SponsorTier",
        div {
          class: "SponsorTierHeader",
          h2 { "Mechanic" }
          p { "Anyone who has donated more than $500!" }
        }
        div {
          class: "MechanicSponsors"
        } 
      }
    }
    Footer {}
  }
}