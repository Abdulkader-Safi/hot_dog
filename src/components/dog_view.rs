use dioxus::prelude::*;

// Import our server function
use crate::apis::save_dog;

#[derive(serde::Deserialize)]
struct DogApi {
    message: String,
}

#[component]
pub fn DogView() -> Element {
    let mut img_src =
        use_signal(|| "https://images.dog.ceo/breeds/pitbull/dog-3981540_1280.jpg".to_string());

    // Skip button: load a new random dog image
    let skip = move |_| async move {
        let response = reqwest::get("https://dog.ceo/api/breeds/image/random")
            .await
            .unwrap()
            .json::<DogApi>()
            .await
            .unwrap();
        img_src.set(response.message);
    };

    // Save button: save current dog to backend, then load a new one
    let save = move |_| async move {
        // Get the current image URL
        let current_image = img_src.read().clone();

        // Call the server function to save the dog
        match save_dog(current_image).await {
            Ok(_) => {
                println!("Dog saved successfully!");

                // After saving, load a new dog image
                let response = reqwest::get("https://dog.ceo/api/breeds/image/random")
                    .await
                    .unwrap()
                    .json::<DogApi>()
                    .await
                    .unwrap();
                img_src.set(response.message);
            }
            Err(e) => {
                eprintln!("Failed to save dog: {:?}", e);
            }
        }
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
