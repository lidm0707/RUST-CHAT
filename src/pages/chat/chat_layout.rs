use dioxus::prelude::*;
#[component]
pub fn Bg(message_container: Element, right_config: Element) -> Element {
    rsx! {
        div{class:"flex content-height",
            div{class:"w-3/5 bg-blue-500 p-4 mr-4 overflow-y-auto content-full",
                id: "message-container-scroll",

               {message_container}
            }
            div{class:"w-1/5 bg-blue-500 p-4 content-full ",
               {right_config}
            }
        }

    }
}
