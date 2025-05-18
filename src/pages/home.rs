use dioxus::prelude::*;

use crate::components::Footer;

/// The Home page component that will be rendered when the current route is `[Route::Home]`
#[component]
pub fn HomePage() -> Element {
    rsx! {
        Footer {}
    }
}
