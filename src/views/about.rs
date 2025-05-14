use dioxus::prelude::*;
use crate::views::Footer;

#[component]
pub fn About() -> Element {
  rsx! {
    Footer {}
  }
}