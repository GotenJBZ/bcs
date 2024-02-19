#![no_main]
use libfuzzer_sys::fuzz_target;
use bcs::to_bytes;
use bcs::from_bytes;

fuzz_target!(|data: &[u8]| {

    let serialized = to_bytes(data).expect("Failed to serialize");

    let deserialized: Vec<u8> = from_bytes(&serialized).expect("Failed to deserialize");
    
    assert_eq!(data, deserialized, "Data mismatch after serialize/deserialize cycle");
});
