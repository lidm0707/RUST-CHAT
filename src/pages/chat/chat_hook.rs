use std::rc::Rc;

use super::chat_model::MessageSender;
use dioxus::prelude::*;
use futures::{SinkExt, StreamExt};
use gloo_net::websocket::{futures::WebSocket, Message};

pub fn send_channel(msg_lr: Signal<Vec<MessageSender>>) -> Coroutine<String> {
    let mut msg_lr = msg_lr;

    use_coroutine(move |mut rx: UnboundedReceiver<String>| async move {
        let ws = WebSocket::open("wss://echo.websocket.org").unwrap();
        let (mut write, mut read) = ws.split();

        spawn(async move {
            while let Some(msg) = rx.next().await {
                let _ = write.send(Message::Text(msg)).await.unwrap();
            }
        });

        while let Some(msg) = read.next().await {
            if let Ok(Message::Text(text)) = msg {
                msg_lr.write().push(MessageSender::Server(text));
            }
        }
    })
}

#[derive(Debug, Clone, PartialEq, Eq, Props)]
pub struct ChatSignal {
    pub msg_type: Signal<Vec<MessageSender>>,
    pub text_msg: Signal<String>,
    pub scroll_to_msg: Signal<Option<Rc<MountedData>>>,
}

impl ChatSignal {
    pub fn new() -> Self {
        Self {
            msg_type: use_signal(|| Vec::new()),
            text_msg: use_signal(|| String::new()),
            scroll_to_msg: use_signal(|| None),
        }
    }
}
