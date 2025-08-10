use gloo_net::http::{Request, Response};
use gloo_net::Error;
use shared::models::auth_model::AuthModel;
use web_sys::RequestCredentials;

pub async fn request_authen(login_model: AuthModel) -> Result<Response, Error> {
    let json = serde_json::to_string(&login_model).unwrap();
    let resp = Request::post("http://127.0.0.1:8997/auth")
        .header("Content-Type", "application/json")
        .credentials(RequestCredentials::Include)
        .body(json);

    match resp {
        // Parse data from here, such as storing a response token
        Ok(req) => {
            let res = req.send().await;
            res
        }
        Err(err) => Err(err),
    }
}
