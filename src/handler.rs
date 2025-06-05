/* Imports using the axum webframework;
extract: Allows us to extract data from HTTP requests, such as Path for URL parameters, Query for query strings, and State for shared application state.
http::StatusCode: Used to represent HTTP status codes.
response::IntoResponse: allows handler functions to return different types as HTTP responses.
Json: helper for sending and receiving JSON data. */

use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};

/* Imports Uuid from the uuid crate, used for generating and handling unique identifiers. */
use uuid::Uuid;

/* Imports structures and functions from our local crate (IE our project). */
use crate::{
    model::{CreateTodoSchema, QueryOptions, Todo, UpdateTodoSchema, DB},
    response::{SingleTodoResponse, TodoData, TodoListResponse},
};

/* When user navigates to health checker route, print a message. */
pub async fn health_checker_handler() -> impl IntoResponse {
    const MESSAGE: &str = "Build Simple CRUD API in Rust using Axum";

    let json_response = serde_json::json!({
        "status": "success",
        "message": MESSAGE
    });

    Json(json_response)
}

/* Asynchronous function that handles use trying to get a list of todos. */
pub async fn todos_list_handler(
    opts: Option<Query<QueryOptions>>,
    State(db): State<DB>,
) -> impl IntoResponse {
    let todos = db.lock().await;
        // Accesses and locks databse for reading - The database needs to be 'locked' so that the data isn't being changed from multiple requests running at the same time.
    let Query(opts) = opts.unwrap_or_default();
        // If query parameters are provided, use them. Otherwise use default values.
    let limit = opts.limit.unwrap_or(10);
        // Checks if a limit is provided in the query for how many todo items can be listed per page. Not integral to our current program. Mainly used for organization in front-end pages.
    let offset = (opts.page.unwrap_or(1) - 1) * limit;
        // Calculates where to start in the todo list for pagination, uses the page number from the query or defaults to page 1.
    let todos: Vec<Todo> = todos.clone().into_iter().skip(offset).take(limit).collect();
        // Creates a list of todos for the current page.
        // todos.clone(): Clones the todo list so we can work with it.
        // into_iter(): turns the list into an iterator
        // skip(offset) Skips todos before the current page we are accessing - offset is defined earlier in this function.
        // take(limit): Takes only the number of todos for this page.
        // collect(): Collects the results into a new vector.

    /* Prepares the data to send back as a JSON response. */
    let json_response = TodoListResponse {
        status: "success".to_string(),
        results: todos.len(),
        todos,
    };

    /* Wraps the response in Axum's 'Json' type so it can be sent as a JSON HTTP response. */
    Json(json_response)
}

/* Function handling creating a new todo. */
pub async fn create_todo_handler(
    State(db): State<DB>,
    Json(body): Json<CreateTodoSchema>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let mut vec = db.lock().await;

    /* Checks to see if this todo already exists. */
    if let Some(todo) = vec.iter().find(|todo| todo.title == body.title) {
        let error_response = serde_json::json!({
            "status": "fail",
            "message": format!("Todo with title: '{}' already exists", todo.title),
        });
        return Err((StatusCode::CONFLICT, Json(error_response)));
    }

    /* Generates a unique ID and time stamp for this todo */
    let uuid_id = Uuid::new_v4();
    let datetime = chrono::Utc::now();

    
    /* Create's the todo structure. */
    let todo = Todo {
        id: Some(uuid_id.to_string()),
        title: body.title,
        content: body.content,
        completed: Some(false),
            // Automatically sets to false when you create a new task.
        createdAt: Some(datetime),
        updatedAt: Some(datetime),
    };

    /* Adds the new todo to the database/shared todo list. */
    vec.push(todo.clone());
        // We have to clone it because the todo is used again the response.

    /* Prepares the data to be sent back as a JSON response to the client. */
    let json_response = SingleTodoResponse {
        status: "success".to_string(),
        data: TodoData { todo },
    };

    /* Returns an HTTP 201 Created status and JSON response to the client. */
    Ok((StatusCode::CREATED, Json(json_response)))
}

/* Retrieves the requested todo item. */
pub async fn get_todo_handler(
    Path(id): Path<Uuid>,
    State(db): State<DB>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let id = id.to_string();
    let vec = db.lock().await;

    /* The match keyword lets us compare a values. It's ideal for working with enums where we want a specific outcome for a specific variable. The match below lets us search through a list of todo's for a requested ID which is in the Route. If no todo is found with that ID it will return an error. */
    match vec.iter().find(|todo| todo.id == Some(id.clone())) {
        Some(todo) => {
            let json_response = SingleTodoResponse {
                status: "success".to_string(),
                data: TodoData { todo: todo.clone() },
            };
            Ok((StatusCode::OK, Json(json_response)))
        }
        None => {
            let err_response = serde_json::json!({
                "status": "fail",
                "message": format!("Todo with ID: {} not found", id)
            });
            Err((StatusCode::NOT_FOUND, Json(err_response)))
        }
    }
}

/* Allows us to edit a todo item by ID. */
pub async fn edit_todo_handler(
    Path(id): Path<Uuid>,
    State(db): State<DB>,
    Json(body): Json<UpdateTodoSchema>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let id = id.to_string();
    let mut vec = db.lock().await;

    if let Some(todo) = vec.iter_mut().find(|todo| todo.id == Some(id.clone())) {
        let datetime = chrono::Utc::now();
        let title = body.title.clone().unwrap_or_else(|| todo.title.clone());
        let content = body.content.clone().unwrap_or_else(|| todo.content.clone());
        let completed = body.completed.unwrap_or(todo.completed.unwrap());

        let payload = Todo {
            id: todo.id.to_owned(),
            title: if !title.is_empty() {
                title
            } else {
                todo.title.to_owned()
            },
            content: if !content.is_empty() {
                content
            } else {
                todo.content.to_owned()
            },
            completed: Some(completed),
            createdAt: todo.createdAt,
            updatedAt: Some(datetime),
        };
        *todo = payload;

        let json_response = SingleTodoResponse {
            status: "success".to_string(),
            data: TodoData { todo: todo.clone() },
        };
        Ok((StatusCode::OK, Json(json_response)))
    } else {
        let error_response = serde_json::json!({
            "status": "fail",
            "message": format!("Todo with ID: {} not found", id)
        });
        Err((StatusCode::NOT_FOUND, Json(error_response)))
    }
}

/* Function to delete a todo item by ID. */
pub async fn delete_todo_handler(
    Path(id): Path<Uuid>,
    State(db): State<DB>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let id = id.to_string();
    let mut vec = db.lock().await;

    if let Some(pos) = vec.iter().position(|todo| todo.id == Some(id.clone())) {
        vec.remove(pos);
        return Ok((StatusCode::NO_CONTENT, Json("")));
    }

    let error_response = serde_json::json!({
        "status": "fail",
        "message": format!("Todo with ID: {} not found", id)
    });

    Err((StatusCode::NOT_FOUND, Json(error_response)))
}
