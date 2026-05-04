use std::panic;

use cust::interpret;

#[test]
fn generated_malformed_programs_do_not_panic() {
    let alphabet: Vec<char> =
        "int main() { return 0; }[]*&+-/%=!<>,;\n\t\"'\\abcdefghijklmnopqrstuvwxyz0123456789_@#$"
            .chars()
            .collect();
    let mut state = 0xC057_F022_u64;

    for case_index in 0..512 {
        let len = (next_u64(&mut state) % 192) as usize;
        let mut source = String::new();
        if case_index % 4 == 0 {
            source.push_str("int main() {\n");
        }
        for _ in 0..len {
            let ch = alphabet[(next_u64(&mut state) as usize) % alphabet.len()];
            source.push(ch);
        }
        if case_index % 4 == 0 {
            source.push_str("\n}\n");
        }

        let result = panic::catch_unwind(|| {
            let _ = interpret(&source);
        });

        assert!(
            result.is_ok(),
            "interpret panicked for generated case {case_index}: {source:?}"
        );
    }
}

#[test]
fn arbitrary_byte_inputs_do_not_panic_after_lossy_utf8_decoding() {
    let mut state = 0xC057_BA7E_u64;

    for case_index in 0..512 {
        let len = (next_u64(&mut state) % 256) as usize;
        let mut bytes = Vec::with_capacity(len);
        for _ in 0..len {
            bytes.push((next_u64(&mut state) & 0xff) as u8);
        }
        let source = String::from_utf8_lossy(&bytes).into_owned();

        let result = panic::catch_unwind(|| {
            let _ = interpret(&source);
        });

        assert!(
            result.is_ok(),
            "interpret panicked for arbitrary byte case {case_index}: {bytes:?}"
        );
    }
}

fn next_u64(state: &mut u64) -> u64 {
    *state = state
        .wrapping_mul(6_364_136_223_846_793_005)
        .wrapping_add(1_442_695_040_888_963_407);
    *state
}
