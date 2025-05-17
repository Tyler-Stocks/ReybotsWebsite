use dioxus::prelude::*;

use crate::components::Footer;

#[component]
pub fn About() -> Element {
  rsx! {
    Footer {}
  }
}