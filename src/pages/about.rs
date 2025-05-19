use dioxus::prelude::*;

use crate::components::FooterComponent;

#[component]
#[allow(non_snake_case)]
pub fn AboutPage() -> Element {
  rsx! {
    FooterComponent {}
  }
}