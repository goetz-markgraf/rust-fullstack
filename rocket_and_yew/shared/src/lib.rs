use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct Todo {
    pub id: usize,
    pub title: String,
    pub completed: bool,
}
