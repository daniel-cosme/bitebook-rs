use anyhow::Result;
use serde::{de::DeserializeOwned, Serialize};
use std::sync::Arc;
use wasmtime::{Engine, Linker, Store};

pub struct WasmRuntime {
    engine: Arc<Engine>,
}

impl WasmRuntime {
    pub fn new(engine: Arc<Engine>) -> Self {
        Self { engine }
    }

    pub async fn call_wasm_function<T, R>(
        &self,
        wasm_file: &str,
        function_name: &str,
        input: T,
    ) -> Result<R>
    where
        T: Serialize,
        R: DeserializeOwned,
    {
        // Serialize the input to JSON
        let input_json = serde_json::to_string(&input)?;

        // Load the WASM module
        let mut store = Store::new(&self.engine, ());
        let module = wasmtime::Module::from_file(&self.engine, wasm_file)?;
        
        let mut linker = Linker::new(&self.engine);
        linker.func_wrap("env", "console_log", |caller: wasmtime::Caller<'_, ()>, ptr: i32, len: i32| {
            // Simple logging function if needed by WASM
        })?;

        let instance = linker.instantiate(&mut store, &module)?;

        // Get the exported function
        let func = instance.get_typed_func::<(i32, i32), i32>(&mut store, function_name)?;

        // Prepare input data - in a real scenario, we'd need to pass the input differently
        // For now, we'll just return an error if the WASM file doesn't exist
        if std::fs::metadata(wasm_file).is_err() {
            // Return mock data for development purposes when WASM is not compiled
            return Ok(serde_json::from_str("{\"error\":\"WASM file not found\"}")?);
        }

        // In a real implementation, we would need to properly interface with the WASM memory
        // to pass and retrieve data. This is a simplified placeholder.
        // For now, we'll return mock data for development.
        let mock_result = match function_name {
            "list_patients" => r#"[
                {"id": 1, "name": "John Doe", "email": "john@example.com", "phone": "(11) 99999-9999"},
                {"id": 2, "name": "Jane Smith", "email": "jane@example.com", "phone": "(11) 98888-8888"}
            ]"#,
            "create_patient" => r#"{"id": 3, "name": "New Patient", "email": "new@example.com", "phone": "(11) 97777-7777"}"#,
            "list_appointments" => r#"[
                {"id": 1, "patient_id": 1, "date": "2023-10-15", "time": "10:00", "description": "Cleaning"},
                {"id": 2, "patient_id": 2, "date": "2023-10-16", "time": "14:30", "description": "Filling"}
            ]"#,
            "create_appointment" => r#"{"id": 3, "patient_id": 1, "date": "2023-10-20", "time": "16:00", "description": "Checkup"}"#,
            "list_payments" => r#"[
                {"id": 1, "appointment_id": 1, "amount": 150.0, "method": "Credit Card", "date": "2023-10-15"},
                {"id": 2, "appointment_id": 2, "amount": 80.0, "method": "Cash", "date": "2023-10-16"}
            ]"#,
            "create_payment" => r#"{"id": 3, "appointment_id": 1, "amount": 120.0, "method": "Debit", "date": "2023-10-20"}"#,
            _ => r#"{}"#
        };

        let result: R = serde_json::from_str(mock_result)?;
        Ok(result)
    }
}