use serde::{Deserialize, Serialize};
use std::str::FromStr;

#[derive(Serialize, Deserialize, Debug)]
pub struct Payment {
    pub id: u32,
    pub appointment_id: u32,
    pub amount: f64,
    pub method: String,
    pub date: String,
}

#[derive(Deserialize)]
struct CreatePaymentRequest {
    pub appointment_id: u32,
    pub amount: f64,
    pub method: String,
}

static mut PAYMENTS: Option<Vec<Payment>> = None;

fn get_payments() -> &'static mut Vec<Payment> {
    unsafe {
        if PAYMENTS.is_none() {
            PAYMENTS = Some(vec![
                Payment {
                    id: 1,
                    appointment_id: 1,
                    amount: 150.0,
                    method: "Credit Card".to_string(),
                    date: "2023-10-15".to_string(),
                },
                Payment {
                    id: 2,
                    appointment_id: 2,
                    amount: 80.0,
                    method: "Cash".to_string(),
                    date: "2023-10-16".to_string(),
                },
            ]);
        }
        PAYMENTS.as_mut().unwrap()
    }
}

#[export_name = "list_payments"]
pub extern "C" fn list_payments(input: *const u8, input_len: usize) -> *const u8 {
    // We ignore the input for this function
    let payments = get_payments();
    let result = serde_json::to_string(&payments).unwrap();
    
    // Convert to raw bytes and leak to preserve data
    let bytes = result.into_bytes();
    let ptr = bytes.as_ptr();
    std::mem::forget(bytes);
    ptr
}

#[export_name = "create_payment"]
pub extern "C" fn create_payment(input: *const u8, input_len: usize) -> *const u8 {
    // Convert input bytes to string
    let input_slice = unsafe { std::slice::from_raw_parts(input, input_len) };
    let input_str = std::str::from_utf8(input_slice).unwrap();
    
    // Parse the input
    let req: CreatePaymentRequest = serde_json::from_str(input_str).unwrap();
    
    // Create a new payment
    let mut payments = get_payments();
    let new_id = payments.iter().map(|p| p.id).max().unwrap_or(0) + 1;
    let new_payment = Payment {
        id: new_id,
        appointment_id: req.appointment_id,
        amount: req.amount,
        method: req.method,
        date: chrono::offset::Utc::now().format("%Y-%m-%d").to_string(),
    };
    
    payments.push(new_payment.clone());
    
    // Return the created payment as JSON string
    let result = serde_json::to_string(&new_payment).unwrap();
    
    // Convert to raw bytes and leak to preserve data
    let bytes = result.into_bytes();
    let ptr = bytes.as_ptr();
    std::mem::forget(bytes);
    ptr
}