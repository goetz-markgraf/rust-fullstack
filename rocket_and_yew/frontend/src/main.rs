mod api;
mod todo_view;

use todo_view::TodoView;
use yew::prelude::*;

#[function_component(App)]
fn app() -> Html {
    html! {
       <div class="todo-container">
           <h1>{"Todo App"}</h1>
           <TodoView />
       </div>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
