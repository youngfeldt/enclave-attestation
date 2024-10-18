use base64::{engine::general_purpose::STANDARD as BASE64_STANDARD, Engine};
use serde_cbor::from_slice;
use std::error::Error;
use std::fs;
use std::path::Path;

fn decode_attestation_document(base64_input: &str) -> Result<AttestationDoc, Box<dyn Error>> {
    // First, decode the Base64-encoded string to get the raw CBOR bytes
    let decoded_bytes = BASE64_STANDARD.decode(base64_input)?;

    // Use serde_cbor to deserialize the CBOR data into the AttestationDoc structure
    let attestation_doc: AttestationDoc = from_slice(&decoded_bytes)?;

    Ok(attestation_doc)
}

fn main() -> Result<(), Box<dyn Error>> {
    // Path to the base64-encoded attestation document
    let path = Path::new("attestation.b64");

    // Read the base64-encoded attestation document from a file
    let base64_string = fs::read_to_string(path)?;

    // Decode the attestation document
    let attestation_doc = decode_attestation_document(&base64_string)?;

    // Print out the decoded attestation document
    println!("{:#?}", attestation_doc);

    Ok(())
}

