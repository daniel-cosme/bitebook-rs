use serde::{Deserialize, Serialize};
use std::str::FromStr;

#[derive(Serialize, Deserialize, Debug)]
pub struct Patient {
    pub id: u32,
    pub name: String,
    pub email: String,
    pub phone: String,
}

#[derive(Deserialize)]
struct CreatePatientRequest {
    pub name: String,
    pub email: String,
    pub phone: String,
}

static mut PATIENTS: Option<Vec<Patient>> = None;

fn get_patients() -> &'static mut Vec<Patient> {
    unsafe {
        if PATIENTS.is_none() {
            PATIENTS = Some(vec![
                Patient {
                    id: 1,
                    name: "John Doe".to_string(),
                    email: "john@example.com".to_string(),
                    phone: "(11) 99999-9999".to_string(),
                },
                Patient {
                    id: 2,
                    name: "Jane Smith".to_string(),
                    email: "jane@example.com".to_string(),
                    phone: "(11) 98888-8888".to_string(),
                },
            ]);
        }
        PATIENTS.as_mut().unwrap()
    }
}

#[export_name = "list_patients"]
pub extern "C" fn list_patients(input: *const u8, input_len: usize) -> *const u8 {
    // We ignore the input for this function
    let patients = get_patients();
    let result = serde_json::to_string(&patients).unwrap();
    
    // Convert to raw bytes and leak to preserve data
    let bytes = result.into_bytes();
    let ptr = bytes.as_ptr();
    std::mem::forget(bytes);
    ptr
}

#[export_name = "create_patient"]
pub extern "C" fn create_patient(input: *const u8, input_len: usize) -> *const u8 {
    // Convert input bytes to string
    let input_slice = unsafe { std::slice::from_raw_parts(input, input_len) };
    let input_str = std::str::from_utf8(input_slice).unwrap();
    
    // Parse the input
    let req: CreatePatientRequest = serde_json::from_str(input_str).unwrap();
    
    // Create a new patient
    let mut patients = get_patients();
    let new_id = patients.iter().map(|p| p.id).max().unwrap_or(0) + 1;
    let new_patient = Patient {
        id: new_id,
        name: req.name,
        email: req.email,
        phone: req.phone,
    };
    
    patients.push(new_patient.clone());
    
    // Return the created patient as JSON string
    let result = serde_json::to_string(&new_patient).unwrap();
    
    // Convert to raw bytes and leak to preserve data
    let bytes = result.into_bytes();
    let ptr = bytes.as_ptr();
    std::mem::forget(bytes);
    ptr
}