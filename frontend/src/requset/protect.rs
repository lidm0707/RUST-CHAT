use gloo_net::http::{Request, Response};
use gloo_net::Error;
use web_sys::RequestCredentials;

pub async fn request_protect_get() -> Result<Response, Error> {
    let response = Request::get("http://127.0.0.1:8997/api/me")
        .credentials(RequestCredentials::Include)
        .send()
        .await;
    response
}
