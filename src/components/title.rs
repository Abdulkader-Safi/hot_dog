use dioxus::prelude::*;

#[derive(Clone)]
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
