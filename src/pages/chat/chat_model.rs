#[derive(Debug)]
pub enum MessageSender {
    Server(String),
    User(String),
}
