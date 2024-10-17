use dioxus::prelude::*;

use super::Route;

#[component]
pub fn NotFound(segments: Vec<String>) -> Element {
    rsx! {
        div {
            h1 { "404 Not Found" }
            p { "The page you are looking for does not exist." }
            Link { to: Route::TodoApp {}, "To Main Page" }
        }
    }
}
