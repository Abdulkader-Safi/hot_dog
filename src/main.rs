mod apis;
mod components;

use components::{App, DogView, Favorites, NavBar};
use dioxus::prelude::*;

#[derive(Routable, Clone, PartialEq)]
#[rustfmt::skip]
enum Route {
    #[layout(NavBar)]
    #[route("/")]
    DogView,
    #[route("/favorites")]
    Favorites,
}

fn main() {
    // On client targets (web, desktop, mobile), launch normally
    #[cfg(not(feature = "server"))]
    dioxus::launch(App);

    // On server target, use dioxus::serve to configure the Axum router
    #[cfg(feature = "server")]
    dioxus::serve(|| async move {
        // Create a new axum router for our Dioxus app
        let router = dioxus::server::router(App);

        // The router automatically includes all server functions
        // defined with #[post] or #[get] macros

        // You can customize the router here if needed:
        // let router = router.layer(/* custom middleware */);

        // Return the configured router
        Ok(router)
    })
}
