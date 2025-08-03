use dioxus::logger::tracing::info;
use dioxus::prelude::*;
use gloo_net::http::Request;

use super::chat_layout::Bg;
use crate::pages::chat::box_customer_short::BoxCustomerShort;
use crate::pages::chat::chat_hook::{send_channel, ChatSignal};
use crate::pages::chat::chat_model::MessageSender;
use crate::routes::Route; // <--- CHANGED: Use `futures` directly as per gloo-net docs
                          // Shared navbar component.

#[component]
pub fn Chat() -> Element {
    let navigator = use_navigator();
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
                    info!("click send");

                    send.send(chat_signal.text_msg.read().to_string());
                    chat_signal.msg_type.write().push(MessageSender::User(chat_signal.text_msg.read().to_string()));
                },
                "ส่งข้อความ"
            }
            button {
                class: "ml-2 bg-blue text-white px-4 py-2 rounded hover:bg-red-500",
                onclick: move |_| {
                    println!("click help");
                    spawn(async move {
                        let resp = Request::get("http://127.0.0.1:8997/hello")
                            .header("Content-Type", "application/json")
                            .json("");

                        match resp {
                            // Parse data from here, such as storing a response token
                            Ok(req) => {
                                let res = req.send().await;
                                match res {
                                    Ok(response) => {
                                        let response_text = response.text().await.unwrap();
                                        println!("Response: {}", response_text);
                                        navigator.push(Route::Home {});
                                    }
                                    Err(err) => {
                                        println!("Error: {}", err);
                                    }
                                }
                            }

                            //Handle any errors from the fetch here
                            Err(_err) => {
                                println!("Login failed - you need a login server running on localhost:8080.")
                            }
                        }
                    });
                },
                "HELLO"
            }



        }
    }
}
