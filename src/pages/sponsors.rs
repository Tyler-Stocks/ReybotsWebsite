use dioxus::prelude::*;
use crate::data_types::*;

use crate::components::Footer;

const CSS: Asset = asset!("styles/sponsors.css");

// Other photos
const TEAM_PHOTO: Asset = asset!("/assets/images/sponsor_us.avif");

const IMAGE_OPTIONS: ImageAssetOptions = ImageAssetOptions::new()
    .with_preload(true)
    .with_avif();

/*
 * Missing sponsor informatinon:
 *      Tyler Mechanical:
 *           Link
 *      Corey Laprade Financial Ltd.:
 *           Link 
 *           Logo
 *      Ally and Dennis Guevin:
 *           Link
 *           Logo
 *      Bay Ride Electric:
 *           Link 
 *           Logo
 *      Schaffer & Sons Ltd:
 *           Link
 *           Logo
 *      Canadian Opinion Research Ltd.:
 *           Link
 *           Logo
 *      Coulson Aviation:
 *           Logo
 */

// Must be a slice, as Vec<Asset> requires an allocator, not available at compile time
const SPONSOR_LOGOS: [Asset; 8] = [
  // Engineer Sponsors
  asset!("/assets/images/HowellDataSystemsLogo.avif", IMAGE_OPTIONS),
  asset!("/assets/images/CountryGrocerLogo.avif", IMAGE_OPTIONS),
  asset!("/assets/images/BramleyHouseEnterprisesLogo.avif", IMAGE_OPTIONS),
  // Mechanic Sponsors
  asset!("/assets/images/Bernhardt Contracting Logo.png", IMAGE_OPTIONS),
  asset!("/assets/images/TylerMechanicalLogo.avif", IMAGE_OPTIONS),
  asset!("/assets/images/SenseEngineeringLogo.avif", IMAGE_OPTIONS),
  asset!("/assets/images/KnappetProjectsIncLogo.avif", IMAGE_OPTIONS),
  asset!("/assets/images/FronteraHomesLogo.avif", IMAGE_OPTIONS)
];

#[component]
#[allow(non_snake_case)]
fn EngineerSponsor(name: String, description: String, image: String, link: Option<String>) -> Element {
  rsx! {
    div {
      class: "EngineerSponsor",

      match link {
        Some(link) => rsx! { a { href: link, target: "_blank", rel: "noreferer", img { src: image} } },
        None       => rsx! { img { src: image} }
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
#[allow(non_snake_case)]
pub fn SponsorsPage() -> Element {
  let show_menu = use_context::<Signal<bool>>();

  let sponsors_data = include_str!("../../data/sponsors.json");
  let mut sponsors: Sponsors = serde_json::from_str(sponsors_data)?;

  sponsors.inject_logo_paths(Box::new(SPONSOR_LOGOS)); 

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

          for sponsor in sponsors.engineer {
            EngineerSponsor {
              name: sponsor.name,
              description: sponsor.description,
              image: sponsor.logo_file_name,
              link: sponsor.link,
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