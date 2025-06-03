/* **Summary:**  
This file defines the data structures (models) for your todo app, including the todo item itself, how to store them safely for concurrent access, and helper structs for updating and querying todos. It uses Rust’s type system and some popular libraries to make data handling safe and easy. */

/* Imports date and time utitilites from the 'chrono' crate (Rust's popular date/time library). */
use chrono::prelude::*;
/* Imports traits for converting Rust data to/from JSON or other formats. */
use serde::{Deserialize, Serialize};
/* Imports `Arc` (a thread-safe reference-counted pointer, lets you share data safely across threads) and `Mutex` (a lock to safely allow only one thread to access data at a time, but this one is async-friendly from `tokio`). */
use std::sync::Arc;
use tokio::sync::Mutex;

/* Allows us to write 'non-snakecase' field names. */
#[allow(non_snake_case)]
/* Automatically gives our structure some useful abilities.
    'Debug': Print it for debugging
    Deserialize, Serialize: Convert to/from JSON'
    'Clone': Make copies of it. */
#[derive(Debug, Deserialize, Serialize, Clone)]
/* Defines a public structure (like a class or a record) named 'Todo' */
pub struct Todo {
    pub id: Option<String>, // Option: could be missing, not needed.
    pub title: String, // Required string
    pub content: String, // Required string
    pub completed: Option<bool>, // Optional true/false
    pub createdAt: Option<DateTime<Utc>>, // Option date/time
    pub updatedAt: Option<DateTime<Utc>>, // Option date/time
}

/* Defines a type alias 'DB' for a thread-safe, shareable, lockable list of 'Todo' items. */
pub type DB = Arc<Mutex<Vec<Todo>>>;

/* Defines a function that creates and returns a new, empty, thread-safe todo list. */
pub fn todo_db() -> DB {
    Arc::new(Mutex::new(Vec::new()))
}

/* Same as above but adds 'default': allows the struct to be created with default values. */
#[derive(Debug, Deserialize, Default)]
pub struct QueryOptions {
    pub page: Option<usize>,
    pub limit: Option<usize>,
}

#[allow(non_snake_case)]
#[derive(Debug, Deserialize, Clone)]
/* Defines a struct for updated a todo item, note that each field is option, so you can update the todo only what you want and everything else will stay the same. */
pub struct UpdateTodoSchema {
    pub title: Option<String>,
    pub content: Option<String>,
    pub completed: Option<bool>,
}