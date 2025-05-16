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
fn EngineerSponsorLeft(
  name: &'static str,
  description: &'static str,
  image: Asset,
  link: Option<&'static str>,
) -> Element {
  rsx! {
    div {
      class: "EngineerSponsor EngineerSponsorLeft",

      img {
        src: image,
        if link.is_some() { a { href: link.unwrap(), target: "_blank", rel: "noreferrer" } }
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
fn EngineerSponsorRight(
  name: &'static str, 
  description: &'static str, 
  link: Option<&'static str>, 
  image: Asset
) -> Element {
  rsx! {
    div {
      class: "EngineerSponsor EngineerSponsorRight",
     
      div {
        class: "EngineerSponsorDescription",
        h3 { "{name}" }
        p { "{description}" }
      }
      
      img { 
        src: image,
        if link.is_some() { a { href: link.unwrap(), target: "_blank", rel: "noreferrer" } }
      }
    }
  }
}

#[component]
pub fn Sponsors() -> Element {
  let show_menu = use_context::<Signal<bool>>();
  let mounted = use_mounted();
  let size = use_size(mounted);

  rsx! {
    document::Stylesheet { href: CSS }

    main {
      onmounted: move |event| mounted.onmounted(event),
      class: "Sponsors",
      style: if *show_menu.read() { "overflow: hidden" } else { "overflow: initial"},
      section {
        class: "SponsorsHeader",
        div {
          class: "ThankYouContainer",
          h1 { "Thank You" }
          
          match size.width() {
            0.0..=1000.0 => {
              rsx! { 
                p {
                  r#"Lorem ipsum dolor sit amet, consectetuer adipiscing elit. Aenean commodo ligula eget dolor. Aenean massa. Cum sociis natoque penatibus et magnis dis parturient montes, nascetur ridiculus mus."#
                } 
              }
            } 
            1001.0.. => {
              rsx! {
                p {
                  r#"Lorem ipsum dolor sit amet, consectetuer adipiscing elit. Aenean commodo ligula eget dolor. Aenean massa. Cum sociis natoque penatibus et magnis dis parturient montes, nascetur ridiculus mus.
                  Donec quam felis, ultricies nec, pellentesque eu, pretium quis, sem. Nulla consequat massa quis enim. Donec pede justo, fringilla vel, aliquet nec, vulputate eget, arcu.
                  In enim justo, rhoncus ut, imperdiet a, venenatis vitae, justo. Nullam dictum felis eu pede mollis pretium. Integer tincidunt. Cras dapibus. Vivamus elementum semper nisi. Aenean vulputate eleifend tellus.
                  "#
                } 
              }   
            }
            _ => panic!("Invalid Screen Width!")
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
          class: "SponsorTierElementsContainer",
          EngineerSponsorRight {
             name: "Country Grocer",
            description: "They are a cool store.",
            image: COUNTRY_GROCER_LOGO,
            link: Some("https://www.countrygrocer.com/"),
          }
          EngineerSponsorLeft {
             name: "Bramley House Enterprises",
            description: "Bramley House Enterprises is a real estate holding company based in Victoria, BC.",
            image: BRAMELY_HOUSE_ENTERPRISES_LOGO,
            link: None,
          }
          EngineerSponsorRight {
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
          class: "SponsorTierElementContainer"
        } 
      }
    }
    Footer {}
  }
}