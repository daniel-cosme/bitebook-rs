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
pub struct Patient {
    pub id: u32,
    pub name: String,
    pub email: String,
    pub phone: String,
}

#[derive(Deserialize)]
pub struct CreatePatientRequest {
    pub name: String,
    pub email: String,
    pub phone: String,
}

pub async fn list_patients(State(engine): State<Arc<Engine>>) -> Result<Json<Vec<Patient>>, StatusCode> {
    let runtime = WasmRuntime::new(engine);
    
    match runtime
        .call_wasm_function::<(), Vec<Patient>>(
            "wasm_modules/patients.wasm",
            "list_patients",
            (),
        )
        .await
    {
        Ok(patients) => Ok(Json(patients)),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

pub async fn create_patient(
    State(engine): State<Arc<Engine>>,
    Json(payload): Json<CreatePatientRequest>,
) -> Result<Json<Patient>, StatusCode> {
    let runtime = WasmRuntime::new(engine);
    
    match runtime
        .call_wasm_function::<CreatePatientRequest, Patient>(
            "wasm_modules/patients.wasm",
            "create_patient",
            payload,
        )
        .await
    {
        Ok(patient) => Ok(Json(patient)),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}