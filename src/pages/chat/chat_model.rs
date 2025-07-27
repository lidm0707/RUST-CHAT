use std::{fmt::Debug, rc::Rc};

use dioxus::prelude::*;

#[derive(Debug)]
pub enum MessageSender {
    Server(String),
    User(String),
}

#[derive(Debug, Clone, PartialEq, Eq, Props)]
pub struct ChatModel {
    pub msg_type: Signal<Vec<MessageSender>>,
    pub text_msg: Signal<String>,
    pub scroll_to_msg: Signal<Option<Rc<MountedData>>>,
}

impl ChatModel {
    pub fn new() -> Self {
        Self {
            msg_type: use_signal(|| Vec::new()),
            text_msg: use_signal(|| String::new()),
            scroll_to_msg: use_signal(|| None),
        }
    }
}
