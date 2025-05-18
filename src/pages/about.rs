use dioxus::prelude::*;

use crate::components::Footer;

#[component]
#[allow(non_snake_case)]
pub fn AboutPage() -> Element {
  rsx! {
    Footer {}
  }
}