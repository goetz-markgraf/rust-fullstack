#![allow(non_snake_case, unused)]

use dioxus::prelude::*;
use dioxus_logger::tracing;

mod not_found;
mod todo_app;

use not_found::NotFound;
use todo_app::app::TodoApp;



#[derive(Clone, Routable, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
enum Route {
    #[route("/")]
    TodoApp {},
    #[route("/:..segments")]
    NotFound { segments: Vec<String> },
}



fn App() -> Element {
    rsx! {
        div {
            class: "container md:w-auto mx-auto p-4",
            Router::<Route> {}
        }
    }
}



fn main() {
    // Init logger
    dioxus_logger::init(tracing::Level::INFO).expect("failed to init logger");
    tracing::info!("starting app");
    launch(App);
}
