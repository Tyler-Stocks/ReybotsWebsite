use dioxus::prelude::*;
use crate::views::Footer;

const CSS: Asset = asset!("assets/styles/sponsors.css");

const BRAMELY_HOUSE_ENTERPRISES_LOGO: Asset = asset!("assets/images/Bramley House Enterprises Logo.avif");
const COUNTRY_GROCER_LOGO: Asset = asset!("assets/images/Country Grocer Logo.avif");
const HOWELL_DATA_SYSTEMS_LOGO: Asset = asset!("assets/images/Howell Data Systems Logo.avif");
const TEAM_PHOTO: Asset = asset!("assets/images/sponsor_us.avif");

#[derive(Copy, Clone, PartialEq, PartialOrd)]
enum SponsorSide {
  LEFT,
  RIGHT
}

#[component]
fn EngineerSponsor(
  name: &'static str,
  description: &'static str,
  image: Asset,
  link: &'static str,
  side: SponsorSide
) -> Element {
  let className = match side {
    SponsorSide::LEFT  => "EngineerSponsor EngineerSponsorRight",
    SponsorSide::RIGHT => "EngineerSponsor EngineerSponsorLeft"
  };

  rsx! {
    div {
      class: className,

      match side {
        SponsorSide::LEFT => rsx! {
          if !link.is_empty() {
            a { href: link, target: "_blank", rel: "noreffere", img { src: image } }
          } else {
            img { src: image }
          }
          div {
            class: "EngineerSponsorDescription",
            h2 { "{name}" },
            p { "{description}" }
          }
        },
        SponsorSide::RIGHT => rsx! {
          div {
            class: "EngineerSponsorDescription",
            h2 { "{name}" }
            p { "{description}" }

          }
          if !link.is_empty() {
            a { href: link, target: "_blank", rel: "norferrer", img { src: image } }
          } else {
            img { src: image }
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
        div {
          class: "ThankYouContainer",
          h1 { "Thank You" }
          p {
            r#"Lorem ipsum dolor sit amet, consectetuer adipiscing elit. Aenean commodo ligula eget dolor. Aenean massa. Cum sociis natoque penatibus et magnis dis parturient montes, nascetur ridiculus mus.
            Donec quam felis, ultricies nec, pellentesque eu, pretium quis, sem. Nulla consequat massa quis enim. Donec pede justo, fringilla vel, aliquet nec, vulputate eget, arcu.
            In enim justo, rhoncus ut, imperdiet a, venenatis vitae, justo. Nullam dictum felis eu pede mollis pretium. Integer tincidunt. Cras dapibus. Vivamus elementum semper nisi. Aenean vulputate eleifend tellus.
            Aenean leo ligula, porttitor eu, consequat vitae, eleifend ac, enim. Aliquam lorem ante, dapibus in, viverra quis, feugiat a, tellus. Phasellus viverra nulla ut metus varius laoreet. Quisque rutrum.
            In enim justo, rhoncus ut, imperdiet a, venenatis vitae, justo. Nullam dictum felis eu pede mollis pretium. Integer tincidunt. Cras dapibus. Vivamus elementum semper nisi. Aenean vulputate eleifend tellus.
            Aenean leo ligula, porttitor eu, consequat vitae, eleifend ac, enim. Aliquam lorem ante, dapibus in, viverra quis, feugiat a, tellus. Phasellus viverra nulla ut metus varius laoreet. Quisque rutrum.
            "#
          }
        }
        figure {
          img { src: TEAM_PHOTO, alt: "Team Image" }
          figcaption { "Team Photo At Houston World Championship 2025" }
        }
      }
      section {
        class: "EngineerSection",
        div {
          class: "EngineerHeader",
          h1 { "Engineer" }
          p { "Anyone who has donated more than $1000!" }
        }

        div {
          class: "EngineerSponsors",
          EngineerSponsor {
            name: "Country Grocer",
            description: "They are a cool store. I remember this one time I bought a thing from them and I was like this thing is so country. But that kinda makes sense cause its the country grocer which is pretty cool",
            image: COUNTRY_GROCER_LOGO,
            link: "https://www.countrygrocer.com/",
            side: SponsorSide::RIGHT
          }
          EngineerSponsor {
            name: "Bramley House Enterprises",
            description: "They do be doing some epic houses n' stuff. They also gave us big money so that's pretty sick of them.",
            image: BRAMELY_HOUSE_ENTERPRISES_LOGO,
            link: "",
            side: SponsorSide::LEFT
          }
          EngineerSponsor {
            name: "Howell Data Systems",
            description: "They do data and other imporatant datistical things invloving data. They also do POS I think so that's pretty imporant",
            image: HOWELL_DATA_SYSTEMS_LOGO,
            link: "",
            side: SponsorSide::RIGHT
          }
        }
      }
      // section {
      //   class: "Mechanic",
      //   h1 { "Mechanic" }
      //   p { "Anyone who has donated!" }
      // }
    }
    Footer {}
  }
}