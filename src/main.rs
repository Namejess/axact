use std::sync::{Arc, Mutex};

use axum::{
    extract::State,
    http::Response,
    response::{Html, IntoResponse},
    routing::get,
    Json, Router, Server,
};
use sysinfo::{CpuExt, System, SystemExt};

#[tokio::main]
async fn main() {
    let router = Router::new() // Creating the router. It is a tree of routes. Each route is a function that will be called when the route is matched.
        .route("/", get(root_get)) // Adding a route. The first argument is the path. The second argument is the function that will be called when the route is matched.
        .route("/index.js", get(indexjs_get))
        .route("/api/cpus", get(cpus_get))
        .with_state(AppState {
            // Adding the state to the router. This state will be available to all routes. It is wrapped in an Arc to allow it to be shared between threads.
            sys: Arc::new(Mutex::new(System::new())), // Creating a new System information structure. It is wrapped in a Mutex to allow it to be shared between threads.
        });
    let server = Server::bind(&"0.0.0.0:8081".parse().unwrap()).serve(router.into_make_service()); // Starting the server.
    let addr = server.local_addr(); // Getting the address on which the server is listening.
    println!(" Listing on {addr}"); // Printing the address on which the server is listening.

    server.await.unwrap(); // Waiting for the server to stop.
}

#[derive(Clone)]
struct AppState {
    sys: Arc<Mutex<System>>, // System information structure. It is wrapped in a Mutex to allow it to be shared between threads.
}

#[axum::debug_handler]
async fn root_get() -> impl IntoResponse {
    let markup = tokio::fs::read_to_string("src/index.html").await.unwrap(); // Reading the index.html file.

    Html(markup) // Returning the index.html file.
}

#[axum::debug_handler]
async fn indexjs_get() -> impl IntoResponse {
    let markup = tokio::fs::read_to_string("src/index.js").await.unwrap(); // Reading the index.js file.

    Response::builder() // Creating the response.
        .header("content-type", "application/javascript;charset=utf-8")
        .body(markup)
        .unwrap()
}

#[axum::debug_handler]
async fn cpus_get(State(state): State<AppState>) -> impl IntoResponse {
    let mut sys = state.sys.lock().unwrap(); // Getting the System information structure from the state. It is wrapped in a Mutex to allow it to be shared between threads.

    sys.refresh_cpu(); // Refreshing CPU information.

    let v: Vec<_> = sys.cpus().iter().map(|cpu| cpu.cpu_usage()).collect(); // Getting the CPU usage. It is a value between 0 and 100.

    Json(v)
}
