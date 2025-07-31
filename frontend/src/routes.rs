use crate::components::nav::Navbar;
use crate::pages::chat::chat_page::Chat;
use crate::pages::home::home_page::Home;
use crate::pages::login::login_page::Login;
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
