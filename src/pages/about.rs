use dioxus::prelude::*;

use crate::components::Footer;

#[component]
pub fn AboutPage() -> Element {
  rsx! {
    Footer {}
  }
}