use athena::{LocalDate, LocalDateTime, LocalTime, XffValue};
use athena::float::HpFloat;
use athena::graph::Graph;
use nabu::serde::{read, remove_file, write_legacy};

#[test]
fn test_v4_roundtrip_simple() {
    let path = "test_simple_v4.xff";
    let values = vec![
        XffValue::Null,
        XffValue::from(true),
        XffValue::from(false),
        XffValue::NaN,
        XffValue::PNan,
        XffValue::NNan,
        XffValue::Infinity,
        XffValue::NegInfinity,
    ];

    for val in values {
        write_legacy(path, val.clone(), 4).unwrap();
        let read_val = read(path).unwrap();
        assert_eq!(val, read_val);
    }
    remove_file(path).unwrap();
}

#[test]
fn test_v4_roundtrip_new_types() {
    let path = "test_new_types_v4.xff";

    // LocalDate
    let val = XffValue::LocalDate(LocalDate::new(2026, 5, 16));
    write_legacy(path, val.clone(), 4).unwrap();
    assert_eq!(val, read(path).unwrap());

    // LocalTime
    let val = XffValue::LocalTime(LocalTime::new(14, 30, 0, 123));
    write_legacy(path, val.clone(), 4).unwrap();
    assert_eq!(val, read(path).unwrap());

    // LocalDateTime
    let val = XffValue::LocalDateTime(LocalDateTime::new(
        LocalDate::new(2026, 5, 16),
        LocalTime::new(14, 30, 0, 123)
    ));
    write_legacy(path, val.clone(), 4).unwrap();
    assert_eq!(val, read(path).unwrap());

    // HpFloat
    let val = XffValue::HpFloat(HpFloat::new(12345, 2)); // 123.45
    write_legacy(path, val.clone(), 4).unwrap();
    assert_eq!(val, read(path).unwrap());

    // Ascii
    let val = XffValue::Ascii(athena::XffString::from("Hello ASCII"));
    write_legacy(path, val.clone(), 4).unwrap();
    assert_eq!(val, read(path).unwrap());

    remove_file(path).unwrap();
}

#[test]
fn test_v4_roundtrip_graph() {
    let path = "test_graph_v4.xff";
    let mut g = Graph::new();
    let n0 = g.add_node(XffValue::from("root"), XffValue::Null);
    let n1 = g.add_node(XffValue::from("child"), XffValue::from("meta"));
    g.add_connection(n0, n1, XffValue::from("link")).unwrap();
    
    let val = XffValue::Graph(g);
    write_legacy(path, val.clone(), 4).unwrap();
    let read_val = read(path).unwrap();
    assert_eq!(val, read_val);
    remove_file(path).unwrap();
}

#[test]
fn test_v4_delta_encoding_large() {
    let path = "test_large_v4.xff";
    let mut values = Vec::new();
    // Add many values to ensure offsets grow beyond 1 byte in LEB128
    for i in 0..200 {
        values.push(XffValue::Data(athena::Data::from(vec![i as u8; 2])));
    }
    let val = XffValue::Array(athena::Array::from(values));
    write_legacy(path, val.clone(), 4).unwrap();
    assert_eq!(val, read(path).unwrap());
    remove_file(path).unwrap();
}

#[test]
fn test_v4_hamming_distance_simulation() {
    use nabu::XffValue;
    use nabu::serde::{write_legacy};
    
    let path = "test_hamming.xff";
    let val = XffValue::from(true); // TRU is 0x87 (1000 0111)
    write_legacy(path, val, 4).unwrap();
    
    let mut content = std::fs::read(path).unwrap();
    // Find the TRU marker. It's after magic (4) and version (1) and optional meta (none).
    // So byte 5 should be 0x87.
    assert_eq!(content[5], 0x87);
    
    // 1-bit flip: 0x87 -> 0x86 (1000 0110)
    content[5] = 0x86;
    std::fs::write(path, &content).unwrap();
    assert!(read(path).is_err()); // Parity error
    
    // 2-bit flip: 0x87 -> 0x85 (1000 0101)
    content[5] = 0x85;
    std::fs::write(path, &content).unwrap();
    assert!(read(path).is_err()); // Even parity but not a valid v4 marker
    
    // 3-bit flip: 0x87 -> 0x84 (1000 0100)
    content[5] = 0x84;
    std::fs::write(path, &content).unwrap();
    assert!(read(path).is_err()); // Parity error
    
    // 4-bit flip: 0x87 -> 0x00 (0000 0000) which is NUL
    content[5] = 0x00;
    std::fs::write(path, &content).unwrap();
    assert!(read(path).is_ok()); // Valid marker (NUL)
    
    remove_file(path).unwrap();
}
