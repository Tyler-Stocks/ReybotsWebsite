use dioxus::prelude::*;
use crate::Route;

const CSS: Asset = asset!("/assets/styles/footer.css");

#[component]
pub fn Footer() -> Element {
    rsx! {
        document::Stylesheet { href: CSS }
        
        footer {
            p {
                class: "Header",
                "Interested in Funding Our Team?"
            }
            Link {
                to: Route::SponsorUs {},
                button {
                    class: "SponsorUsButton",
                    "Sponsor Us!"
                }
            }
            div {
                class: "FooterElements",
                div {
                    class: "EmailElement",
                    p {
                        class: "FooterSubHeader",
                        "Contact Us"
                    }
                    a {
                        href: "mailto:reynoldsreybots@gmail.com",
                        target: "_blank",
                        rel: "noreferrer",
                        p {
                            class: "Email",
                            "reynoldsreybots@gmail.com"
                        }
                    }
                }
                div {
                    class: "SocialMediaElement",
                    a {
                        href: "https://www.instagram.com/reynoldsreybots/",
                        target: "_blank",
                        rel: "noreferrer",
                        button {
                            class: "SocialMediaButton",
                            aria_label: "Instagram Link",
                            i { class: "fa-brands fa-instagram" }
                        }
                    }
                    a {
                        href: "https://www.facebook.com/Reybots/",
                        target: "_blank",
                        rel: "noreferrer",
                        button {
                            class: "SocialMediaButton",
                            aria_label: "Facebook Link",
                            i { class: "fa-brands fa-facebook" }
                        }
                    }
                }
            }
        }
    }
}