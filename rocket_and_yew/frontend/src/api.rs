use reqwest::Client;
use reqwest::Error;
use shared::Todo;

pub async fn fetch_todos() -> Result<Vec<Todo>, Error> {
    let client = Client::new();
    let res = client.get("http://127.0.0.1:8080/api/todos").send().await?;
    res.json::<Vec<Todo>>().await
}

pub async fn create_todo(todos: &[Todo], title: &str) -> Result<Todo, Error> {
    let highest_id = todos.iter().map(|it| it.id).max().unwrap_or(0);
    let todo = Todo {
        id: highest_id + 1,
        title: title.to_string(),
        completed: false,
    };
    let client = Client::new();
    let res = client
        .post("http://127.0.0.1:8080/api/todos")
        .json(&todo)
        .send()
        .await?;
    res.json::<Todo>().await
}

pub async fn update_todo(todo: &Todo) -> Result<Todo, Error> {
    let client = Client::new();
    let res = client
        .put(format!("http://127.0.0.1:8080/api/todos/{}", todo.id))
        .json(&todo)
        .send()
        .await?;
    res.json::<Todo>().await
}
