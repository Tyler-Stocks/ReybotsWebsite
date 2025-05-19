use crate::Route;
use dioxus::prelude::*;
use dioxus_resize_observer::use_size;
use dioxus_use_mounted::use_mounted;

const NAVBAR_CSS: Asset = asset!("styles/navbar.css");
const REYBOTS_LOGO: Asset = asset!("assets/images/Reybots Logo.svg");

#[component]
#[allow(non_snake_case)]
fn SideMenuElementComponent(destination: Route, label: &'static str) -> Element {
    let mut show_side_menu: Signal<bool> = use_context::<Signal<bool>>();

    rsx! {
        Link {
            onclick: move |_| show_side_menu.set(false),
            to: destination,
            p { class: "SideMenuElement", "{label}" }
        }
    }
}

#[component]
#[allow(non_snake_case)]
fn NavHamburgerComponent() -> Element {
    let mut show_menu: Signal<bool> = use_context::<Signal<bool>>();

    let toggle_menu = move |_| {
        let show_menu_value: bool = *show_menu.read();
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
#[allow(non_snake_case)]
pub fn NavSideMenuComponent() -> Element {
    let show_menu: Signal<bool> = use_context::<Signal<bool>>();

    rsx! {
        NavHamburgerComponent {}
        div {
            class: if *show_menu.read() { "SideMenu SideMenuOpen" } else { "SideMenu SideMenuClosed" },
                SideMenuElementComponent {
                    destination: Route::AboutPage {},
                    label: "About"
                }
                SideMenuElementComponent {
                    destination: Route::SponsorsPage {},
                    label: "Sponsors"
                }
                SideMenuElementComponent {
                    destination: Route::SponsorUsPage {},
                    label: "Sponsor Us"
                }
                SideMenuElementComponent {
                    destination: Route::CompetitionsPage {},
                    label: "Competitions"
                }
                SideMenuElementComponent {
                    destination: Route::ContactPage {},
                    label: "Contact"
                }
        }
    }
}

#[component]
#[allow(non_snake_case)]
fn TopMenuElementComponent(destination: Route, label: &'static str) -> Element {
    let mut show_menu: Signal<bool> = use_context::<Signal<bool>>();

    rsx! {
        Link {
            onclick: move |_| show_menu.set(false),
            to: destination,
            p { class: "TopMenuElement", "{label}" }
        }
    }
}

#[component]
#[allow(non_snake_case)]
pub fn NavTopMenuComponent() -> Element {
    rsx! {
        div {
            class: "TopMenu",
            TopMenuElementComponent {
                destination: Route::HomePage {},
                label: "Home"
            }
            TopMenuElementComponent {
                destination: Route::SponsorsPage {},
                label: "Sponsors"
            }
            TopMenuElementComponent {
                destination: Route::SponsorUsPage {},
                label: "Sponsor Us"
            }
            TopMenuElementComponent {
                destination: Route::CompetitionsPage {},
                label: "Competitions"
            }
            TopMenuElementComponent {
                destination: Route::AboutPage {},
                label: "About"
            }
            TopMenuElementComponent {
                destination: Route::ContactPage {},
                label: "Contact"
            }
        }
    }
}

#[component]
#[allow(non_snake_case)]
pub fn NavComponent() -> Element {
    let mounted = use_mounted();
    let width: u64 = use_size(mounted).width() as u64;

    rsx! {
        document::Stylesheet { href: NAVBAR_CSS }

        nav {
            onmounted: move |event| mounted.onmounted(event),
            div {
                class: "LogoContainer",
                img { src: REYBOTS_LOGO }

                p {
                    match width {
                        0..1400    => "Reybots",
                        1400..1510 => "Reynolds Reybots",
                        1510..     => "Reynolds Reybots | #18840"
                    }
                }

            }

            match width {
               0..900 => rsx! { NavSideMenuComponent {} },
               900.. => rsx! { NavTopMenuComponent {}  },
            }
        }

        Outlet::<Route> {}
    }
}
