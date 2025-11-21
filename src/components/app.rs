use super::dog_view::DogView;
use super::title::{Title, TitleState};
use dioxus::prelude::*;

const MAIN_CSS: Asset = asset!("/assets/main.css");

#[component]
pub fn App() -> Element {
    use_context_provider(|| TitleState("HotDog".to_string()));

    rsx! {
        document::Stylesheet { href: MAIN_CSS }
        Title {}
        DogView {}
    }
}
