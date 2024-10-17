use rocket::{http::Status, response::status::Custom, serde::json::Json};
use shared::Todo;

use crate::{save_todos_to_file, TodoList};

#[get("/todos")]
pub async fn list_todos(todos: &rocket::State<TodoList>) -> Json<Vec<Todo>> {
    let todos = todos.lock().await;
    Json(todos.clone())
}

#[post("/todos", data = "<todo>")]
pub async fn create_todo(
    todos: &rocket::State<TodoList>,
    todo: Json<Todo>,
) -> Result<Json<Todo>, Custom<String>> {
    let mut todos = todos.lock().await;
    todos.push(todo.0.clone());
    save_todos_to_file(&todos).map_err(|_| {
        Custom(
            Status::InternalServerError,
            "Failed to save todos".to_string(),
        )
    })?;
    Ok(Json(todo.0.clone()))
}

#[put("/todos/<id>", data = "<todo>")]
pub async fn update_todo(
    todos: &rocket::State<TodoList>,
    id: usize,
    todo: Json<Todo>,
) -> Result<Option<Json<Todo>>, Custom<String>> {
    let mut todos = todos.lock().await;
    if let Some(existing_todo) = todos.iter_mut().find(|t| t.id == id) {
        *existing_todo = todo.0.clone();
        save_todos_to_file(&todos).map_err(|_| {
            Custom(
                Status::InternalServerError,
                "Failed to save todos".to_string(),
            )
        })?;
        Ok(Some(Json(todo.0.clone())))
    } else {
        Ok(None)
    }
}

#[delete("/todos/<id>")]
pub async fn delete_todo(
    todos: &rocket::State<TodoList>,
    id: usize,
) -> Result<Option<Json<bool>>, Custom<String>> {
    let mut todos = todos.lock().await;
    if todos.iter().any(|t| t.id == id) {
        todos.retain(|t| t.id != id);
        save_todos_to_file(&todos).map_err(|_| {
            Custom(
                Status::InternalServerError,
                "Failed to save todos".to_string(),
            )
        })?;
        Ok(Some(Json(true)))
    } else {
        Ok(None)
    }
}
