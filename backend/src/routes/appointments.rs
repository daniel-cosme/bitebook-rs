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
pub struct Appointment {
    pub id: u32,
    pub patient_id: u32,
    pub date: String,
    pub time: String,
    pub description: String,
}

#[derive(Deserialize)]
pub struct CreateAppointmentRequest {
    pub patient_id: u32,
    pub date: String,
    pub time: String,
    pub description: String,
}

pub async fn list_appointments(
    State(engine): State<Arc<Engine>>,
) -> Result<Json<Vec<Appointment>>, StatusCode> {
    let runtime = WasmRuntime::new(engine);
    
    match runtime
        .call_wasm_function::<(), Vec<Appointment>>(
            "wasm_modules/appointments.wasm",
            "list_appointments",
            (),
        )
        .await
    {
        Ok(appointments) => Ok(Json(appointments)),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

pub async fn create_appointment(
    State(engine): State<Arc<Engine>>,
    Json(payload): Json<CreateAppointmentRequest>,
) -> Result<Json<Appointment>, StatusCode> {
    let runtime = WasmRuntime::new(engine);
    
    match runtime
        .call_wasm_function::<CreateAppointmentRequest, Appointment>(
            "wasm_modules/appointments.wasm",
            "create_appointment",
            payload,
        )
        .await
    {
        Ok(appointment) => Ok(Json(appointment)),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}