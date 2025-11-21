use dioxus::prelude::*;

#[derive(serde::Deserialize)]
struct DogApi {
    message: String,
}

#[component]
pub fn DogView() -> Element {
    let mut img_src =
        use_signal(|| "https://images.dog.ceo/breeds/pitbull/dog-3981540_1280.jpg".to_string());

    let skip = move |_| {};
    let save = move |_| async move {
        let response = reqwest::get("https://dog.ceo/api/breeds/image/random")
            .await
            .unwrap()
            .json::<DogApi>()
            .await
            .unwrap();
        img_src.set(response.message)
    };

    rsx! {
        div { id: "dogview",
            img { src: "{img_src}" }
        }
        div { id: "buttons",
            button { onclick: skip, id: "skip", "skip" }
            button { onclick: save, id: "save", "save!" }
        }
    }
}
