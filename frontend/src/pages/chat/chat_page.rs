use dioxus::prelude::*;
// use gloo_net::http::Request;
use web_sys::console;

use super::chat_layout::Bg;
use crate::pages::chat::box_customer_short::BoxCustomerShort;
use crate::pages::chat::chat_hook::{send_channel, ChatSignal};
use crate::pages::chat::chat_model::MessageSender;
use crate::requset::protect::request_protect_get;
use crate::routes::Route; // <--- CHANGED: Use `futures` directly as per gloo-net docs
                          // Shared navbar component.

#[component]
pub fn Chat() -> Element {
    let mut chat_signal = ChatSignal::new();
    let _ = use_resource(move || async move {
        let _ = chat_signal.msg_type.read();

        if let Some(header) = chat_signal.scroll_to_msg.cloned() {
            header
                .scroll_to(ScrollBehavior::Smooth)
                .await
                .unwrap_or_default();
        }
    });

    let send = send_channel(chat_signal.msg_type);
    let navigator = use_navigator();
    let error_state = use_signal(|| None::<String>);
    console::log_1(&"ho".into());

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
        div {
            class: "p-4 w-full",
            Bg{
                left_config:rsx!{BoxCustomerShort{}},
                message_container:rsx!{
                    for msg in chat_signal.msg_type.read().iter() {
                        match msg {
                            MessageSender::User(msg) => {
                               rsx!{ p { class:"flex justify-end",
                                   onmounted: move |cx| {
                                   chat_signal.scroll_to_msg.set(Some(cx.data()));
                               },"user : {msg}" }}
                            },
                            MessageSender::Server(msg) => {
                               rsx!{ p { class:"flex justify-start",
                                   onmounted: move |cx| {
                                   chat_signal.scroll_to_msg.set(Some(cx.data()));
                               },"server: {msg}" }}
                            }
                        }
                    }

                },

                right_config:rsx!{div{" RIGHT"}}

            }

            span{ class:"m-2","💬" }
            input {
                class: "border rounded p-2 mt-4 mb-2 w-3/5 bg-white",
                placeholder: "พิมพ์ข้อความ...",
                value: "{chat_signal.text_msg}",
                oninput: move |e| chat_signal.text_msg.set(e.value().clone())
            }

            button {
                class: "ml-2 bg-blue text-white px-4 py-2 rounded ",
                onclick: move |_| {

                    send.send(chat_signal.text_msg.read().to_string());
                    chat_signal.msg_type.write().push(MessageSender::User(chat_signal.text_msg.read().to_string()));
                },
                "ส่งข้อความ"
            }




        }
    }
}
