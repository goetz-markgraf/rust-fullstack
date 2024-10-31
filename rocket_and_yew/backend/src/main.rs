#[macro_use]
extern crate rocket;

use controllers::controller::{create_todo, list_todos, update_todo};
use rocket::tokio::sync::Mutex;
use std::fs::{read_to_string, File};
use std::io::{self, BufWriter};
use std::sync::Arc;
use std::vec::Vec;

use shared::Todo;

mod controllers;

type TodoList = Arc<Mutex<Vec<Todo>>>;

const TODO_FILE_PATH: &str = "todos.json";



#[rocket::main]
async fn main() {
    let todos = load_todos_from_file().expect("Failed to load todos");
    let todos = Arc::new(Mutex::new(todos));

    rocket::build()
        .manage(todos)
        .mount("/api", routes![list_todos, create_todo, update_todo])
        .launch()
        .await
        .expect("Failed to launch Rocket server");
}



fn load_todos_from_file() -> io::Result<Vec<Todo>> {
    let contents = match read_to_string(TODO_FILE_PATH) {
        Ok(text) => text,
        Err(_) => return Ok(Vec::new()),
    };
    let todos: Vec<Todo> = serde_json::from_str(&contents)?;
    Ok(todos)
}

pub fn save_todos_to_file(todos: &Vec<Todo>) -> io::Result<()> {
    let file = File::create(TODO_FILE_PATH)?;
    let writer = BufWriter::new(file);
    serde_json::to_writer(writer, todos)?;
    Ok(())
}
