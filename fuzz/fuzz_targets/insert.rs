#![no_main]

use libfuzzer_sys::fuzz_target;

fuzz_target!(|data: &[u8]| {
    vec_multi_tree::fuzz_insert(data);
});
