use dioxus::prelude::*;

#[derive(Clone)]
#[allow(dead_code)]
pub struct TitleState(pub String);

#[component]
pub fn Title() -> Element {
    let title = use_context::<TitleState>();

    rsx! {
        div { id: "title",
            h1 { "{title.0}! 🌭" }
        }
    }
}
