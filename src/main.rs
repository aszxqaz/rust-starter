// This starter uses the `axum` crate to create an asyncrohnous web server
// The async runtime being used, is `tokio`
// This starter also has logging, powered by `tracing` and `tracing-subscriber`

use axum::{http::StatusCode, response::IntoResponse, routing::get, Json, Router};
use std::net::SocketAddr;

// This derive macro allows our main function to run asyncrohnous code. Without it, the main function would run syncrohnously
#[tokio::main]
async fn main() {
    use std::time::Instant;
    let now = Instant::now();

    let resp = reqwest::get("https://testnet.binancefuture.com/fapi/v1/depth?symbol=btcusdt")
        .await?
        .text()
        .await?;

        let elapsed = now.elapsed();
    println!("Elapsed: {:.2?}", elapsed);
    println!("{:#?}", resp);
}

// This is our route handler, for the route root
// Make sure the function is `async`
// We specify our return type, `&'static str`, however a route handler can return anything that implements `IntoResponse`

async fn root() -> &'static str {
    "Hello, World!"
}

// This is our route handler, for the route complex
// Make sure the function is async
// We specify our return type, this time using `impl IntoResponse`

async fn complex() -> impl IntoResponse {
    // For this route, we are going to return a Json response
    // We create a tuple, with the first parameter being a `StatusCode`
    // Our second parameter, is the response body, which in this example is a `Json` instance
    // We construct data for the `Json` struct using the `serde_json::json!` macro
    (
        StatusCode::OK,
        Json(serde_json::json!({
            "message": "Hello, World!"
        })),
    )
}
