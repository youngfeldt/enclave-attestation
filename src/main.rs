use base64::{engine::general_purpose::STANDARD as BASE64_STANDARD, Engine};
use serde::{Deserialize, Serialize};
use serde_cbor::from_slice;
use std::collections::HashMap;
use std::error::Error;
use std::fs;
use std::path::Path;

// Define the structure for the outer attestation document
#[derive(Debug, Serialize, Deserialize)]
struct AttestationDoc {
    payload: Vec<u8>,       // The payload is CBOR-encoded
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
fn decode_attestation_document(base64_input: &str) -> Result<AttestationPayload, Box<dyn Error>> {
    // Step 1: Decode base64 to get the raw CBOR bytes
    let decoded_bytes = BASE64_STANDARD.decode(base64_input.trim())?;

    // Step 2: Deserialize the outer attestation document
    let attestation_doc: AttestationDoc = from_slice(&decoded_bytes)?;

    // Step 3: Deserialize the payload (which is CBOR-encoded)
    let payload: AttestationPayload = from_slice(&attestation_doc.payload)?;

    Ok(payload)
}

fn main() -> Result<(), Box<dyn Error>> {
    // Path to the base64-encoded attestation document
    let path = Path::new("attestation.b64");

    // Read the base64-encoded attestation document
    let base64_string = fs::read_to_string(path)?;

    // Decode and deserialize the attestation document
    let payload = decode_attestation_document(&base64_string)?;

    // Print the decoded fields from the payload
    println!("{:#?}", payload);

    Ok(())
}
