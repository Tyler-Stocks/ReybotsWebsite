use dioxus::prelude::*;
use dioxus_resize_observer::use_size;
use dioxus_use_mounted::use_mounted;
use crate::views::Footer;

const CSS: Asset = asset!("/assets/styles/sponsors.css");

// Engineer Tier Sponsors
const BRAMELY_HOUSE_ENTERPRISES_LOGO: Asset = asset!("/assets/images/Bramley House Enterprises Logo.avif");
const COUNTRY_GROCER_LOGO: Asset = asset!("/assets/images/Country Grocer Logo.avif");
const HOWELL_DATA_SYSTEMS_LOGO: Asset = asset!("/assets/images/Howell Data Systems Logo.avif");

// Mechanic Tier Sponsors
const BERNHARDT_CONTRACTING_LOGO: Asset = asset!("/assets/images/Bernhardt Contracting Logo.png");

// Other photos
const TEAM_PHOTO: Asset = asset!("/assets/images/sponsor_us.avif");

#[component]
fn EngineerSponsor(
  name: &'static str,
  description: &'static str,
  image: Asset,
  link: Option<&'static str>,
) -> Element {
  rsx! {
    div {
      class: "EngineerSponsor",

      match link {
        Some(url) => rsx! { a { href: url, target: "_blank", rel: "noreferer", img { src: image} } },
        None      => rsx! { img { src: image} }
      }

      // if link.is_some() {
      //   img { src: image}
      // } else {
      //   a { href: link, target: "_blank", rel: "noreferrer", img { src: image} }
      // }

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
          EngineerSponsor {
             name: "Country Grocer",
            description: "They are a cool store.",
            image: COUNTRY_GROCER_LOGO,
            link: Some("https://www.countrygrocer.com/"),
          }
          EngineerSponsor {
             name: "Bramley House Enterprises",
            description: "Bramley House Enterprises is a real estate holding company based in Victoria, BC.",
            image: BRAMELY_HOUSE_ENTERPRISES_LOGO,
            link: None,
          }
          EngineerSponsor {
            name: "Howell Data Systems",
            description: "Whatever business challenge you need to address, HDS offers all of the necessary components for your business to excel in today's rapidly-evolving retail environment.",
            image: HOWELL_DATA_SYSTEMS_LOGO,
            link: Some("https://www.howelldatasystems.com/"),
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