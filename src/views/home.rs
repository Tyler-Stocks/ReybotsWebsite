use dioxus::prelude::*;
use crate::views::Footer;

/// The Home page component that will be rendered when the current route is `[Route::Home]`
#[component]
pub fn Home() -> Element {
    rsx! {
        Footer {}
    }
}
