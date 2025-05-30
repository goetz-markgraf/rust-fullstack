use dioxus::prelude::*;

use super::server_functions::{add_todo, read_todos, toggle_completed};
use super::types::Todo;



#[component]
pub fn TodoApp() -> Element {
    let mut todo_list = use_resource(async move || read_todos().await);
    let mut new_todo_title = use_signal(|| "".to_string());

    rsx! {
        div {
            h1 { class: "h1", "Todo-App" }
            match &*todo_list.read() {
                Some(Ok(todos)) => rsx! {
                    div {
                        class: "px-5 py-3 border shadow-lg rounded-lg",
                        ShowTodos {
                            todo_list: todos.clone(),
                            on_finished_task: move |id| async move {
                                let _ = toggle_completed(id).await;
                                todo_list.restart();
                            }
                        }
                    }
                    input {
                        class: "mt-4 border border-slate-500 focus:ring-3 mr-4 py-1 px-2 rounded-md",
                        placeholder: "Enter new todo",
                        value: "{new_todo_title}",
                        oninput: move |e| new_todo_title.set(e.value())
                    }
                    button {
                        class: "bg-linear-to-b from-emerald-300 to-emerald-400 border-emerald-700 border-2 text-black py-1 rounded-md px-4",
                        onclick: move |_| async move {
                            if add_todo(new_todo_title()).await.is_ok() {
                                todo_list.restart();
                            }
                        },
                        "Add Todo"
                    }
                },
                Some(Err(error)) => rsx! {
                    p { class:"text-red-800", "Error occurred: {error}" }
                },
                None => rsx!{
                    p { "Loading todos..." }
                }
            }
        }
    }
}



#[component]
fn ShowTodos(todo_list: Vec<Todo>, on_finished_task: EventHandler<u32>) -> Element {
    rsx! {
        ul {
            for todo in todo_list.iter() {
                SingleTodo {
                    single_todo: todo.clone(),
                    on_finished_task: on_finished_task
                }
            }
        }
    }
}



#[component]
fn SingleTodo(single_todo: Todo, on_finished_task: EventHandler<u32>) -> Element {
    let callback = move |_| on_finished_task.call(single_todo.id);

    rsx! {
        li {
            key: "{single_todo.id}",
            class: if single_todo.completed { "completed" } else { "" },
            input {
                class: "mr-2",
                r#type: "checkbox",
                checked: single_todo.completed,
                onclick: callback
            }
            span {
                class: "cursor-default",
                onclick: callback,
                "{single_todo.text}",
            }
        }
    }
}
