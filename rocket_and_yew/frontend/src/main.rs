mod api;
mod todo_view;

use todo_view::TodoView;
use yew::prelude::*;

fn main() {
    yew::Renderer::<App>::new().render();
}

#[function_component(App)]
fn app() -> Html {
    html! {
       <div class="todo-container">
           <h1>{"Todo App"}</h1>
           <TodoView />
       </div>
    }
}
