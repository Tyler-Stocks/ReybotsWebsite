use dioxus::prelude::*;

use crate::components::FooterComponent;

/// The Home page component that will be rendered when the current route is `[Route::Home]`
#[component]
#[allow(non_snake_case)]
pub fn HomePage() -> Element {
    rsx! {
        FooterComponent {}
    }
}
