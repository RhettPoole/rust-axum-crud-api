/* This file is used to start our server and register all necessary routes for our todo API. This implements all logic from our handler/model/response modules. This is the entry piont for our API. */

/* These lines tell Rust to include code from other files (modules) named `handler.rs`, `model.rs`, `response.rs`, and `route.rs`. This keeps your code organized by separating different responsibilities (like handling requests, defining data, formatting responses, and setting up routes). */
mod handler;
mod model;
mod response;
mod route;

/* Imports types and constants from the Axum web framework */
use axum::http::{
    header::{ACCEPT, AUTHORIZATION, CONTENT_TYPE},
    HeaderValue, Method,
        // header: HTTP header names used for controlling what kind of requests our server will accept
        // HeaderValue: Represents the value of an HTTP header
        // Method: Represents HTTP methods like GET, POST, PATCH, DELETE.
};
use route::create_router;
    // Imports the create_router function from our 'route' module. Sets up all routes and URLS (and what they do) for our API.
use tower_http::cors::CorsLayer;
    // Imports the 'CorsLayer' type, controls what websites are allowed to talk to our API (not important now, but would be if we had frontend AND a backend.)

/* Tells Rust to use the Tokio runtime for running async functions, allows our server to handle many requests at once without blocking. When we're testing with current functionality there will only be one test at once, because there is only one user at a time (you).*/
#[tokio::main]
async fn main() {
    /* Creates a cors policy, This is crucial for security and for letting your frontend (running on a different port once you develop it) talk to your backend. */
    let cors = CorsLayer::new()
        .allow_origin("http://localhost:3000".parse::<HeaderValue>().unwrap())
            // Only allows requests from a specific port, our frontend app.
        .allow_methods([Method::GET, Method::POST, Method::PATCH, Method::DELETE])
            // Only allows certain HTTP methods. GET POST PATCH DELETE, which are tested in Postman.
        .allow_credentials(true)
            // Allows cookies or authentication info to be sent. - Not imporant at this phase of our API.
        .allow_headers([AUTHORIZATION, ACCEPT, CONTENT_TYPE]);
            // Only allows certain headers in our requests.

    /* Creates our main application by calling: */
    let app = create_router().layer(cors);
        // create_router(): sets up all of our API routes.
        // Adds a CORS policy as a "layer" to control who can access our API.

    println!("🚀 Server started successfully");
        // Message letting us know that the server has started.
    let listener = tokio::net::TcpListener::bind("0.0.0.0:8000").await.unwrap();
        // Sets up our server to listen for incoming connections on port 8000 on all of our network interfaces ('0.0.0.0')
        // This is how our server is able to receive requests from Postman or other browsers instead of just at a frontend which isn't built yet.
    axum::serve(listener, app).await.unwrap();
        // Starts the Axum server, using the listener (network socket) and our app (routes + CORS). The server now handles incoming requests.
        // This is the line that aactually allows our API to be available to clients.
}