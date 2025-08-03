use dioxus::prelude::*;
use frontend::routes::Route; // <--- CHANGED: Use `futures` directly as per gloo-net docs

const TAILWIND_CSS: Asset = asset!("/assets/tailwind.css");

fn main() {
    dioxus::launch(App);
}
#[component]
fn App() -> Element {
    rsx! {
        document::Link { rel: "stylesheet", href: TAILWIND_CSS }
        Router::<Route> {}
    }
}
