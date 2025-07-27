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
