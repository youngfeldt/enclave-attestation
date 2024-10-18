use base64::{engine::general_purpose::STANDARD as BASE64_STANDARD, Engine};
// use remote_attestation_verifier::{verify, parse_document};
use remote_attestation_verifier::parse_document;
use serde::{Deserialize, Serialize};
// use serde_cbor::from_slice;
use std::error::Error;
use std::fs;
use std::path::Path;

// Structure to deserialize attestation document from CBOR
// #[derive(Debug, Serialize, Deserialize)]
// struct AttestationDoc {
//     module_id: String,
//     timestamp: u64,
//     pcrs: std::collections::HashMap<String, String>,
//     public_key: String,
//     user_data: Option<String>,
//     nonce: Option<String>,
//     signature: String,
// }

// Decode base64 and CBOR-encoded attestation document
fn decode_attestation_document(base64_input: &str) -> Result<remote_attestation_verifier::AttestationDocument, Box<dyn Error>>{
    // Decode base64 to get the raw CBOR bytes
    println!("Decoding");
    let decoded_bytes = BASE64_STANDARD.decode(base64_input)?;
    println!("DONE decoding b64 ");

    // Deserialize the CBOR data into AttestationDoc structure
    let attestation_doc = parse_document(&decoded_bytes)?;

    println!("{:?}", attestation_doc); 
    // let json_output = serde_json::to_string_pretty(&attestation_doc)?;
    // println!("Decoded and Verified Attestation Document:\n{}", json_output);

    Ok(attestation_doc)
}

// Validate the attestation document signature
// fn verify_attestation_signature(attestation: &AttestationDoc) -> Result<(), Box<dyn Error>> {
//     // Use the verify function to validate the attestation document
//     // Placeholder for actual verification logic.
//     // Assuming the function `verify` performs signature validation.
    
//     // For now, let's assume the document is valid:
//     Ok(())
// }

fn main() -> Result<(), Box<dyn Error>> {
    // Path to the base64-encoded attestation document
    let path = Path::new("attestation.b64");

    // Read the base64-encoded attestation document
    let base64_string = fs::read_to_string(path)?;

    // Decode and deserialize the attestation document
    let attestation_doc = decode_attestation_document(&base64_string)?;
    println!("{:?}", attestation_doc); 

    // Validate the attestation document's signature
    // verify_attestation_signature(&attestation_doc)?;

    // Convert the decoded attestation document to JSON and pretty-print it
    // let json_output = serde_json::to_string_pretty(&attestation_doc)?;
    // println!("Decoded and Verified Attestation Document:\n{}", json_output);

    Ok(())
}

