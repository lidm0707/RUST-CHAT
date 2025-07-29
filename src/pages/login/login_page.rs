use dioxus::prelude::*;
use gloo_net::http::Request;

use crate::pages::login::login_model::LoginModel;
#[component]
pub fn Login() -> Element {
    let onsubmit = move |evt: FormEvent| {
        spawn(async move {
            let username = evt.values()["username"].as_value();
            let password = evt.values()["password"].as_value();
            let login_model = LoginModel::new(username, password);
            let json = serde_json::to_string(&login_model).unwrap();
            println!("click");

            let resp = Request::post("http://localhost:8080/login")
                .header("Content-Type", "application/json")
                .body(json);

            match resp {
                // Parse data from here, such as storing a response token
                Ok(req) => {
                    req.send().await.unwrap();
                    println!("Login successful!")
                }

                //Handle any errors from the fetch here
                Err(_err) => {
                    println!("Login failed - you need a login server running on localhost:8080.")
                }
            }
        })
    };

    rsx! {

        div{ class: "h-full w-full",

        div {
            class: "min-h-screen flex items-center justify-center bg-gray-100",
            div {
                class: "bg-white bg-opacity-10 backdrop-blur-md shadow-md rounded-xl p-8 w-full max-w-md border border-white border-opacity-30",
                h1 {
                    class: "text-2xl font-bold mb-6 text-center ",
                    "Login"
                }
                form {
                    onsubmit:  move |evt| {
                        // evt.prevent_default(); // <-- if use it rerender what the ?
                        let _ = onsubmit(evt);
                        println!("test");
                    }

                    ,
                    div {
                        class: "mb-4",
                        label {
                            class: "block  mb-2",
                            "Username"
                        }
                        input {
                            class: "w-full px-4 py-2 rounded-lg bg-white bg-opacity-20  placeholder-white focus:outline-none",
                            r#type: "text",
                            id: "username",
                            name: "username",
                            placeholder: "Enter your username"
                        }
                    }
                    div {
                        class: "mb-6",
                        label {
                            class: "block  mb-2",
                            "Password"
                        }
                        input {
                            class: "w-full px-4 py-2 rounded-lg bg-white bg-opacity-20  placeholder-white focus:outline-none",
                            r#type: "password",
                            id: "password",
                            name: "password",
                            placeholder: "Enter your password"
                        }
                    }
                    button {
                        class: "w-full bg-blue-500 hover:bg-blue-600 font-bold py-2 px-4 rounded",
                        r#type: "submit",
                        "Login"
                    }
                }
            }
        }
    }
    }
}
