use crate::wasm_runtime::WasmRuntime;
use axum::{
    extract::State,
    http::StatusCode,
    response::Json,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use wasmtime::Engine;

#[derive(Serialize, Deserialize, Debug)]
pub struct Payment {
    pub id: u32,
    pub appointment_id: u32,
    pub amount: f64,
    pub method: String,
    pub date: String,
}

#[derive(Deserialize)]
pub struct CreatePaymentRequest {
    pub appointment_id: u32,
    pub amount: f64,
    pub method: String,
}

pub async fn list_payments(State(engine): State<Arc<Engine>>) -> Result<Json<Vec<Payment>>, StatusCode> {
    let runtime = WasmRuntime::new(engine);
    
    match runtime
        .call_wasm_function::<(), Vec<Payment>>(
            "wasm_modules/payments.wasm",
            "list_payments",
            (),
        )
        .await
    {
        Ok(payments) => Ok(Json(payments)),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

pub async fn create_payment(
    State(engine): State<Arc<Engine>>,
    Json(payload): Json<CreatePaymentRequest>,
) -> Result<Json<Payment>, StatusCode> {
    let runtime = WasmRuntime::new(engine);
    
    match runtime
        .call_wasm_function::<CreatePaymentRequest, Payment>(
            "wasm_modules/payments.wasm",
            "create_payment",
            payload,
        )
        .await
    {
        Ok(payment) => Ok(Json(payment)),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}