use dioxus::prelude::*;

use crate::routes::Route;

#[component]
pub fn Home() -> Element {
    // Initiate the asynchronous operation
    // Spawn the asynchronous task
    const IMAGE_RUST_CHAT: Asset = asset!("/assets/rust-chat-logo.png");

    rsx! {
        div {style:"
            padding-top:2rem;
            background-color: #000000;
            ",
            div {
                style: "
                display: flex;
                justify-content: flex-end;
                height: 40px;
                width: 100vw;
                background-color: #000000;
                padding: 0 5rem;
                ",
                Link {
                    to: Route::Login {},
                    style:"
                        color: #F97316; /* สีส้ม */
                        font-weight: bold;
                        text-decoration: none;
                        padding: 0.5rem 1rem;
                        border: 2px solid #F97316;
                        border-radius: 6px;
                        transition: background-color 0.3s ease;
                        ",
                    "Login"
                }
            }
        }
        section {
            style:
            "background-color: #000000;
            color: white;
            font-family: sans-serif;
            padding: 4rem 2rem;
            text-align: center;
            height: 100vh;
            width: 100vw;",
            img {
                src:"{IMAGE_RUST_CHAT}",
                alt: "Rust Chat Logo",
                style:
                "display: block;
                margin-left: auto;
                margin-right: auto;
                width: 240px;
                height: auto;
                margin-bottom: 2rem;
                "
            }
            div {
                style: "max-width: 800px; margin: 0 auto;",

                p {
                    style: "font-size: 1.25rem; margin-bottom: 0.5rem; color: #f3f4f6;",
                    "An open-source platform for multi-channel customer communication."
                }
                p {
                    style: "font-size: 1.25rem; margin-bottom: 0.5rem; color: #f3f4f6;",
                    "Built with Rust for unmatched speed and safety."
                }
                p {
                    style: "font-size: 1.25rem; margin-bottom: 0.5rem; color: #f3f4f6;",
                    "Connect seamlessly across chat, email, and social media."
                }
                p {
                    style: "font-size: 1.25rem; margin-bottom: 2rem; color: #f3f4f6;",
                    "Empowering businesses to respond faster and smarter."
                }
                a {
                    href: "https://github.com/lidm0707/RUST-CHAT",
                    style: "
                        background-color: #F97316;
                        color: white;
                        padding: 0.75rem 1.5rem;
                        border-radius: 8px;
                        text-decoration: none;
                        font-weight: bold;
                    ",
                    "View on GitHub"
                }
            }
        }
    }
}
