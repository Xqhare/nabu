use athena::{Object, Table, Uuid};
use nabu::XffValue;
use nabu::serde::{read, remove_file, write_legacy};

#[test]
fn test_v3_roundtrip_simple() {
    let path = "test_simple.xff";
    let values = vec![
        XffValue::Null,
        XffValue::Boolean(true),
        XffValue::Boolean(false),
        XffValue::NaN,
        XffValue::Infinity,
        XffValue::NegInfinity,
    ];

    for val in values {
        write_legacy(path, val.clone(), 3).unwrap();
        let read_val = read(path).unwrap();
        assert_eq!(val, read_val);
    }
    remove_file(path).unwrap();
}

#[test]
fn test_v3_roundtrip_complex() {
    let path = "test_complex.xff";
    let val = XffValue::String("Hello XFF v3! 🦀".to_string());
    write_legacy(path, val.clone(), 3).unwrap();
    let read_val = read(path).unwrap();
    assert_eq!(val, read_val);

    let val = XffValue::from(123456789usize);
    write_legacy(path, val.clone(), 3).unwrap();
    let read_val = read(path).unwrap();
    assert_eq!(val, read_val);

    let val = XffValue::from(-123456789isize);
    write_legacy(path, val.clone(), 3).unwrap();
    let read_val = read(path).unwrap();
    assert_eq!(val, read_val);

    let val = XffValue::from(3.1415926535f64);
    write_legacy(path, val.clone(), 3).unwrap();
    let read_val = read(path).unwrap();
    assert_eq!(val, read_val);

    let val = XffValue::from(vec![1u8, 2, 3, 4, 5]);
    write_legacy(path, val.clone(), 3).unwrap();
    let read_val = read(path).unwrap();
    assert_eq!(val, read_val);

    remove_file(path).unwrap();
}

#[test]
fn test_v3_roundtrip_specialized() {
    let path = "test_special.xff";

    let val = XffValue::DateTime(1700000000000);
    write_legacy(path, val.clone(), 3).unwrap();
    assert_eq!(val, read(path).unwrap());

    let val = XffValue::Duration(3600000);
    write_legacy(path, val.clone(), 3).unwrap();
    assert_eq!(val, read(path).unwrap());

    let val = XffValue::Uuid(Uuid::new([0x12; 16]));
    write_legacy(path, val.clone(), 3).unwrap();
    assert_eq!(val, read(path).unwrap());

    remove_file(path).unwrap();
}

#[test]
fn test_v3_roundtrip_parent() {
    let path = "test_parent.xff";

    // Array
    let val = XffValue::from(vec![
        XffValue::from("nested"),
        XffValue::from(42),
        XffValue::from(true),
    ]);
    write_legacy(path, val.clone(), 3).unwrap();
    assert_eq!(val, read(path).unwrap());

    // Object
    let mut obj = Object::new();
    obj.insert("key1", "value1");
    obj.insert("key2", 42);
    let val = XffValue::Object(obj);
    write_legacy(path, val.clone(), 3).unwrap();
    assert_eq!(val, read(path).unwrap());

    // Table
    let mut table = Table::with_columns(vec!["name".to_string(), "age".to_string()]);
    table
        .add_row(vec![XffValue::from("Alice"), XffValue::from(30)])
        .unwrap();
    table
        .add_row(vec![XffValue::from("Bob"), XffValue::from(25)])
        .unwrap();
    let val = XffValue::Table(table);
    write_legacy(path, val.clone(), 3).unwrap();
    assert_eq!(val, read(path).unwrap());

    remove_file(path).unwrap();
}
