use dioxus::prelude::*;
use crate::views::Footer;

const CSS: Asset = asset!("/assets/styles/sponsors.css");

const BRAMELY_HOUSE_ENTERPRISES_LOGO: Asset = asset!("/assets/images/bramley_house_enterprises.avif");

#[component]
fn EngineerSponsor(
  name: &'static str, 
  description: &'static str, 
  image: Asset, 
  link: &'static str
) -> Element {
  rsx! {
    div {
      class: "EngineerSponsor",
      div {
        class: "EngineerSponsorImageContainer",
        h2 { "{name}" },
        img { src: image }
      },
      div {
        class: "EngineerSponsorDescriptionContainer",
        p { "{description}" }
        
        if !link.is_empty() {
          a { href: link, target: "_blank", rel: "noreferrer",
            button { "Check Them Out!" } 
          }  
        }
      }
    }
  }
}

#[component]
fn MechanicSponsor(image: Asset, link: &'static str) -> Element {
  rsx! {
    div {
      class: "MechanicSponsor",
      div {
        class: "MechanicSponsorImageContainer",
        img { src: image }
        a { href: link, target: "_blank", rel: "noreferrer" }
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
        h1 { "Our Sponsors: " }
        p { "We are truly thankful for all of our sponsors. You guys help us to build cool stuff and inspire others to do the same!" }
        p { "Want to join the list? Learn more about how you can help us on our mission:" }
        div { button { "Sponsor Us"} }
      }
      // section {
      //   class: "Engineer",
      //   h1 { "Engineer" }
      //   p { "Anyone who has donated more than $1000!" }
      //   div {
      //     class: "EngineerSponsors",
      //     EngineerSponsor {
      //       name: "BRAMLEY HOUSE ENTERPRISES",
      //       description: "Bramley House Enterprises is a real estate holding company based in Victoria, BC.",
      //       image: BRAMELY_HOUSE_ENTERPRISES_LOGO,
      //       link: ""
      //     }
      //   }
      // }
      // section {
      //   class: "Mechanic",
      //   h1 { "Mechanic" }
      //   p { "Anyone who has donated!" }
      // }
    }
    Footer {}
  }
}