use dioxus::prelude::*;
#[component]
pub fn Bg(message_container: Element, left_config: Element, right_config: Element) -> Element {
    rsx! {

        div{class:"flex content-height",
            div{class:"w-1/5 bg-blue-500 justify-items-center p-4 overflow-y-auto content-full scrollbar-modern",
               {left_config}
            }
            div{class:"w-3/5 bg-blue-500 p-4 mr-4 overflow-y-auto content-full scrollbar-modern",
                id: "message-container-scroll",

               {message_container}
            }
            div{class:"w-1/5 bg-blue-500 p-4 content-full ",
               {right_config}
            }
        }

    }
}
