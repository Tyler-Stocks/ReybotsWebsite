use crate::Route;
use dioxus::prelude::*;


const NAVBAR_CSS: Asset = asset!("/assets/styles/navbar.css");
const REYBOTS_LOGO: Asset = asset!("assets/images/reybots_logo.svg");

#[component]
fn NavBarButton(destination: Route, label: String) -> Element {
  rsx! {
    Link {
      to: destination,
      div {
        class: "ButtonContainer",
        p { "{label}" }
      }
    }
  }
}

#[component]
pub fn NavBar() -> Element {
    rsx! {
        document::Stylesheet { href: NAVBAR_CSS }

        nav {
            div {
                class: "LogoContainer",
                img { src: REYBOTS_LOGO }
                p { "Reybots" }
            }
            div {
                class: "ButtonsContainer",
                NavBarButton {
                    destination: Route::Home {},
                    label: "Home"
                }
                NavBarButton {
                    destination: Route::Sponsors {},
                    label: "Sponsors"
                }
                NavBarButton {
                    destination: Route::SponsorUs {},
                    label: "Sponsor Us"
                }
                NavBarButton {
                    destination: Route::Competitions { },
                    label: "Compeitions"
                }
                NavBarButton {
                    destination: Route::About { },
                    label: "About"
                }
            }
            div {
                class: "ButtonsContainer",
                NavBarButton {
                    destination: Route::Contact {},
                    label: "Contact"
                }
            }
        }

        Outlet::<Route> {}
    }
}
