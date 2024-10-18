use base64::{engine::general_purpose::STANDARD as BASE64_STANDARD, Engine};
use remote_attestation_verifier::{AttestationDocument, parse_document, verify};
use serde_cbor::{from_slice, Value};  // Import Value to help with debugging
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
    println!("START: Step 1: Decoding base64 to get raw CBOR bytes");

    // Step 1: Decode base64 to get raw CBOR bytes
    let decoded_bytes = BASE64_STANDARD.decode(base64_input.trim())?;
    println!("DONE: Step 1: Base64 decoded, now processing CBOR bytes");

    // Step 2: Parse the attestation document using the provided `parse_document` function
    println!("START: Step 2: Parsing the attestation document using `parse_document`");
    let attestation_doc = parse_document(&decoded_bytes)?;
    println!("DONE: Step 2: Attestation document parsed successfully");

    Ok(attestation_doc)
}

// Function to decode the inner payload bytestring
fn decode_payload(payload_bytes: &[u8]) -> Result<AttestationPayload, Box<dyn Error>> {
    println!("START: Decoding the payload bytes (CBOR format)");

    // Deserialize the CBOR payload into the AttestationPayload struct
    let payload: AttestationPayload = from_slice(payload_bytes)?;
    println!("DONE: Payload successfully deserialized");

    Ok(payload)
}

// Function to extract PCR values from the attestation document
fn extract_pcr_values(attestation_doc: &AttestationDocument) -> Result<(), Box<dyn Error>> {
    println!("START: Extracting the CBOR-encoded payload from the attestation document");

    // Step 1: The payload is a bytestring, we need to decode it as CBOR
    let payload_cbor = &attestation_doc.payload;
    println!("DONE: Extracted the CBOR payload as a byte string");

    // Step 2: Decode the CBOR-encoded payload into AttestationPayload struct
    let payload = decode_payload(payload_cbor)?;
    
    // Step 3: Extract and print PCR values
    println!("START: Checking and printing PCR values from the payload");
    if let Some(pcrs) = payload.pcrs {
        for (index, value) in pcrs.iter() {
            println!("PCR[{}]: {:?}", index, value);
        }
    } else {
        println!("No PCR values found in the attestation document.");
    }
    println!("DONE: PCR values processed");

    Ok(())
}

fn main() -> Result<(), Box<dyn Error>> {
    // Step 1: Read the base64-encoded attestation document
    println!("START: Main Step 1: Reading base64-encoded attestation document from file");
    let path = Path::new("attestation.b64");
    let base64_string = fs::read_to_string(path)?;
    println!("DONE: Main Step 1: Attestation document read successfully");

    // Step 2: Decode the attestation document
    println!("START: Main Step 2: Decoding the attestation document");
    let attestation_doc = decode_attestation_document(&base64_string)?;
    println!("DONE: Main Step 2: Attestation document decoded");

    // Step 3: Extract and print the PCR values
    println!("START: Main Step 3: Extracting and printing PCR values from attestation document");
    extract_pcr_values(&attestation_doc)?;
    println!("DONE: Main Step 3: PCR values extracted and printed");

    Ok(())
}
