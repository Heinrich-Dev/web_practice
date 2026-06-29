// Henry Boekhoff
// Little TCP Rust Server for learning
// Comments are for future Henry

// HTTP routing and requests library
// routing::get routes get requests
// to a specified handler.
// Json is for JSON stuff, go figure
// Router is the type for routing
use axum::{routing::get, Json, Router};

// serialization of data structures
// used for JSON in this instance
use serde::Serialize;

// I have now learned that Cross-Origin
// Resource Sharing is a thing and this
// crate allows you to do it.
use tower_http::cors::{Any, CorsLayer};

#[derive(Serialize)]
struct MessageResponse {
    message: String,
}

// tokio lets me define a main function
// as asynchronous with this macro.
#[tokio::main]
async fn main() {
    // Need cors for letting Vue running on
    // a different port access data sent from this server
    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any);

    // Defining an API route using Router
    let app = Router::new()
        .route("/api/home", get(get_home))
        .layer(cors);

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000").await.unwrap();
    println!("Server listening...");

    axum::serve(listener, app).await.unwrap();
}

async fn get_home() -> Json<MessageResponse> {
    let response = MessageResponse {
        message: String::from("No way."),
    };
    Json(response)
}
