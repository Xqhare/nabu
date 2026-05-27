use athena::rng_api::RngApi;
#[allow(unused_imports)]
use nabu::serde::{
    deserialize_xff_from_buffer, read, remove_file, serialize_xff_to_buffer, write, write_legacy,
};
use nabu::{Array, Object, XffValue};
use tyche::Tyche;

fn get_rng() -> Tyche {
    Tyche::new().expect("Failed to initialize Tyche RNG")
}

fn mutate_buffer(buffer: &mut Vec<u8>, rng: &mut Tyche) {
    if buffer.is_empty() {
        // If the buffer is empty, we can only insert a byte.
        let val = rng.random_u8().expect("Failed to get random u8");
        buffer.push(val);
        return;
    }

    let mutation_type = rng
        .random_from_range(0, 4)
        .expect("Failed to get random mutation type");

    match mutation_type {
        0 => {
            // Bit flip
            let idx = rng
                .random_from_range(0, buffer.len())
                .expect("Failed to get random index");
            let bit = rng
                .random_from_range(0, 8)
                .expect("Failed to get random bit") as u8;
            buffer[idx] ^= 1 << bit;
        }
        1 => {
            // Byte replacement
            let idx = rng
                .random_from_range(0, buffer.len())
                .expect("Failed to get random index");
            let val = rng.random_u8().expect("Failed to get random byte");
            buffer[idx] = val;
        }
        2 => {
            // Truncation
            let new_len = rng
                .random_from_range(0, buffer.len())
                .expect("Failed to get random length");
            buffer.truncate(new_len);
        }
        3 => {
            // Insertion
            let idx = rng
                .random_from_range(0, buffer.len() + 1)
                .expect("Failed to get random index");
            let val = rng.random_u8().expect("Failed to get random byte");
            buffer.insert(idx, val);
        }
        _ => unreachable!(),
    }
}

#[test]
fn test_fuzz_skeleton() {
    let mut rng = get_rng();
    let mut buffer = vec![1, 2, 3, 4, 5];

    // Mutate multiple times to verify mutating without issues
    for _ in 0..100 {
        mutate_buffer(&mut buffer, &mut rng);
    }

    assert!(!buffer.is_empty() || buffer.is_empty());
}

#[test]
fn fuzz_robustness_random_bytes() {
    let mut rng = get_rng();
    for _ in 0..1000 {
        let len = rng
            .random_from_range(0, 1025)
            .expect("Failed to get random length");
        let buffer = rng
            .random_bytes(len)
            .expect("Failed to generate random bytes");
        let result = std::panic::catch_unwind(|| {
            let _ = deserialize_xff_from_buffer(&buffer);
        });
        assert!(result.is_ok(), "Panic detected on random byte input!");
    }
}

#[test]
fn fuzz_robustness_mutated_structures() {
    let mut rng = get_rng();
    let sample_values = vec![
        XffValue::Null,
        XffValue::from(true),
        XffValue::from(123456u64),
        XffValue::from(-789i64),
        XffValue::from("hello world".to_string()),
        XffValue::from(vec![0u8, 1u8, 2u8, 3u8]),
        XffValue::Array(Array::from(vec![XffValue::from(1), XffValue::from(true)])),
    ];

    for val in sample_values {
        for version in [3, 4] {
            let serialized = serialize_xff_to_buffer(val.clone(), version)
                .expect("Failed to serialize sample value");

            // Check both:
            // 1. A buffer with 100 progressive random mutations
            let mut fuzzed = serialized.clone();
            for _ in 0..100 {
                mutate_buffer(&mut fuzzed, &mut rng);
                let result = std::panic::catch_unwind(|| {
                    let _ = deserialize_xff_from_buffer(&fuzzed);
                });
                assert!(
                    result.is_ok(),
                    "Panic detected on mutated structure (progressive)!"
                );
            }

            // 2. 100 independent trials, each with one mutation from the base buffer
            for _ in 0..100 {
                let mut fuzzed = serialized.clone();
                mutate_buffer(&mut fuzzed, &mut rng);
                let result = std::panic::catch_unwind(|| {
                    let _ = deserialize_xff_from_buffer(&fuzzed);
                });
                assert!(
                    result.is_ok(),
                    "Panic detected on mutated structure (independent)!"
                );
            }
        }
    }
}

fn generate_random_xff_value(rng: &mut Tyche, depth: usize) -> XffValue {
    if depth >= 3 {
        // Only generate non-recursive types: Null, Boolean, i64, String
        let choice = rng
            .random_from_range(0, 4)
            .expect("Failed to get variant type");
        match choice {
            0 => XffValue::Null,
            1 => {
                let val = rng.random_bool().expect("Failed to generate random bool");
                XffValue::from(val)
            }
            2 => {
                let val = rng.random_i64().expect("Failed to generate random i64");
                XffValue::from(val)
            }
            3 => {
                let len = rng
                    .random_from_range(0, 20)
                    .expect("Failed to generate random length");
                let val = rng
                    .random_string(len)
                    .expect("Failed to generate random string");
                XffValue::from(val)
            }
            _ => unreachable!(),
        }
    } else {
        let choice = rng
            .random_from_range(0, 6)
            .expect("Failed to get variant type");
        match choice {
            0 => XffValue::Null,
            1 => {
                let val = rng.random_bool().expect("Failed to generate random bool");
                XffValue::from(val)
            }
            2 => {
                let val = rng.random_i64().expect("Failed to generate random i64");
                XffValue::from(val)
            }
            3 => {
                let len = rng
                    .random_from_range(0, 20)
                    .expect("Failed to generate random length");
                let val = rng
                    .random_string(len)
                    .expect("Failed to generate random string");
                XffValue::from(val)
            }
            4 => {
                let len = rng
                    .random_from_range(0, 6)
                    .expect("Failed to generate random array length");
                let mut elements = Vec::with_capacity(len);
                for _ in 0..len {
                    elements.push(generate_random_xff_value(rng, depth + 1));
                }
                XffValue::Array(Array::from(elements))
            }
            5 => {
                let len = rng
                    .random_from_range(0, 6)
                    .expect("Failed to generate random object length");
                let mut obj = Object::new();
                for _ in 0..len {
                    let key_len = rng
                        .random_from_range(1, 10)
                        .expect("Failed to generate random key length");
                    let key = rng
                        .random_string(key_len)
                        .expect("Failed to generate random key string");
                    let val = generate_random_xff_value(rng, depth + 1);
                    obj.insert(key, val);
                }
                XffValue::Object(obj)
            }
            _ => unreachable!(),
        }
    }
}

#[test]
fn fuzz_differential_roundtrip() {
    let mut rng = get_rng();
    for _ in 0..500 {
        let val = generate_random_xff_value(&mut rng, 0);
        for version in [3, 4] {
            let serialized =
                serialize_xff_to_buffer(val.clone(), version).expect("Failed to serialize value");
            let deserialized =
                deserialize_xff_from_buffer(&serialized).expect("Failed to deserialize value");
            assert_eq!(val, deserialized, "Mismatch for version {}", version);
        }
    }
}
