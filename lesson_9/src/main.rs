use std::io;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum PaymentError {
    #[error("Invalid Card Number")]
    InvalidCard,
    #[error("Insufficient Funds")]
    InsufficientFunds,
    #[error("Payment Gateway Error: {0}")]
    GatewayError(String),
    #[error("Unknown Error Occurred")]
    Unknown,
}

pub fn process_payment(card_number: &str, amount: f64) -> Result<String, PaymentError> {
    if card_number.len() != 16 {
        return Err(PaymentError::InvalidCard);
    }

    if amount <= 1500.0 {
        return Err(PaymentError::InsufficientFunds);
    }

    // Simulating a payment gateway error
    if card_number.starts_with("1234") {
        return Err(PaymentError::GatewayError(
            "Payment gateway is down".to_string(),
        ));
    }

    // If everything is fine, return a success message
    Ok(format!(
        "Payment of ₹{:.2} processed successfully for card ending in {}",
        amount,
        &card_number[12..]
    ))
}

pub fn main() {
    println!("Enter Card Number (16 digits):");
    let mut card_number = String::new();
    io::stdin().read_line(&mut card_number).unwrap();

    println!("Enter Amount to pay:");
    let mut amount_str = String::new();
    io::stdin().read_line(&mut amount_str).unwrap();

    let amount: f64 = amount_str.trim().parse().unwrap_or(0.0);
    let card_number = card_number.trim();

    match process_payment(card_number, amount) {
        Ok(msg) => println!("{}", msg),
        Err(e) => eprintln!("Payment Error: {}", e),
    }
}
