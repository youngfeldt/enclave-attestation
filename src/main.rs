use base64::{engine::general_purpose::STANDARD as BASE64_STANDARD, Engine};
use remote_attestation_verifier::{AttestationDocument, parse_document, verify};
use serde_cbor::from_slice;
use std::collections::HashMap;
use std::error::Error;
use std::fs;
use std::path::Path;

// Struct for the decoded payload, containing PCRs and other fields
#[derive(Debug, serde::Deserialize)]
struct AttestationPayload {
    pcrs: Option<HashMap<String, Vec<u8>>>, // PCR values are usually byte arrays
    // Include other fields from the payload as needed
}

// Function to decode the base64 and CBOR-encoded attestation document
fn decode_attestation_document(base64_input: &str) -> Result<AttestationDocument, Box<dyn Error>> {
    // Step 1: Decode base64 to get raw CBOR bytes
    let decoded_bytes = BASE64_STANDARD.decode(base64_input.trim())?;
    
    // Step 2: Parse the attestation document using the provided `parse_document` function
    let attestation_doc = parse_document(&decoded_bytes)?;

    Ok(attestation_doc)
}

// Function to extract PCR values from the payload
fn extract_pcr_values(attestation_doc: &AttestationDocument) -> Result<(), Box<dyn Error>> {
    // Step 1: Decode the CBOR-encoded payload
    let payload_cbor = &attestation_doc.payload;
    
    // Step 2: Deserialize the CBOR payload into a structured format
    let payload: AttestationPayload = from_slice(payload_cbor)?;
    
    // Step 3: Extract and print PCR values
    if let Some(pcrs) = payload.pcrs {
        for (index, value) in pcrs.iter() {
            println!("PCR[{}]: {:?}", index, value);
        }
    } else {
        println!("No PCR values found in the attestation document.");
    }
    
    Ok(())
}

fn main() -> Result<(), Box<dyn Error>> {
    // Step 1: Read the base64-encoded attestation document
    let path = Path::new("attestation.b64");
    let base64_string = fs::read_to_string(path)?;

    // Step 2: Decode the attestation document
    let attestation_doc = decode_attestation_document(&base64_string)?;

    // Step 3: Extract and print the PCR values
    extract_pcr_values(&attestation_doc)?;
    
    Ok(())
}