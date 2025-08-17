use dioxus::prelude::*;
use frontend::routes::Route; // <--- CHANGED: Use `futures` directly as per gloo-net docs
use tracing::{info, Level};

const TAILWIND_CSS: Asset = asset!("/assets/tailwind.css");

fn main() {
    dioxus_logger::init(Level::INFO).expect("failed to init logger");

    info!("started");
    dioxus::launch(App);
}
#[component]
fn App() -> Element {
    rsx! {
        document::Link { rel: "stylesheet", href: TAILWIND_CSS }
        Router::<Route> {}
    }
}
