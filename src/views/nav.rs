use crate::Route;
use dioxus::prelude::*;
use dioxus_resize_observer::use_size;
use dioxus_use_mounted::use_mounted;


const NAVBAR_CSS: Asset = asset!("/assets/styles/navbar.css");
const REYBOTS_LOGO: Asset = asset!("assets/images/reybots_logo.svg");

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
                destination: Route::Home {},
                label: "Home"
            }
            TopMenuElement {
                destination: Route::Sponsors {},
                label: "Sponsors"
            }
            TopMenuElement {
                destination: Route::SponsorUs {},
                label: "Sponsor Us"
            }
            TopMenuElement {
                destination: Route::Competitions {},
                label: "Competitions"
            }
            TopMenuElement {
                destination: Route::About {},
                label: "About"
            }
            TopMenuElement {
                destination: Route::Contact {},
                label: "Contact"
            }
        }
    }
}

#[component]
pub fn Nav() -> Element {
    let mounted = use_mounted();
    let size = use_size(mounted);


    rsx! {
        document::Stylesheet { href: NAVBAR_CSS }

        nav {
            onmounted: move |event| mounted.onmounted(event),
            div {
                class: "LogoContainer",
                img { src: REYBOTS_LOGO }

                p {
                    match size.width() as u64 {
                        0..1400    => "Reybots",
                        1400..1510 => "Reynolds Reybots",
                        1510..     => "Reynolds Reybots | #18840"
                    }
                }

            }

            match size.width() as u64 {
               0..600 => rsx! { NavSideMenu {} },
               600.. => rsx! { NavTopMenu {}  },
            }
        }

        Outlet::<Route> {}
    }
}
