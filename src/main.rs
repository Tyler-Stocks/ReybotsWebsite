use dioxus::prelude::*;

use views::{Home, Competitions, Sponsors, SponsorUs, Contact, About, NavBar};

mod views;

#[derive(Debug, Clone, Routable, PartialEq)]
#[rustfmt::skip]
enum Route {
    #[layout(NavBar)]
        #[route("/")]
        Home {},
        #[route("/competitions")]
        Competitions {},
        #[route("/sponsors")]
        Sponsors {},
        #[route("/sponsor_us")]
        SponsorUs {},
        #[route("/contact")]
        Contact {},
        #[route("/about")]
        About {}
}

const FAVICON: Asset = asset!("/assets/favicon.ico");
const CSS: Asset = asset!("/assets/styles/main.css");

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    rsx! {
        document::Link { rel: "icon", href: FAVICON }
        document::Stylesheet { href: CSS }
        Router::<Route> {}
    }
}
