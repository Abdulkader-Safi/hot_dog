use crate::apis::get_saved_dogs;
use dioxus::prelude::*;

#[component]
pub fn Favorites() -> Element {
    // Create a pending resource that resolves to the list of dogs from the backend
    // Wait for the favorites list to resolve with `.suspend()`
    let favorites = use_resource(get_saved_dogs);

    match &*favorites.read_unchecked() {
        Some(Ok(dogs)) => rsx! {
            div { id: "favorites",
                div { id: "favorites-container",
                    if dogs.is_empty() {
                        p { "No favorite dogs saved yet! Go save some from the home page." }
                    } else {
                        for dog in dogs {
                            div { key: "{dog.id}", class: "favorite-dog",
                                img { src: "{dog.url}" }
                            }
                        }
                    }
                }
            }
        },
        Some(Err(e)) => rsx! {
            div { "Error loading favorites: {e}" }
        },
        None => rsx! {
            div { "Loading favorites..." }
        },
    }
}
