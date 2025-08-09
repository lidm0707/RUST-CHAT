use crate::routes::Route;
use dioxus::prelude::*;

#[component]
pub fn Navbar() -> Element {
    rsx! {
        div{class:"flex", // maybe take another broken
            div {class:" p-3 bg-gray-800 text-white w-auto",
                div{class:"text-orange-500", h1{"RUSTCHAT"}}
                div{
                    Link {
                    to: Route::Home {},
                    "Home"
                }}

                div{
                Link {
                to: Route::Chat {},
                "Chat"
                }}
                div{}
            }
            Outlet::<Route> {}
        }

    }
}
