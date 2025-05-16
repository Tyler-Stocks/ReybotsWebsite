use dioxus::prelude::*;

use views::*;

mod views;
mod data_components;

#[derive(Debug, Clone, Routable, PartialEq)]
#[rustfmt::skip]
enum Route {
    #[layout(Nav)]
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
        About {},
}

const FAVICON: Asset = asset!("/assets/images/Reybots Logo.svg");
const CSS: Asset = asset!("/assets/styles/main.css");

fn main() {
    launch(App);
}

#[component]
#[allow(non_snake_case)]
fn App() -> Element {
    #[allow(unused_variables)]
    let show_menu: Signal<bool> = use_context_provider(|| Signal::new(false));

    rsx! {
        document::Link { rel: "icon", href: FAVICON }
        document::Stylesheet { href: CSS }

        head {
            // Configures font awesome, we use it for the icons found in the footer. https://fontawesome.com/
            link {
                rel: "stylesheet",
                href: "https://cdnjs.cloudflare.com/ajax/libs/font-awesome/6.2.0/css/all.min.css",
                integrity: "sha512-xh6O/CkQoPOWDdYTDqeRdPCVd1SpvCA9XXcUnZS2FmJNp1coAFzvtCN9BmamE+4aHK8yyUHUSCcJHgXloTyT2A==",
                crossorigin: "anonymous",
                referrerpolicy: "noreferrer"
            }
        }

        Router::<Route> {}
    }
}
