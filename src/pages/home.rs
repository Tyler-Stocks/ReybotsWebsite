use dioxus::prelude::*;

use crate::components::Footer;

/// The Home page component that will be rendered when the current route is `[Route::Home]`
#[component]
#[allow(non_snake_case)]
pub fn HomePage() -> Element {
    rsx! {
        Footer {}
    }
}
