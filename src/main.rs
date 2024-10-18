use base64::{engine::general_purpose::STANDARD as BASE64_STANDARD, Engine};
use remote_attestation_verifier::{parse_document, AttestationDocument};
use serde::{Serialize};
use std::error::Error;
use std::fs;
use std::path::Path;
use std::collections::HashMap;

// Structure to hold just the timestamp and PCR values
#[derive(Serialize)]
struct SimplifiedAttestationDoc {
    timestamp: u64,
    pcrs: HashMap<String, String>,
}

// Function to decode base64 and CBOR-encoded attestation document and extract specific fields
fn decode_attestation_document(base64_input: &str) -> Result<SimplifiedAttestationDoc, Box<dyn Error>> {
    // Decode base64 to get the raw CBOR bytes
    let decoded_bytes = BASE64_STANDARD.decode(base64_input)?;

    // Parse the attestation document
    let attestation_doc: AttestationDocument = parse_document(&decoded_bytes)?;

    dbg!(&attestation_doc); 
    // Extract the fields we're interested in: timestamp and PCR values
    // let simplified_doc = SimplifiedAttestationDoc {
    //     timestamp: attestation_doc.timestamp, // Assuming `timestamp` field exists
    //     pcrs: attestation_doc.pcrs,           // Assuming `pcrs` field exists and is a map
    // };

    Ok(simplified_doc)
}

fn main() -> Result<(), Box<dyn Error>> {
    // Path to the base64-encoded attestation document
    let path = Path::new("attestation.b64");

    // Read the base64-encoded attestation document
    let base64_string = fs::read_to_string(path)?;

    // Decode and extract the timestamp and PCR values
    // let simplified_attestation_doc = decode_attestation_document(&base64_string)?;

    // // Serialize the simplified structure to JSON and pretty-print it
    // let json_output = serde_json::to_string_pretty(&simplified_attestation_doc)?;
    // println!("Simplified Attestation Document:\n{}", json_output);

    Ok(())
}

