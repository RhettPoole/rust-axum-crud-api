/* **Summary:**  
This file sets up a simple web server using Rust and the Axum framework. It listens on port 8000 and responds to GET requests at `/api/healthcheecker` with a JSON message. The code uses async/await for non-blocking operations and prints a message when the server starts. */

/* This file was used to just create a basic server with the minimum functionality of starting a server using AXUM. */

/* This line imports tools from 'axum' web framework for Rust. */
use axum:: {response:: IntoResponse, routing::get, Json, Router};
    // 'IntoResponse': Lets us return different types from our handler functions.
    // 'get': Used to define HTTP GET Routes.
    // 'Json': Lets us easily return JSON responses.
    // 'Router': Used to set up our API routes.

/* Defines an asnych function (meaning it can run without blocking other code) call 'health_checker'. */
async fn health_checker_handler() -> impl IntoResponse {
    // Returns something that can be turned into an HTTP response. '->' specifies return type.

    /* Defines a constant string called "MESSAGE" */
     const MESSAGE: &str = "Build Simply CRUD API in Rust using Axum";
        // '&str' is using a string slice, represents a view into a string. Uses string data.

    /* Creates a JSON object with two fields 'status' and 'message'. Uses 'serde_json' crate to build this JSON. */
    let json_response = serde_json::json!({
        // Every Rust project is 'crate'. serde_json uses 'cargo', which is built into Rust, to compile and run our code
        // json! uses what's called a 'macro' to write code that generates other code. Here we are generating JSON. View a comment underneath the function to see what exactly serde_json is doing.
        "status": "success",
        "message": MESSAGE
    });

    /*-> serde_json compiling example
    let data = serde_json::json!({
    "name": "Alice",
    "age": 30,
    "is_member": true
    }); */
    // This creates a json object like this: {"name":"Alice","age":30,"is_member":true}
    

    /* Wraps the JSON object we just created in an Axum 'Json' type, to it can be as a JSON HTTP response to the browser or console. */
    Json(json_response)
} // In Rust, the 'return' keyword is implied on the last line of the function. It's returned automatically. 'return' can still be used earlier in the function, if we want to return something specific.

/* Attribute that tells Rust to use the Tokio runtime for asynchronous code. Basically makes the 'main' function able to use 'async'. */
#[tokio::main]

/* Defines the main entory point of the program as an asynchronous function. */
async fn main() {
    /* Creates a new router (like a web server's route table). */
    let app = Router::new().route("/api/healthchecker", get(health_checker_handler));
        // This also adds a route: when someone send a GET response to '/api/healthchecker', it will call 'health_checker_handler'.

    /* Prints a message to the console that the server is up and running. */
    println!("Server started successfully, listening at http://localhost:8000/api/healthchecker/");

    /* Starts listening for network connections on all IP addresses ('0.0.0.0') at port '8000'. */
    let listener = tokio::net::TcpListener::bind("0.0.0.0:8000").await.unwrap();
        // 'await' means it waits for this function to finish before moving to the next item.
        // 'unwrap' means if there is an error, crach the program. - Good for dev phase.

    /* Starts the web server using the listener and router above. */
    axum::serve(listener, app).await.unwrap();
}