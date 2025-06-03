/***Summary:**  
This file defines the shapes of the JSON responses your API will send back. Each struct represents a different kind of response: a generic message, a single todo, or a list of todos. The `Serialize` trait makes it easy to turn these structs into JSON for your users. */

/* Imports the 'Todo' struct from our model.rs file so we can use it here. */
use crate::model::Todo;
/* Imports the 'Serialize' trait. Which lets us turn Rust data into JSON. */
use serde::Serialize;

#[derive(Serialize)]
pub struct GenericResponse {
    pub status: String,
    pub message: String,
}

#[derive(Serialize, Debug)]
pub struct TodoData {
    pub todo: Todo,
}

#[derive(Serialize, Debug)]
pub struct SingleTodoResponse {
    pub status: String,
    pub data: TodoData,
}

#[derive(Serialize, Debug)]
pub struct TodoListResponse {
    pub status: String,
    pub results: usize,
    pub todos: Vec<Todo>,
}