use leptos::{server, ServerFnError};

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Todo {
    pub id: usize,
    pub description: String,
    pub completed: bool,
}



#[server(GetTodos, "/api", "GetJson")]
pub async fn get_todos() -> Result<Vec<Todo>, ServerFnError> {
    let todos = read_todos_from_file()?;

    Ok(todos)
}

#[server]
pub async fn add_todo(todo_title: String) -> Result<Todo, ServerFnError> {
    let mut todos: Vec<Todo> = read_todos_from_file()?;

    let max_id = todos.iter().map(|it| it.id).max().unwrap_or(0);
    let todo = Todo {
        id: max_id + 1,
        completed: false,
        description: todo_title,
    };

    // append the new todo to the list of todos
    todos.push(todo.clone());

    write_todos_to_file(&todos)?;

    Ok(todo)
}

#[server]
pub async fn update_todo(id: usize, completed: bool) -> Result<Todo, ServerFnError> {
    let mut todos: Vec<Todo> = read_todos_from_file()?;
    let mut changed: Option<&Todo> = None;

    // update the todo at the given index
    for todo in todos.iter_mut() {
        if todo.id == id {
            todo.completed = completed;
            changed = Some(todo);
        }
    }

    match changed {
        None => Err(ServerFnError::ServerError("Todo not found".to_string())),
        Some(todo) => {
            let ret = Ok(todo.clone());
            write_todos_to_file(&todos)?;
            ret
        }
    }

    // match todos.iter().find(|it| it.id == id) {
    //     None => Err(ServerFnError::ServerError("Todo not found".to_string())),
    //     Some(&mut todo) => {
    //         todo.completed = completed;
    //         write_todos_to_file(&todos)?;
    //         Ok(todo.clone())
    //     }
    // }
}


pub fn read_todos_from_file() -> Result<Vec<Todo>, ServerFnError> {
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



pub fn write_todos_to_file(todos: &[Todo]) -> Result<(), ServerFnError> {
    use std::fs::File;
    use std::io::prelude::*;

    let json = serde_json::to_string(todos)?;

    let mut writer = File::create("todos.json")?;
    writer.write_all(json.as_bytes())?;

    Ok(())
}
