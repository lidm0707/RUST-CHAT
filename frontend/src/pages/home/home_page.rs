use dioxus::prelude::*;
use web_sys::console;

use crate::{requset::protect::request_protect_get, routes::Route};

#[component]
pub fn Home() -> Element {
    let navigator = use_navigator();
    let error_state = use_signal(|| None::<String>);
    console::log_1(&"ho".into());
    // Initiate the asynchronous operation
    // Spawn the asynchronous task
    spawn(async move {
        if let Err(res) = request_protect_get().await {
            console::log_1(&format!("{:?}", res).into());
            error_state.clone().set(Some("wait".to_string()));
            error_state
                .clone()
                .set(Some("Failed to fetch data".to_string()));
        }
    });
    use_effect(move || {
        let error_state = error_state.clone();
        let _ = error_state.read();
        if let Some(_) = error_state.cloned() {
            navigator.push(Route::Login {});
        }
    });

    rsx! {
        div { class: "bg-gray-100 h-screen w-screen", "Welcome" }
        div{"{error_state :?}"}
    }
}
