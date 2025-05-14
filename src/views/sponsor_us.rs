use dioxus::prelude::*;
use crate::views::Footer;

const CSS: Asset = asset!("assets/styles/sponsor_us.css");

const SPONSOR_US_IMAGE: Asset = asset!("/assets/images/sponsor_us.avif");
const SPONSOR_INSTRUCTIONS_PDF: Asset = asset!("/assets/other/Reynolds Reybots Donation Instructions.pdf");

#[component]
pub fn SponsorUs() -> Element {
  rsx! {
    document::Stylesheet { href: CSS }

    main {
      class: "SponsorUs",
      h1 { "Sponsor Us" }
      p { "PLS GIVE US MONEY" }
      div {
        class: "SponsorLink",
        a {
          href: SPONSOR_INSTRUCTIONS_PDF,
          target: "_blank" ,
          rel: "noreferrer",
          button { "Donation/Sponsorship Instructions" }
        }
      }
      div {
        class: "InformationForSponsors",
        img { src: SPONSOR_US_IMAGE }
      }
      h2 { "How You Can Help" }
      div {
        class: "FinancialInformationForSponsors",
        ul {
          li { "The most direct way to help us is financially, through one of the following avenues:" }
          ul {
            li { "Be a donor: this is eligible for tax receipts if done through the school district. Details can be found in the button down below and you will be included with other individuals and companies in the donor section of our webpage if you wish." }
            li { "Be a Sponsor: for Canada Revenue Agency Purposes this is separated from regular donors as it comes with a form of expected benefit. Sponsors are not eligible for a tax receipt through the school district." }
            li { "Pledge funds: One of our biggest financial difficulties is not knowing if we will qualify for world championships, and then having to frantically obtain funds. If you or your company are willing to pledge funds conditinally if we qualify, either as a donor or sponsor, please reach out to us directly." }
          }
        }
        p {
          class: "getStartedNotification",
          "If you would like more information, or have any questions about sponsoring us, please feel free to email us at reynoldsreybots@gmail.com!"
        }
      }
    }

    Footer {}
  }
}