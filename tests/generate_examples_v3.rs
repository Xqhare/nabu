use athena::{Data, Object, Table, Uuid};
use nabu::XffValue;
use nabu::serde::write_legacy;

#[test]
fn generate_v3_examples() {
    let base_path = "xff-example-data/";

    // 1. Simple Primitive Values
    let primitives = vec![
        XffValue::Null,
        XffValue::from(true),
        XffValue::from(false),
        XffValue::NaN,
        XffValue::Infinity,
        XffValue::NegInfinity,
    ];
    for (i, val) in primitives.into_iter().enumerate() {
        let name = match i {
            0 => "v3_null",
            1 => "v3_boolean_t",
            2 => "v3_boolean_f",
            3 => "v3_nan",
            4 => "v3_infinity",
            5 => "v3_neginfinity",
            _ => unreachable!(),
        };
        write_legacy(format!("{}{}", base_path, name), val, 3).expect("Failed to write primitive");
    }

    // 2. Complex Types
    write_legacy(
        format!("{}v3_string", base_path),
        XffValue::from("Hello XFF v3! 🦀"),
        3,
    )
    .unwrap();
    write_legacy(
        format!("{}v3_number_u", base_path),
        XffValue::from(123456789usize),
        3,
    )
    .unwrap();
    write_legacy(
        format!("{}v3_number_i", base_path),
        XffValue::from(-123456789isize),
        3,
    )
    .unwrap();
    write_legacy(
        format!("{}v3_number_f", base_path),
        XffValue::from(3.1415926535f64),
        3,
    )
    .unwrap();
    write_legacy(
        format!("{}v3_data", base_path),
        XffValue::Data(Data::from(vec![0xDE, 0xAD, 0xBE, 0xEF])),
        3,
    )
    .unwrap();
    write_legacy(
        format!("{}v3_datetime", base_path),
        XffValue::from_unix_timestamp_millis(1700000000000),
        3,
    )
    .unwrap();
    write_legacy(
        format!("{}v3_duration", base_path),
        XffValue::from_duration_millis(3600000),
        3,
    )
    .unwrap();
    write_legacy(
        format!("{}v3_uuid", base_path),
        XffValue::Uuid(Uuid::new([0xAA; 16])),
        3,
    )
    .unwrap();

    // 3. Parent Types
    // Array
    let array = XffValue::from(vec![
        XffValue::from("nested"),
        XffValue::from(42),
        XffValue::from(true),
        XffValue::from(vec![XffValue::from(1.1), XffValue::from(2.2)]),
    ]);
    write_legacy(format!("{}v3_array_nested", base_path), array, 3).unwrap();

    // Object
    let mut obj = Object::new();
    obj.insert("project", "Nabu");
    obj.insert("version", 3);
    obj.insert("status", "development");
    obj.insert("tags", vec!["binary", "rust", "integrity"]);
    write_legacy(format!("{}v3_object", base_path), XffValue::Object(obj), 3).unwrap();

    // Table (The flagship v3 feature)
    let mut table = Table::with_columns(vec![
        "id".to_string(),
        "username".to_string(),
        "active".to_string(),
        "score".to_string(),
    ]);
    table
        .add_row(vec![
            XffValue::from(1),
            XffValue::from("Xqhare"),
            XffValue::from(true),
            XffValue::from(9001),
        ])
        .unwrap();
    table
        .add_row(vec![
            XffValue::from(2),
            XffValue::from("Gemini"),
            XffValue::from(true),
            XffValue::from(8888),
        ])
        .unwrap();
    table
        .add_row(vec![
            XffValue::from(3),
            XffValue::from("User"),
            XffValue::from(false),
            XffValue::from(100),
        ])
        .unwrap();
    write_legacy(format!("{}v3_table", base_path), XffValue::Table(table), 3).unwrap();

    // 4. Complex Nested Structure (similar to complex_read_and_write_v0)
    let mut root = Object::new();
    let mut meta = Object::new();
    meta.insert("author", "Xqhare");
    meta.insert("spec", "v3");
    root.insert("metadata", XffValue::Object(meta));

    let mut content = Object::new();
    content.insert("title", "XFF v3 Example File");
    content.insert(
        "body",
        "This file demonstrates all XFF v3 features in one structure.",
    );

    let mut data_section = Object::new();
    data_section.insert(
        "binary_blob",
        XffValue::Data(Data::from(vec![0x00, 0xFF, 0xAA, 0x55])),
    );
    data_section.insert(
        "special_nums",
        vec![XffValue::NaN, XffValue::Infinity, XffValue::NegInfinity],
    );

    content.insert("data", XffValue::Object(data_section));
    root.insert("content", XffValue::Object(content));

    write_legacy(
        format!("{}v3_complex_complete", base_path),
        XffValue::Object(root),
        3,
    )
    .unwrap();
}
