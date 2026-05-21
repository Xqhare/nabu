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

#[test]
fn test_v4_compliance_fixes() {
    use athena::{LocalDate, LocalTime, LocalDateTime, Metadata, XffValue, Data};
    use athena::float::HpFloat;
    use athena::checksum::crc32;
    use nabu::serde::{read, write_legacy, remove_file};

    let path = "test_compliance_fixes.xff";

    // Helper to get serialized value bytes for any XffValue
    fn get_serialized_value_bytes(val: XffValue) -> Vec<u8> {
        let temp_path = "temp_val.xff";
        write_legacy(temp_path, val, 4).unwrap();
        let content = std::fs::read(temp_path).unwrap();
        let _ = std::fs::remove_file(temp_path);
        // XFF v4 file structure: Magic (4 bytes) + Version (1 byte) + Value + EM (1 byte)
        // Strip the first 5 bytes (Magic + Version) and the last byte (EM terminator)
        let len = content.len();
        content[5..len - 1].to_vec()
    }

    fn encode_leb128(mut val: u128) -> Vec<u8> {
        let mut buf = Vec::new();
        loop {
            let mut byte = (val & 0x7F) as u8;
            val >>= 7;
            if val != 0 {
                byte |= 0x80;
            }
            buf.push(byte);
            if val == 0 {
                break;
            }
        }
        buf
    }

    // 1. Test Object/OrderedObject/Metadata key deserialization with XffValue::Ascii keys.
    let key_bytes = get_serialized_value_bytes(XffValue::Ascii(athena::XffString::from("ascii_key")));
    let val_bytes = get_serialized_value_bytes(XffValue::from("value"));

    let mut element_bytes = Vec::new();
    element_bytes.extend(key_bytes.clone());
    element_bytes.extend(val_bytes);

    let count = 2; // key + value = 2 elements
    let mut index_data = encode_leb128(count);
    index_data.extend(encode_leb128(0));
    index_data.extend(encode_leb128(key_bytes.len() as u128));

    let index_checksum = crc32(&index_data);

    let mut obj_bytes = Vec::new();
    obj_bytes.push(0x41); // parent::OBJ
    obj_bytes.extend(index_data);
    obj_bytes.extend_from_slice(&index_checksum.to_le_bytes());
    obj_bytes.extend(element_bytes);
    obj_bytes.push(0x60); // internal::EV

    // Construct a full XFF v4 file containing this object
    let mut file_bytes = Vec::new();
    file_bytes.extend_from_slice(b"XFFV");
    file_bytes.push(0x0F); // version 4 bit-chain
    file_bytes.extend(obj_bytes);
    file_bytes.push(0xF0); // internal::EM

    std::fs::write(path, &file_bytes).unwrap();
    let read_val = read(path).unwrap();
    assert!(read_val.is_object());
    let read_obj = read_val.as_object().unwrap();
    assert_eq!(read_obj.get("ascii_key"), Some(&XffValue::from("value")));

    // 2. Test metadata flatness check with all v4 flat types and Data.
    let mut meta = Metadata::new();
    meta.set_custom("ascii", XffValue::Ascii(athena::XffString::from("hello")));
    meta.set_custom("date", XffValue::LocalDate(LocalDate::new(2026, 5, 21)));
    meta.set_custom("time", XffValue::LocalTime(LocalTime::new(18, 0, 0, 0)));
    meta.set_custom("datetime", XffValue::LocalDateTime(LocalDateTime::new(LocalDate::new(2026, 5, 21), LocalTime::new(18, 0, 0, 0))));
    meta.set_custom("hpfloat", XffValue::HpFloat(HpFloat::new(12345, 2)));
    meta.set_custom("pnan", XffValue::PNan);
    meta.set_custom("nnan", XffValue::NNan);
    meta.set_custom("data", XffValue::Data(Data::from(vec![1, 2, 3])));

    let val_meta = XffValue::Metadata(meta);
    write_legacy(path, val_meta.clone(), 4).unwrap();
    let read_meta = read(path).unwrap();
    if let XffValue::Array(arr) = read_meta {
        assert_eq!(arr.len(), 2);
        assert_eq!(arr[0], val_meta);
        assert_eq!(arr[1], XffValue::Null);
    } else {
        panic!("Expected Array with metadata and body");
    }

    // 3. Verify custom NaN payload preservation.
    let custom_nan_bits = 0x7ff8_0000_0000_0001;
    let custom_nan = f64::from_bits(custom_nan_bits);
    assert!(custom_nan.is_nan());

    let val_nan = XffValue::from(custom_nan);
    write_legacy(path, val_nan, 4).unwrap();
    
    let content = std::fs::read(path).unwrap();
    assert_eq!(content[5], 0x3F); // complex::FLT

    let read_nan = read(path).unwrap();
    let read_nan_val = read_nan.into_number().unwrap().into_f64().unwrap();
    assert!(read_nan_val.is_nan());
    assert_eq!(read_nan_val.to_bits(), custom_nan_bits);

    // 4. Verification that ASCII deserialization ignores/clears the MSB.
    let raw_text = b"h\xE5llo"; // 'e' with MSB set is 0xE5 (normal 'e' is 0x65)
    let mut payload = encode_leb128(raw_text.len() as u128);
    payload.extend_from_slice(raw_text);
    let checksum = crc32(&payload);

    let mut asci_val_bytes = Vec::new();
    asci_val_bytes.push(0xA5); // complex::ASCI
    asci_val_bytes.extend(payload);
    asci_val_bytes.extend_from_slice(&checksum.to_le_bytes());
    asci_val_bytes.push(0x60); // internal::EV

    let mut file_bytes = Vec::new();
    file_bytes.extend_from_slice(b"XFFV");
    file_bytes.push(0x0F); // version 4 bit-chain
    file_bytes.extend(asci_val_bytes);
    file_bytes.push(0xF0); // internal::EM

    std::fs::write(path, &file_bytes).unwrap();
    let read_asci = read(path).unwrap();
    assert_eq!(read_asci, XffValue::Ascii(athena::XffString::from("hello")));

    remove_file(path).unwrap();
    let _ = std::fs::remove_file(path);
}

