use base64::prelude::*;  // Import for Base64 decoding
use serde::{Deserialize, Serialize};  // For serde serialization/deserialization
use serde_cbor::from_slice;  // For CBOR decoding
use std::collections::HashMap;  // For HashMap used in the struct
use std::error::Error;  // For error handling
use std::fs;  // For reading files
use std::path::Path;  // For handling file paths

// Define the structure for the outer attestation document
#[derive(Debug, Serialize, Deserialize)]
struct AttestationDoc {
    payload: Vec<u8>,       // The payload is Base64-encoded, needs to be decoded
    signature: Vec<u8>,     // Signature as a byte array
    public_key: Vec<u8>,    // Public key as a byte array
}

// Define the structure for the decoded payload (inner document)
#[derive(Debug, Serialize, Deserialize)]
struct AttestationPayload {
    module_id: String,
    digest: String,
    pcrs: HashMap<String, String>,
    user_data: Option<String>,
    nonce: Option<String>,
    timestamp: Option<u64>,
}

// Function to decode the attestation document and extract the payload
fn decode_attestation_document(base64_input: &str) -> Result<(), Box<dyn Error>> {
    // Step 1: Decode base64 to get the raw CBOR bytes of the outer document
    let decoded_bytes = BASE64_STANDARD.decode(base64_input.trim())?;
    println!("1. Base64 Decoded");

    // Step 2: Try to debug print the CBOR data before deserialization
    match serde_cbor::from_slice::<serde_cbor::Value>(&decoded_bytes) {
        Ok(value) => {
            println!("2. CBOR Decoded Value: {:#?}", value);
        }
        Err(e) => {
            println!("Failed to decode CBOR: {:?}", e);
        }
    }

    Ok(())
}

fn main() -> Result<(), Box<dyn Error>> {
    // Path to the base64-encoded attestation document
    let path = Path::new("attestation.b64");

    // Step 1: Read the base64-encoded attestation document from the file
    let base64_string = fs::read_to_string(path)?;

    // Step 2: Decode and deserialize the attestation document
    decode_attestation_document(&base64_string)?;

    Ok(())
}
