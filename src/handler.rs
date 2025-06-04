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

/* A function that uses Axum webframework to handle requests for listing todo items. */
pub async fn todos_list_handler(
    opts: Option<Query<QueryOptions>>, 
        // Query<QueryOptions> means it’s information from the web address, like page number or how many items to show.
    State(db): State<DB>
        // State(db): A way to get shared data (the todo list) that the whole program can use.
        // DB is the type of this shared data.
) -> impl IntoResponse {
        // impl: implement
        // IntoResponse: The response that will be sent as a reply to a web request.

    /* Wait to use database until it's safe. */
    let todos = db.lock().await;

    /* If there are options in the web address, use them, if not, use default values. */
    let Query(opts) = opts.unwrap_or_default();

    /* Default number to show per page is 10. */
    let limit = opts.limit.unwrap_or(10);

    /* Figure out where to start in the list, based on the page number and limit. */
    let offset = (opts.page.unwrap_or(1) - 1) * limit;

    /* Make a new list of todos, only including the ones for this page. */
    let todos: Vec<Todo> = todos.clone().into_iter().skip(offset).take(limit).collect();

    /* Create a response with the status, number of todos, and the list itself */
    let json_response = TodoListResponse {
        status: "success".to_string(),
        results: todos.len(),
        todos,
    };

    /* Turn the response into JSON. */
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

    /* Create a brand new random unique ID and store it in a variable uuid_id */
    let uuid_id = Uuid::new_v4();
    /* Get the current date and time in UTC. */
    let datetime = chrono::Utc::now();

    /* Body is one of our parameters for this function. Need to initialize the fields of the 'body' parameter before the Todo item can be created. */
    body.id = Some(uuid_id.to_string());
    body.completed= Some(false);
    body.createdAt = Some(datetime);
    body.updatedAt = Some(datetime);

    /* Creates an owned copy of the Todo item after initializing its fields. */
    let todo = body.to_owned();

    /* Pushes the items that we just initialized above to 'body' parameter. */
    vec.push(body);

    let json_response = SingleTodoResponse {
        status: "success".to_string(),
        data: TodoData {todo},
    };

    Ok((StatusCODE::CREATED, Json(json_response)))
}

pub async fn get_todo_handler(
    Path(id): Path<Uuid>,
    State(db): State<DB>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let id = id.to_string();
    let vec = db.lock().await;

    if let Some(todo) = vec.iter().find(|todo| todo.id == Some(id.to_owned())) {
        let json_response = SingleTodoResponse {
            status: "success".to_string(),
            data: TodoData { todo: todo.clone() },
        };
        return Ok((StatusCode::Ok, Json(json_response)));
    }

    let err_response = serde_json::json!({
        "status": "fail",
        "message": format!("Todo with ID: {} not found", id)
    });

    Err((StatusCode::NOT_FOUND, Json(error_response)))
}