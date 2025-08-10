use crate::pages::authen::authen_page::Login;
use crate::pages::chat::chat_page::Chat;
use crate::pages::home::home_page::Home;
use dioxus::prelude::*;
use dioxus_logger::tracing::Instrument;
use wasm_bindgen::JsValue;

use web_sys::console;

#[derive(Debug, Clone, Routable, PartialEq)]
#[rustfmt::skip]
pub enum Route {
    #[layout(Content)]
    #[route("/")]
    Home {},
    #[route("/chat")]
    Chat {},
    #[route("/authen")]
    Login{}
}
#[component]
pub fn Content() -> Element {
    let route = use_route::<Route>();
    console::log_1(&JsValue::from_str(&format!(
        "Current route: {:?}",
        route.to_string()
    )));

    rsx! {
        if route.to_string() != "/" && route.to_string() != "/authen" {
            div {class:"flex",
                div { class: " p-3 bg-gray-800 text-white w-auto",
                    div { class: "text-orange-500", h1 { "RUSTCHAT" } }
                    div {
                        Link { to: Route::Home {}, "Home" }
                    }
                    div {
                        Link { to: Route::Chat {}, "Chat" }
                    }
                }
                Outlet::<Route> {}
            }
        } else {
            Outlet::<Route> {}
        }
    }
}
