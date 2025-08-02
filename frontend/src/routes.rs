use crate::components::nav::Navbar;
use crate::pages::authen::authen_page::Login;
use crate::pages::chat::chat_page::Chat;
use crate::pages::home::home_page::Home;
use dioxus::prelude::*;

#[derive(Debug, Clone, Routable, PartialEq)]
#[rustfmt::skip]
pub enum Route {
    #[layout(Navbar)]
    #[route("/")]
    Home {},
    #[route("/chat")]
    Chat {},
    #[route("/authen")]
    Login{}
}
