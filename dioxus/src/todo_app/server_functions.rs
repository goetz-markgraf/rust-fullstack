use dioxus::prelude::*;

use super::types::Todo;

#[server(PostServerData)]
pub async fn add_todo(text: String) -> Result<Todo, ServerFnError> {
    let mut todos = read_todos_from_file()?;
    let id = todos.iter().map(|todo| todo.id).max().unwrap_or(0) + 1;
    let todo = Todo {
        id,
        text,
        completed: false,
    };

    todos.push(todo.clone());
    write_todos_to_file(&todos)?;
    Ok(todo)
}

#[server(GetServerData)]
pub async fn read_todos() -> Result<Vec<Todo>, ServerFnError> {
    read_todos_from_file()
}

#[server(PutServerData)]
pub async fn toggle_completed(id: u32) -> Result<(), ServerFnError> {
    let mut todos = read_todos_from_file()?;
    let todo = todos
        .iter_mut()
        .find(|todo| todo.id == id)
        .ok_or(ServerFnError::new("Not Found"))?;
    todo.completed = !todo.completed;

    write_todos_to_file(&todos)?;
    Ok(())
}

// Helper functions for storing and retrieving todos

fn read_todos_from_file() -> Result<Vec<Todo>, ServerFnError> {
    use std::fs::File;
    use std::io::prelude::*;

    let todos = match File::open("todos.json") {
        Ok(mut file) => {
            let mut buf = String::new();
            file.read_to_string(&mut buf)?;

            serde_json::from_str(&buf)?
        }
        Err(_) => Vec::new(),
    };

    Ok(todos)
}

fn write_todos_to_file(todos: &[Todo]) -> Result<(), ServerFnError> {
    use std::fs::File;
    use std::io::prelude::*;

    let json = serde_json::to_string(todos)?;

    let mut writer = File::create("todos.json")?;
    writer.write_all(json.as_bytes())?;

    Ok(())
}
