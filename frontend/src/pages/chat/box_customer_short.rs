use dioxus::prelude::*;

#[component]
pub fn BoxCustomerShort() -> Element {
    rsx! {

            for _ in 0..20{
                div{ class: "h-20 w-60 bg-orange-500 rounded-lg m-2 p-4"
                ,"test"
                }
            }

    }
}
