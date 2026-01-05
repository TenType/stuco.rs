use crate::Route;
use dioxus::prelude::*;

const MOBILE_BG: Asset = asset!("/assets/crab-and-fisherman.jpeg");
const DESKTOP_BG: Asset = asset!("/assets/crab-and-fisherman-16x9.jpeg");

#[component]
pub fn Home() -> Element {
    rsx! {
        document::Title { "Intro to Rust Lang" }
        div {
            class: "-mt-16 h-[90vh] w-full bg-cover bg-center flex items-center justify-center shadow-2xl bg-(image:--bg-mobile) lg:bg-(image:--bg-desktop)",
            style: format!("--bg-mobile: url('{}'); --bg-desktop: url('{}')", MOBILE_BG, DESKTOP_BG),
            div { class: "bg-black/25 text-primary text-center px-8 py-6",
                h1 { class: "text-4xl sm:text-6xl font-bold", "Intro to Rust Lang" }
                p { class: "text-xl sm:text-2xl mt-2", "Spring 2026" }
            }
        }
        div { class: "max-w-prose mx-auto px-8 pt-16",
            h2 { class: "text-3xl text-center font-bold italic text-primary mb-6",
                "Intro to Rust Lang"
            }
            p {
                "Welcome to Intro to Rust Lang (98-008). The course will be offered in Spring 2026 by Stephen Mao, Hugo Latendresse, and Anish Pallati at Carnegie Mellon University. Please see the "
                Link { to: Route::About {}, class: "text-secondary", "about page" }
                " for more!"
            }
        }
    }
}
