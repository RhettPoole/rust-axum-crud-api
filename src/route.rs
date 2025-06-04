/* This file is used to organize all routes/urls for us to use in our website. The path's can be used as a reference when writing tests in Postman. */

/* Imports necessary portions of the Axum framework. */
use axum::{
    routing::{get, post},
    Router,
};

/* Imports our functions builtin the handler.rs file for use in this file. */
use crate::{
    handler::{
        create_todo_handler, delete_todo_handler, edit_todo_handler, get_todo_handler,
        health_checker_handler, todos_list_handler,
    },
    model,
};

/* A main router function which is called in the main.rs file to set up our routing. */
pub fn create_router() -> Router {
    let db = model::todo_db();
        // Calls a function in our model.rs file to create/access our todo database/in-memory store.
        // This 'db' is shared with our handler functions so they can read and write todos in the same place.

    /* Creates a new empty router which we can add our API routes into. */
    Router::new()
        .route("/api/healthchecker", get(health_checker_handler))
            // Adds a route for healthchecker, then calls/ties our function to it. This is the base "check that server is running" route.
        .route(
            "/api/todos",
            get(todos_list_handler) // List all todos
                .post(create_todo_handler), // Create a new todo
                    // If the request is a GET, it lists all todos, if it's a POST, it creates a new todo.
        )
        .route(
            "/api/todos/:id",
            get(get_todo_handler) // Get a single todo by ID
                .patch(edit_todo_handler) // Edit a todo by ID
                .delete(delete_todo_handler), // Delete a todo by ID
        )
        .with_state(db)
            // Attaches our 'db' to the router so that all handler function can access and modify the todo list.
}
