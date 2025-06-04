use axum::{
    routing::{get, post},
    Router,
};

use crate::{
    handler::{
        create_todo_handler, delete_todo_handler, edit_todo_handler, get_todo_handler,
        health_checker_handler, todos_list_handler,
    },
    model,
};

pub fn create_router() -> Router {
    let db = model::todo_db();

    Router::new()
        .route("/api/healthchecker", get(health_checker_handler))
        .route(
            "/api/todos",
            get(todos_list_handler) // List all todos
                .post(create_todo_handler), // Create a new todo
        )
        .route(
            "/api/todos/:id",
            get(get_todo_handler) // Get a single todo by ID
                .patch(edit_todo_handler) // Edit a todo by ID
                .delete(delete_todo_handler), // Delete a todo by ID
        )
        .with_state(db)
}
