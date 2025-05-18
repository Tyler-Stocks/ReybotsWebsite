use dioxus::prelude::*;

use crate::components::Footer;

#[component]
#[allow(non_snake_case)]
pub fn ContactPage() -> Element {
  rsx! {
      Footer {}
  }
}