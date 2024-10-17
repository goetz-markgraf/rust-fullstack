use gloo::console::log;
use shared::Todo;
use wasm_bindgen_futures::spawn_local;
use web_sys::wasm_bindgen::JsCast;
use web_sys::wasm_bindgen::UnwrapThrowExt;
use web_sys::HtmlInputElement;
use yew::prelude::*;

use crate::api::{create_todo, fetch_todos, update_todo};

#[function_component(TodoView)]
pub fn todo() -> Html {
    let todos = use_state(Vec::new);
    let new_todo_title = use_state(String::new);

    {
        let todos = todos.clone();
        use_effect_with((), move |_| {
            spawn_local(async move {
                match fetch_todos().await {
                    Ok(t) => todos.set(t),
                    Err(e) => log!(format!("Error fetching todos: {:?}", e)),
                }
            });
        });
    }

    let oninput = {
        let new_todo_title = new_todo_title.clone();
        Callback::from(move |e: InputEvent| {
            let e: Event = e.dyn_into().unwrap_throw();
            let target = e.target().unwrap_throw();
            let target: HtmlInputElement = target.dyn_into().unwrap_throw();
            new_todo_title.set(target.value())
        })
    };

    let onkeypress = {
        let new_todo_title = new_todo_title.clone();
        let todos = todos.clone();
        Callback::from(move |e: KeyboardEvent| {
            if e.key() == "Enter" {
                let todos = todos.clone();
                let new_todo_title = new_todo_title.clone();
                spawn_local(async move {
                    create_and_add_todo(todos, new_todo_title).await;
                });
            }
        })
    };

    let onclick = {
        let new_todo_title = new_todo_title.clone();
        let todos = todos.clone();
        Callback::from(move |_| {
            let todos = todos.clone();
            let new_todo_title = new_todo_title.clone();
            spawn_local(async move {
                create_and_add_todo(todos, new_todo_title).await;
            });
        })
    };

    html! {
        <div>
            <input
                type="text"
                value={(*new_todo_title).clone()}
                {oninput}
                {onkeypress}
                placeholder="What needs to be done?"
            />
            <br />
            <button {onclick}>{"Add Todo"}</button>
            <ul>
                {for (*todos).iter().map(|it| render_todo(&todos, it))}
            </ul>
        </div>
    }
}

fn render_todo(todos: &UseStateHandle<Vec<Todo>>, todo: &Todo) -> Html {
    let completed = if todo.completed {
        Some("todo-completed")
    } else {
        None
    };

    let onchange = {
        let todos = todos.clone();
        let todo = todo.clone();
        Callback::from(move |_| {
            let todos = todos.clone();
            let mut todo = todo.clone();
            spawn_local(async move {
                toggle_todo_completion_state(todos, &mut todo).await;
            });
        })
    };

    html! {
        <li class={classes!(completed)}>
            <input
                type="checkbox"
                checked={todo.completed}
                { onchange }
            />
            {" "}
            { &todo.title }
        </li>
    }
}

async fn create_and_add_todo(
    todos: UseStateHandle<Vec<Todo>>,
    new_todo_title: UseStateHandle<String>,
) {
    if let Ok(todo) = create_todo(&todos, &new_todo_title.to_string()).await {
        let mut new_vec = (*todos).clone();
        new_vec.push(todo);
        todos.set(new_vec);
        new_todo_title.set(String::new());
    }
}

async fn toggle_todo_completion_state(todos: UseStateHandle<Vec<Todo>>, todo: &mut Todo) {
    todo.completed = !todo.completed;

    if update_todo(todo).await.is_ok() {
        let new_todos = todos
            .iter()
            .map(|it| {
                if it.id == todo.id {
                    todo.clone()
                } else {
                    it.clone()
                }
            })
            .collect();
        todos.set(new_todos);
    }
}
