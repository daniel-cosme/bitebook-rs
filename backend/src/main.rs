use axum::{
    extract::State,
    http::StatusCode,
    response::Json,
    routing::{get, post},
    Router,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

mod wasm_runtime;
mod routes {
    pub mod patients;
    pub mod appointments;
    pub mod payments;
}

#[derive(Clone)]
struct AppState {
    wasm_engine: Arc<wasmtime::Engine>,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt::init();

    let engine = wasmtime::Engine::default();
    let state = AppState {
        wasm_engine: Arc::new(engine),
    };

    let app = Router::new()
        .route("/api/patients", get(routes::patients::list_patients))
        .route("/api/patients", post(routes::patients::create_patient))
        .route(
            "/api/appointments",
            get(routes::appointments::list_appointments),
        )
        .route(
            "/api/appointments",
            post(routes::appointments::create_appointment),
        )
        .route("/api/payments", get(routes::payments::list_payments))
        .route("/api/payments", post(routes::payments::create_payment))
        .with_state(state);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await?;
    println!("Server running on http://0.0.0.0:3000");
    axum::serve(listener, app).await?;

    Ok(())
}