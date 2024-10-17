#[macro_use]
extern crate rocket;

use controllers::todos::{create_todo, delete_todo, list_todos, update_todo};
use lazy_static::lazy_static;
use rocket::tokio::sync::Mutex;
use std::fs::OpenOptions;
use std::io::{self, BufReader, BufWriter};
use std::sync::Arc;
use std::vec::Vec;

use shared::Todo;

mod controllers;

type TodoList = Arc<Mutex<Vec<Todo>>>;

lazy_static! {
    static ref TODO_FILE_PATH: String = "todos.json".to_string();
}

fn load_todos_from_file() -> io::Result<Vec<Todo>> {
    let file = match OpenOptions::new().read(true).open(&*TODO_FILE_PATH) {
        Ok(file) => file,
        Err(_) => return Ok(Vec::new()),
    };
    let reader = BufReader::new(file);
    let todos: Vec<Todo> = serde_json::from_reader(reader)?;
    Ok(todos)
}

pub fn save_todos_to_file(todos: &Vec<Todo>) -> io::Result<()> {
    let file = OpenOptions::new()
        .write(true)
        .truncate(true)
        .create(true)
        .open(&*TODO_FILE_PATH)?;
    let writer = BufWriter::new(file);
    serde_json::to_writer(writer, todos)?;
    Ok(())
}

#[rocket::main]
async fn main() {
    let todos = load_todos_from_file().expect("Failed to load todos");
    let todos = Arc::new(Mutex::new(todos));

    rocket::build()
        .manage(todos)
        .mount(
            "/api",
            routes![list_todos, create_todo, update_todo, delete_todo],
        )
        .launch()
        .await
        .expect("Failed to launch Rocket server");
}
