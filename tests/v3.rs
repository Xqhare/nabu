use athena::{CommandCharacter, Object, Table, Uuid};
use nabu::XffValue;
use nabu::serde::{read, remove_file, write_legacy};

#[test]
fn test_v3_command_characters() {
    let path = "test_cmd.xff";
    let cmd = CommandCharacter::Bell; // 7
    let val = XffValue::CommandCharacter(cmd);

    write_legacy(path, val, 3).unwrap();
    let read_val = read(path).unwrap();

    // Deserializer reads it back as Data(7)
    assert!(read_val.is_data());
    assert_eq!(read_val.into_data().unwrap().data, vec![7]);

    // ArrayCmdChar
    let ac = vec![CommandCharacter::Backspace, CommandCharacter::LineFeed]; // 8, 10
    let val_ac = XffValue::ArrayCmdChar(ac);

    write_legacy(path, val_ac, 3).unwrap();
    let read_val_ac = read(path).unwrap();

    // Deserializer reads it back as Array [Data(8), Data(10)]
    assert!(read_val_ac.is_array());
    let array = read_val_ac.into_array().unwrap();
    assert_eq!(array.len(), 2);
    assert_eq!(array[0].as_data().unwrap().data, vec![8]);
    assert_eq!(array[1].as_data().unwrap().data, vec![10]);

    remove_file(path).unwrap();
}

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
