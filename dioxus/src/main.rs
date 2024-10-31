#![allow(non_snake_case, unused)]

use dioxus::prelude::*;

mod not_found;
mod todo_app;

fn main() {
    launch(App);
}



fn App() -> Element {
    rsx! {
        div {
            class: "container md:w-auto mx-auto p-4",
            Router::<Route> {}
        }
    }
}



use not_found::NotFound;
use todo_app::app::TodoApp;

#[derive(Clone, Routable, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
enum Route {
    #[route("/")]
    TodoApp {},
    #[route("/:..segments")]
    NotFound { segments: Vec<String> },
}
