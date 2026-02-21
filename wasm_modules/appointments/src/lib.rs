use serde::{Deserialize, Serialize};
use std::str::FromStr;

#[derive(Serialize, Deserialize, Debug)]
pub struct Appointment {
    pub id: u32,
    pub patient_id: u32,
    pub date: String,
    pub time: String,
    pub description: String,
}

#[derive(Deserialize)]
struct CreateAppointmentRequest {
    pub patient_id: u32,
    pub date: String,
    pub time: String,
    pub description: String,
}

static mut APPOINTMENTS: Option<Vec<Appointment>> = None;

fn get_appointments() -> &'static mut Vec<Appointment> {
    unsafe {
        if APPOINTMENTS.is_none() {
            APPOINTMENTS = Some(vec![
                Appointment {
                    id: 1,
                    patient_id: 1,
                    date: "2023-10-15".to_string(),
                    time: "10:00".to_string(),
                    description: "Cleaning".to_string(),
                },
                Appointment {
                    id: 2,
                    patient_id: 2,
                    date: "2023-10-16".to_string(),
                    time: "14:30".to_string(),
                    description: "Filling".to_string(),
                },
            ]);
        }
        APPOINTMENTS.as_mut().unwrap()
    }
}

#[export_name = "list_appointments"]
pub extern "C" fn list_appointments(input: *const u8, input_len: usize) -> *const u8 {
    // We ignore the input for this function
    let appointments = get_appointments();
    let result = serde_json::to_string(&appointments).unwrap();
    
    // Convert to raw bytes and leak to preserve data
    let bytes = result.into_bytes();
    let ptr = bytes.as_ptr();
    std::mem::forget(bytes);
    ptr
}

#[export_name = "create_appointment"]
pub extern "C" fn create_appointment(input: *const u8, input_len: usize) -> *const u8 {
    // Convert input bytes to string
    let input_slice = unsafe { std::slice::from_raw_parts(input, input_len) };
    let input_str = std::str::from_utf8(input_slice).unwrap();
    
    // Parse the input
    let req: CreateAppointmentRequest = serde_json::from_str(input_str).unwrap();
    
    // Create a new appointment
    let mut appointments = get_appointments();
    let new_id = appointments.iter().map(|a| a.id).max().unwrap_or(0) + 1;
    let new_appointment = Appointment {
        id: new_id,
        patient_id: req.patient_id,
        date: req.date,
        time: req.time,
        description: req.description,
    };
    
    appointments.push(new_appointment.clone());
    
    // Return the created appointment as JSON string
    let result = serde_json::to_string(&new_appointment).unwrap();
    
    // Convert to raw bytes and leak to preserve data
    let bytes = result.into_bytes();
    let ptr = bytes.as_ptr();
    std::mem::forget(bytes);
    ptr
}