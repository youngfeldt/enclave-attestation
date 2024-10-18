use base64::{engine::general_purpose::STANDARD as BASE64_STANDARD, Engine};
use remote_attestation_verifier::{AttestationDocument, parse_document};
use serde_cbor::{from_slice, Value};
use std::error::Error;
use std::fs;
use std::path::Path;

// Function to decode the base64 and CBOR-encoded attestation document
fn decode_attestation_document(base64_input: &str) -> Result<AttestationDocument, Box<dyn Error>> {
    // Step 1: Decode base64 to get raw CBOR bytes
    let decoded_bytes = BASE64_STANDARD.decode(base64_input.trim())?;
    println!("DONE: Step 1: Base64 decoding");

    // Step 2: Parse the attestation document using `parse_document`
    let attestation_doc = parse_document(&decoded_bytes)?;
    println!("DONE: Step 2: Parsed attestation document");

    Ok(attestation_doc)
}

// Function to decode the CBOR-encoded payload and extract PCR values
fn extract_pcr_values(attestation_doc: &AttestationDocument) -> Result<(), Box<dyn Error>> {
    // Step 3: Extract the CBOR-encoded payload from the attestation document
    let payload_cbor = &attestation_doc.payload;
    println!("Start: Step 3: Decoding the payload bytes (CBOR format)");

    // Step 4: Deserialize the CBOR payload into a structured format
    let payload: Value = from_slice(payload_cbor)?;
    println!("DONE: Step 4: CBOR deserialization of payload");

    // Step 5: Check the payload for the "pcrs" field and extract PCR values
    if let Value::Map(map) = &payload {
        if let Some(pcr_value) = map.get(&Value::Text("pcrs".to_string())) {
            if let Value::Map(pcrs) = pcr_value {
                println!("PCR values:");
                // Iterate over the PCR values and print them
                // Note: This assumes that the PCR values are stored as bytes in the "pcrs" map
                // may need to adjust this logic based on the actual structure of the payload
                for (index, value) in pcrs {
                    if let (Value::Text(index), Value::Bytes(pcr_data)) = (index, value) {
                        println!("PCR[{}]: {:?}", index, pcr_data);
                    }
                }
            } else {
                println!("PCR field is not in the expected format.");
            }
        } else {
            println!("No PCR values found in the attestation document payload.");
        }
    } else {
        println!("Payload is not a map as expected.");
    }

    Ok(())
}

fn main() -> Result<(), Box<dyn Error>> {
    // Step 1: Read the base64-encoded attestation document from the file
    let path = Path::new("attestation.b64");
    let base64_string = fs::read_to_string(path)?;
    println!("DONE: Step 1: Reading base64-encoded attestation document from file");

    // Step 2: Decode the attestation document
    let attestation_doc = decode_attestation_document(&base64_string)?;

    // Step 3: Extract and print the PCR values
    extract_pcr_values(&attestation_doc)?;
    
    Ok(())
}
