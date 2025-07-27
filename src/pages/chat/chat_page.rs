use dioxus::prelude::*;
use std::rc::Rc;

use super::chat_layout::Bg;
use crate::pages::chat::chat_hook::send_channel;
use crate::pages::chat::chat_model::MessageSender; // <--- CHANGED: Use `futures` directly as per gloo-net docs
                                                   // Shared navbar component.

#[component]
pub fn Chat() -> Element {
    let mut input_text = use_signal(|| String::new());
    let msg_lr = use_signal(|| Vec::<MessageSender>::new());
    let msg_lr_user = msg_lr.clone();
    let msg_lr_loop = msg_lr.clone();
    let header_element = use_signal(|| Option::<Rc<MountedData>>::None);
    let mut header_element_rc1 = header_element.clone();
    let mut header_element_rc2 = header_element.clone();
    // New: Effect to scroll to the bottom when messages change
    let _ = use_resource(move || async move {
        let _ = msg_lr.read();

        if let Some(header) = header_element.cloned() {
            header
                .scroll_to(ScrollBehavior::Smooth)
                .await
                .unwrap_or_default();
        }
    });

    let send = send_channel(msg_lr_loop);

    rsx! {
        div {
            class: "p-4 w-full",
            Bg{
                message_container:rsx!{
                    for msg in msg_lr_loop.read().iter() {
                        match msg {
                            MessageSender::User(msg) => {
                               rsx!{ p { class:"flex justify-end",
                                   onmounted: move |cx| {
                                   header_element_rc1.set(Some(cx.data()));
                               },"user : {msg}" }}
                            },
                            MessageSender::Server(msg) => {
                               rsx!{ p { class:"flex justify-start",
                                   onmounted: move |cx| {
                                   header_element_rc2.set(Some(cx.data()));
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
                value: "{input_text}",
                oninput: move |e| input_text.set(e.value().clone())
            }

            button {
                class: "ml-2 bg-blue text-white px-4 py-2 rounded ",
                onclick: move |_| {
                        let mut msgLRUser = msg_lr_user.clone();
                        send.send(input_text().to_string());
                        msgLRUser.write().push(MessageSender::User(input_text().to_string()));
                },
                "ส่งข้อความ"
            }



        }
    }
}
