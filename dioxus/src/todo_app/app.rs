use dioxus::prelude::*;

use super::server_functions::{add_todo, read_todos, toggle_completed};
use super::types::Todo;



#[component]
pub fn TodoApp() -> Element {
    let mut todo_list = use_resource(|| async move { read_todos().await });
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
                            on_clicked: move |id| {
                                spawn(async move {
                                    let _ = toggle_completed(id).await;
                                    todo_list.restart();
                                });
                            }
                        }
                    }
                    div { class: "mt-4" }
                    input {
                        class: "border border-slate-500 focus:ring mr-4 py-1 px-2 rounded-md",
                        placeholder: "Enter new todo",
                        value: "{new_todo_title}",
                        oninput: move |e| new_todo_title.set(e.value())
                    }
                    button {
                        class: "bg-gradient-to-b from-emerald-300 to-emerald-400 border-emerald-700 border-2 text-black py-1 rounded-md px-4",
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
                None => {
                    rsx!{ p { "Loading todos..." }}
                }
            }
        }
    }
}



#[component]
fn ShowTodos(todo_list: Vec<Todo>, on_clicked: EventHandler<u32>) -> Element {
    rsx! {
        ul {
            for todo in todo_list.iter() {
                SingleTodo {
                    single_todo: todo.clone(),
                    on_clicked: on_clicked
                }
            }
        }
    }
}



#[component]
fn SingleTodo(single_todo: Todo, on_clicked: EventHandler<u32>) -> Element {
    let callback = move |_| on_clicked.call(single_todo.id);

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
