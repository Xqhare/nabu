use athena::{Array, Data, Metadata, Number, Object, Table, Uuid};
use nabu::XffValue;
use nabu::serde::{read, write_legacy};
use std::fs;

const GOLDEN_PATH: &str = "xff-example-data/v3_golden_reference.xff";

fn construct_master_value() -> Vec<XffValue> {
    let mut body_elements = Vec::new();

    // 1. Primitives
    body_elements.push(XffValue::Null);
    body_elements.push(XffValue::Boolean(true));
    body_elements.push(XffValue::Boolean(false));
    body_elements.push(XffValue::NaN);
    body_elements.push(XffValue::Infinity);
    body_elements.push(XffValue::NegInfinity);

    // 2. Numbers
    body_elements.push(XffValue::Number(Number::from(123456789usize)));
    body_elements.push(XffValue::Number(Number::from(-123456789isize)));
    body_elements.push(XffValue::Number(Number::from(3.1415926535f64)));

    // 3. Complex
    body_elements.push(XffValue::String("Golden Standard".to_string()));
    body_elements.push(XffValue::Data(Data::from(vec![0xDE, 0xAD, 0xBE, 0xEF])));

    // 4. Specialized
    body_elements.push(XffValue::DateTime(1741474800000));
    body_elements.push(XffValue::Duration(3600000));
    body_elements.push(XffValue::Uuid(Uuid::new([0xAA; 16])));

    // 5. Parents
    // Array
    body_elements.push(XffValue::Array(Array::from(vec![
        XffValue::from(1),
        XffValue::from(2),
    ])));

    // Object (Single element for determinism)
    let mut obj = Object::new();
    obj.insert("stability", "confirmed");
    body_elements.push(XffValue::Object(obj));

    // OrderedObject
    let oobj = vec![("order".to_string(), XffValue::from("preserved"))];
    body_elements.push(XffValue::OrderedObject(athena::OrderedObject::from(oobj)));

    // Table
    let mut table = Table::with_columns(vec!["id".to_string()]);
    table.add_row(vec![XffValue::from(1)]).unwrap();
    body_elements.push(XffValue::Table(table));

    // 6. Metadata
    let mut meta = Metadata::new();
    meta.set_creator("Nabu Reference Implementation".to_string());

    // IMPORTANT: Return [Metadata, Array(Body)]
    // The serializer will move Metadata to head, and serialize Array as the body.
    vec![
        XffValue::Metadata(meta),
        XffValue::Array(Array::from(body_elements)),
    ]
}

#[test]
#[ignore] // Run manually to update the golden file
fn generate_golden_reference() {
    let data = construct_master_value();
    write_legacy(GOLDEN_PATH, data, 3).expect("Failed to write golden reference");
    println!("Golden reference generated at {}", GOLDEN_PATH);
}

#[test]
fn test_v3_golden_reference_stability() {
    let temp_path = "v3_stability_temp"; // write() will append .xff
    let full_temp_path = "v3_stability_temp.xff";
    // 1. Reconstruct the value
    let data = construct_master_value();

    // 2. Serialize it to a temporary file
    write_legacy(temp_path, data, 3).expect("Serialization failed");
    let current_bytes = fs::read(full_temp_path).expect("Failed to read temp file");

    // 3. Load the reference from disk
    let reference_bytes = fs::read(GOLDEN_PATH)
        .expect("Could not find golden reference file. Run with -- --ignored to generate it.");

    // 4. Byte-by-byte comparison
    assert_eq!(
        current_bytes, reference_bytes,
        "BINARY REGRESSION DETECTED! The serialized output no longer matches the golden reference."
    );

    // 5. Round-trip verification
    let read_val = read(temp_path).expect("Failed to read golden file");
    // Note: read() returns the promoted Metadata structure [Meta, Body]
    if let XffValue::Array(ary) = read_val {
        assert!(ary[0].is_metadata());
        // The body in our case is an Array because we passed a Vec to write
        assert!(ary[1].is_array());
    } else {
        panic!("Read value should be [Metadata, Body] array");
    }

    let _ = fs::remove_file(full_temp_path);
}
