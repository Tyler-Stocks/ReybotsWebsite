use crate::Route;
use dioxus::prelude::*;

use crate::util::document::screen_width;

const NAVBAR_CSS: Asset = asset!("/assets/styles/navbar.css");
const REYBOTS_LOGO: Asset = asset!("assets/images/reybots_logo.svg");

#[component]
fn NavBarButton(destination: Route, label: &'static str) -> Element {
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
fn SideMenuElement(destination: Route, label: &'static str) -> Element {
    let mut show_side_menu = use_context::<Signal<bool>>();
    
    rsx! {
        Link {
            onclick: move |_| show_side_menu.set(false),
            to: destination,
            p { class: "SideMenuElement", "{label}" }
        }
    }
}

/// Navbar component to be shown when the screen width > 1200 px
#[component]
fn NavBar() -> Element {
    rsx! {
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
                label: "Competitions"
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
}

#[component]
fn NavHamburger() -> Element {
    let mut show_menu = use_context::<Signal<bool>>();

    let toggle_menu = move |_| {
        let show_menu_value = *show_menu.read();
        show_menu.set(!show_menu_value);
    };
    
    rsx! {
        svg {
            onclick: toggle_menu,
            class: if *show_menu.read() { "NavHamburger NavHamburgerOpen" } else { "NavHamburger" },
            width: 27,
            height: 22,
            view_box: "0 0 27 22",
            xmlns: "http://www.w3.org/2000/svg",
            rect { width: 27, height: 4, rx: 2 }
            rect { y: 18, width: 27, height: 4, rx: 2 }
            rect { x: 11, y: 9, width: 16, height: 4, rx: 2 }
        }
    }
}

#[component]
fn TopMenuElement(destination: Route, label: &'static str) -> Element {
    let mut show_menu = use_context::<Signal<bool>>();
    
    rsx! {
        Link {
            onclick: move |_| show_menu.set(false),
            to: destination,
            p { class: "TopMenuElement", "{label}" }
        } 
    } 
}

#[component]
pub fn NavTopMenu() -> Element {
    rsx! {
        div {
            class: "TopMenu",
            
            TopMenuElement {
                destination: Route::About {},
                label: "About"
            },
            TopMenuElement { 
                destination: Route::Sponsors {},
                label: "Sponsors"
            }
            TopMenuElement {
                destination: Route::SponsorUs {},
                label: "SponsorUs" 
            }
            TopMenuElement {
                destination: Route::Competitions {},
                label: "Competitions"
            }
            TopMenuElement {
                destination: Route::Contact {},
                label: "Contact"
            }
        }
    }
}

#[component]
pub fn NavSideMenu() -> Element {
    let show_menu = use_context::<Signal<bool>>();
    
    rsx! {
        NavHamburger {} 
        div {
            class: if *show_menu.read() { "SideMenu SideMenuOpen" } else { "SideMenu SideMenuClosed" },
                SideMenuElement {
                    destination: Route::About {},
                    label: "About"
                } 
                SideMenuElement {
                    destination: Route::Sponsors {},
                    label: "Sponsors"
                }
                SideMenuElement {
                    destination: Route::SponsorUs {},
                    label: "Sponsor Us"
                }
                SideMenuElement {
                    destination: Route::Competitions {},
                    label: "Competitions"
                }
                SideMenuElement {
                    destination: Route::Contact {},
                    label: "Contact"
                } 
        }
    }
}

#[component]
pub fn Nav() -> Element {
    rsx! {
        document::Stylesheet { href: NAVBAR_CSS }

        nav {
            div {
                class: "LogoContainer",
                img { src: REYBOTS_LOGO }
                p { "Reybots" }
            }

            match screen_width() {
               0..600 => rsx! { NavSideMenu {} },
               600..1200 => rsx! { NavTopMenu {}  },
               1200.. => rsx! { NavBar {} } 
            }
        }

        Outlet::<Route> {}
    }
}
