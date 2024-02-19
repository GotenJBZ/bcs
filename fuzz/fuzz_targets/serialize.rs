#![no_main]
use libfuzzer_sys::fuzz_target;
use bcs::to_bytes;
use bcs::from_bytes;

fuzz_target!(|data: &[u8]| {

    to_bytes(data).expect("Failed to serialize");
});
