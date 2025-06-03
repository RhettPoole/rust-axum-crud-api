
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
    model::{QueryOptions, Todo, UpdateTodoSchema, DB},
    response::{SingleTodoResponse, TodoData, TodoListResponse},
};

pub async fn todos_list_handler(
    opts: Option<Query<QueryOptions>>,
    State(db): State<DB>
) -> impl IntoResponse {
    let todos = db.lock().await;

    let Query(opts) = opts.unwrap_or_default();

    letlimit = opts.limit.unwrap_or(10);
    let offset = (opts.page.unwrap_or(1) - 1) * limit;

    let todos: Vec<Todo> = todos.clone().into_iter().skip(offset).take(limit).collect();

    let json_response = TodoListResponse {
        status: "success".to_string(),
        results: todos.len(),
        todos,
    };

    Json(json_response)
}

pub async fn create_todo_handler(
    State(db): State<DB>,
    Json(mut body): Json<Todo>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let mut vec = db.lock().await;

    if let Some(todo) = vec.iter().find(|todo| todo.title == body.title) {
        let error_response = serde_json::json!({
            "status": "fail",
            "message": format!("Todo with title: '{}' already exists", todo.title),
        });
        return Err((StatusCode::CONFLICT, Json(error_response)));
    }

    let uuid_id = Uuid::new_v4();
    let datetime = chrono::Utc::now();

    body.id = Some(uuid_id.to_string());
    body.copleted= Some(false);
    body.createdAt = Some(datetime);
    body.updaatedAt = Some(datetime);

    let todo = body.to_owned();

    vec.push(body);

    let json_response = SingleTodoResponse {
        status: "success".to_string(),
        data: TodoData {todo},
    };

    Ok((StatusCODE::CREATED, Json(json_response)))
}