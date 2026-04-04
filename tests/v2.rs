#[cfg(test)]
mod v2 {
    use std::collections::BTreeMap;
    use std::fs;
    use std::path::PathBuf;

    use serde::{read, remove_file, write_legacy};
    use tyche::prelude::*;

    use nabu::*;

    #[test]
    fn test_v2_command_characters() {
        let path = "xff-example-data/v2_test_cmd.xff";
        let cmd = CommandCharacter::Bell; // 7
        let val = XffValue::CommandCharacter(cmd);

        write_legacy(path, val, 2).unwrap();
        let read_val = read(path).unwrap();

        // Deserializer reads it back as Data(7)
        assert!(read_val.is_data());
        assert_eq!(read_val.into_data().unwrap().data, vec![7]);

        // ArrayCmdChar
        let ac = vec![CommandCharacter::Backspace, CommandCharacter::LineFeed]; // 8, 10
        let val_ac = XffValue::ArrayCmdChar(ac);

        write_legacy(path, val_ac, 2).unwrap();
        let read_val_ac = read(path).unwrap();

        // Deserializer reads it back as Array [Data(8), Data(10)]
        assert!(read_val_ac.is_array());
        let array = read_val_ac.into_array().unwrap();
        assert_eq!(array.len(), 2);
        assert_eq!(array[0].as_data().unwrap().data, vec![8]);
        assert_eq!(array[1].as_data().unwrap().data, vec![10]);

        let _ = std::fs::remove_file(path);
    }

    #[test]
    fn hello_world() {
        let path = "xff-example-data/v2-hello-world.xff";

        let mut object: Object = Object::new();

        object.insert("String", XffValue::from("Hi mom!"));
        object.insert("Number", XffValue::from(usize::MAX));
        object.insert("Number", XffValue::from(-42));
        object.insert("Number", XffValue::from(42.69));
        object.insert("Boolean", XffValue::from(true));
        object.insert("Null", XffValue::from(XffValue::Null));

        let mut array: Array = Array::new();
        array.push(XffValue::from("Hello mom!"));
        array.push(XffValue::from(usize::MAX));

        object.insert("Array", XffValue::from(array));

        object.insert(
            "Data",
            XffValue::from(Data::from(vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9])),
        );

        let value = XffValue::from(object);

        let write = write_legacy(path, value.clone(), 2);
        assert!(write.is_ok());
        let read = read(path);
        assert!(read.is_ok());
        let ok = read.unwrap();
        assert_eq!(ok, value);
        let remove = remove_file(path);
        assert!(remove.is_ok());
    }

    #[test]
    fn simple_object() {
        let path = "xff-example-data/v2_simple_object.xff";
        let mut object: Object = Object::new();
        object.insert("String", XffValue::from("Hello mom!"));
        object.insert("Number", XffValue::from(usize::MAX));
        object.insert("Number", XffValue::from(-42));
        object.insert("Number", XffValue::from(42.69));
        object.insert("Boolean", XffValue::from(true));
        object.insert("Null", XffValue::from(XffValue::Null));
        let mut array: Array = Array::new();
        array.push(XffValue::from("Hello mom!"));
        array.push(XffValue::from(usize::MAX));

        object.insert("Array", XffValue::from(array));

        object.insert(
            "Data",
            XffValue::from(Data::from(vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9])),
        );
        let value = XffValue::from(object.clone());
        let write = write_legacy(path, value.clone(), 2);
        assert!(write.is_ok());
        let read = read(path);
        assert!(read.is_ok());
        let ok = read.unwrap();
        assert_eq!(ok, value);
        let remove = remove_file(path);
        assert!(remove.is_ok());
    }

    #[test]
    fn nested_object() {
        let path = "xff-example-data/v2_nested_object.xff";
        let mut object: Object = Object::new();
        object.insert("String", XffValue::from("Hello mom!"));
        object.insert("Number", XffValue::from(usize::MAX));
        object.insert("Number", XffValue::from(-42));
        object.insert("Number", XffValue::from(42.69));
        object.insert("Boolean", XffValue::from(true));
        object.insert("Null", XffValue::from(XffValue::Null));
        let mut value = Object::new();
        value.insert("InnerObject", XffValue::from(object.clone()));
        object.insert("NestedObject", XffValue::from(value));
        let mut array: Array = Array::new();
        array.push(XffValue::from("Hello mom!"));
        array.push(XffValue::from(usize::MAX));

        object.insert("Array", XffValue::from(array));

        object.insert(
            "Data",
            XffValue::from(Data::from(vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9])),
        );
        let value = XffValue::from(object.clone());
        object.insert("Object", value.clone());
        let write = write_legacy(path, value.clone(), 2);
        assert!(write.is_ok());
        let read = read(path);
        assert!(read.is_ok());
        let ok = read.unwrap();
        assert_eq!(ok, value);
        let remove = remove_file(path);
        assert!(remove.is_ok());
    }

    #[test]
    fn simple_vector() {
        let val = XffValue::from(vec![
            XffValue::from("Hello mom!"),
            XffValue::from(-42),
            XffValue::from(42.69),
            XffValue::from(u16::MAX),
            XffValue::from(i16::MIN),
            XffValue::from(true),
            XffValue::from(false),
            XffValue::from(XffValue::Null),
            XffValue::from(Data::from(vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9])),
        ]);

        let write = write_legacy("xff-example-data/v2_simple_vector.xff", val.clone(), 2);
        assert!(write.is_ok());
        let read = read("xff-example-data/v2_simple_vector.xff");
        assert!(read.is_ok());
        let ok = read.unwrap();
        assert_eq!(ok, val);
        let remove = remove_file("xff-example-data/v2_simple_vector.xff");
        assert!(remove.is_ok());
    }

    #[test]
    fn nested_vector() {
        let val = XffValue::from(vec![
            XffValue::from("Hello mom!"),
            XffValue::from(-42),
            XffValue::from(42.69),
            XffValue::from(usize::MAX),
            XffValue::from(isize::MIN),
            XffValue::from(true),
            XffValue::from(false),
            XffValue::from(XffValue::Null),
            XffValue::from(Data::from(vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9])),
            XffValue::from(vec![
                XffValue::from("Hello mom!"),
                XffValue::from(-42),
                XffValue::from(42.69),
                XffValue::from(u64::MAX),
                XffValue::from(i64::MIN),
                XffValue::from(true),
                XffValue::from(false),
                XffValue::from(XffValue::Null),
                XffValue::from(Data::from(vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9])),
                XffValue::from(vec![
                    XffValue::from("Hello mom!"),
                    XffValue::from(-42),
                    XffValue::from(42.69),
                    XffValue::from(u32::MAX),
                    XffValue::from(i32::MIN),
                    XffValue::from(true),
                    XffValue::from(false),
                    XffValue::from(XffValue::Null),
                    XffValue::from(Data::from(vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9])),
                    XffValue::from(vec![
                        XffValue::from("Hello mom!"),
                        XffValue::from(-42),
                        XffValue::from(42.69),
                        XffValue::from(u32::MAX),
                        XffValue::from(i32::MIN),
                        XffValue::from(true),
                        XffValue::from(false),
                        XffValue::from(XffValue::Null),
                        XffValue::from(Data::from(vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9])),
                        XffValue::from(vec![
                            XffValue::from("Hello mom!"),
                            XffValue::from(-42),
                            XffValue::from(42.69),
                            XffValue::from(u32::MAX),
                            XffValue::from(i32::MIN),
                            XffValue::from(true),
                            XffValue::from(false),
                            XffValue::from(XffValue::Null),
                            XffValue::from(Data::from(vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9])),
                        ]),
                    ]),
                ]),
            ]),
        ]);

        let write = write_legacy("xff-example-data/v2_nested_vector.xff", val.clone(), 2);
        assert!(write.is_ok());
        let read = read("xff-example-data/v2_nested_vector.xff");
        assert!(read.is_ok());
        let ok = read.unwrap();
        assert_eq!(ok, val);
        let remove = remove_file("xff-example-data/v2_nested_vector.xff");
        assert!(remove.is_ok());
    }

    #[test]
    fn actual_data() {
        let path = "xff-example-data/v2_actual_data.xff";
        let real_data = XffValue::from(Data::from(fs::read("src/lib.rs").unwrap()));
        let real_data2 = XffValue::from(Data::from(fs::read("Cargo.toml").unwrap()));
        let real_data3 = XffValue::from(Data::from(fs::read("README.md").unwrap()));
        let data = XffValue::from(vec![
            real_data.clone(),
            real_data2.clone(),
            real_data3.clone(),
        ]);
        let write = serde::write_legacy(path, data, 2);
        assert!(write.is_ok());
        let read = serde::read(path);
        assert!(read.is_ok());
        let read = read.unwrap();
        assert!(read.is_array());
        let array = read.into_array().unwrap();
        assert_eq!(array.len(), 3);
        assert_eq!(array[0], real_data);
        assert_eq!(array[1], real_data2);
        assert_eq!(array[2], real_data3);
    }

    #[test]
    fn zero_length_data_stores() {
        let path = "xff-example-data/v2_empty.xff";
        let obj = Object::new();
        let data = XffValue::from(obj);
        let write = serde::write_legacy(path, data, 2);
        assert!(write.is_ok());
        let read = serde::read(path);
        assert!(read.is_ok());
        let read = read.unwrap();
        assert!(read.is_object());
        let obj = read.into_object().unwrap();
        assert_eq!(obj.len(), 0);

        let ary = Array::new();
        let data = XffValue::from(ary);
        let write = serde::write_legacy(path, data, 2);
        assert!(write.is_ok());
        let read = serde::read(path);
        assert!(read.is_ok());
        let read = read.unwrap();
        assert!(read.is_array());
        let ary = read.into_array().unwrap();
        assert_eq!(ary.len(), 0);

        //cleanup
        fs::remove_file(path).unwrap();
    }

    #[test]
    fn zero_length_string() {
        let path = "xff-example-data/v2_empty_string.xff";
        let data = XffValue::from("");
        let write = serde::write_legacy(path, data, 2);
        assert!(write.is_ok());
        let read = serde::read(path);
        assert!(read.is_ok());
        let read = read.unwrap();
        assert!(read.is_string());
        assert_eq!(read.into_string().unwrap(), "");
    }

    #[test]
    fn zero_length_data() {
        let path = "xff-example-data/v2_empty_data.xff";
        let data = XffValue::from(Data::from(vec![]));
        let write = serde::write_legacy(path, data, 2);
        assert!(write.is_ok());
        let read = serde::read(path);
        assert!(read.is_ok());
        let read = read.unwrap();
        assert!(read.is_data());
        assert_eq!(read.into_data().unwrap(), Data::from(vec![]));
    }

    #[test]
    fn read_write_loop_object() {
        let path = "xff-example-data/v2_loop_complex.xff";
        for n in 0..100 {
            if n == 0 {
                // create a new file
                let data = XffValue::from(BTreeMap::from([
                    (
                        "array".to_string(),
                        XffValue::from(Array::from(vec![XffValue::from(n)])),
                    ),
                    ("key0".to_string(), XffValue::from(42.69)),
                ]));
                let write = serde::write_legacy(path, data, 2);
                assert!(write.is_ok());
            } else {
                // read the file and append
                let read = serde::read(path);
                assert!(read.is_ok());
                let mut data = read.unwrap().into_object().unwrap();
                let mut ary = data.remove("array").unwrap().into_array().unwrap();
                ary.push(XffValue::from(n));
                data.insert("array".to_string(), XffValue::from(ary));
                data.insert(format!("key{}", n), XffValue::from(42.69));
                let write = serde::write_legacy(path, XffValue::from(data), 2);
                assert!(write.is_ok());
            }
        }

        // read the file and assert the result
        let read = serde::read(path);
        assert!(read.is_ok());
        let read = read.unwrap().into_object().unwrap();
        assert_eq!(read.len(), 101);
        assert_eq!(read["array"].into_array().unwrap().len(), 100);
        assert_eq!(read["key0"], XffValue::from(42.69));
        assert_eq!(read["key42"], XffValue::from(42.69));
        assert_eq!(read["key69"], XffValue::from(42.69));
        assert_eq!(read["key99"], XffValue::from(42.69));

        // remove the file
        std::fs::remove_file(path).unwrap();
    }

    #[test]
    fn read_write_loop_array() {
        let path = "xff-example-data/v2_loop.xff";
        for n in 0..100 {
            if n == 0 {
                // create a new file
                let data = XffValue::from(vec![XffValue::from(format!("Value {}", n))]);
                let write = serde::write_legacy(path, data, 2);
                assert!(write.is_ok());
            } else {
                // read the file and append
                let read = serde::read(path);
                assert!(read.is_ok());
                let mut data = read.unwrap().into_array().unwrap();
                data.push(XffValue::from(format!("Value {}", n)));
                let write = serde::write_legacy(path, XffValue::from(data), 2);
                assert!(write.is_ok());
            }
        }
        // read the file and assert the result
        let read = serde::read(path);
        assert!(read.is_ok());
        let read = read.unwrap().into_array().unwrap();
        assert_eq!(read.len(), 100);
        assert_eq!(read[0], XffValue::from("Value 0"));
        assert_eq!(read[42], XffValue::from("Value 42"));
        assert_eq!(read[69], XffValue::from("Value 69"));
        assert_eq!(read[99], XffValue::from("Value 99"));

        // clear the file
        std::fs::remove_file(path).unwrap();
    }

    #[test]
    fn primitive_values() {
        let string = XffValue::from("hello mom");
        let u_num = XffValue::from(u8::MAX);
        let i_num = XffValue::from(i8::MIN);
        let f_num = XffValue::from(f32::from(42.69));
        let t_bool = XffValue::from(true);
        let f_bool = XffValue::from(false);
        let non = XffValue::Null;
        let lorem_short = XffValue::from(
            "Lorem ipsum dolor sit amet, qui minim labore adipisicing minim sint cillum sint consectetur cupidatat.",
        );
        let lorem_long = XffValue::from(
            "Lorem ipsum dolor sit amet, officia excepteur ex fugiat reprehenderit enim labore culpa sint ad nisi Lorem pariatur mollit ex esse exercitation amet. Nisi anim cupidatat excepteur officia. Reprehenderit nostrud nostrud ipsum Lorem est aliquip amet voluptate voluptate dolor minim nulla est proident. Nostrud officia pariatur ut officia. Sit irure elit esse ea nulla sunt ex occaecat reprehenderit commodo officia dolor Lorem duis laboris cupidatat officia voluptate. Culpa proident adipisicing id nulla nisi laboris ex in Lorem sunt duis officia eiusmod. Aliqua reprehenderit commodo ex non excepteur duis sunt velit enim. Voluptate laboris sint cupidatat ullamco ut ea consectetur et est culpa et culpa duis.",
        );
        let xff_val = XffValue::from(vec![
            string,
            u_num,
            i_num,
            f_num,
            t_bool,
            f_bool,
            non,
            lorem_short,
            lorem_long,
        ]);
        assert!(xff_val.is_array());
        let write = serde::write_legacy(
            "xff-example-data/v2_primitive_values.xff",
            xff_val.clone(),
            2,
        );
        assert!(write.is_ok());

        let read = serde::read("xff-example-data/v2_primitive_values.xff");
        assert!(read.is_ok());
        let read = read.unwrap().clone();
        assert!(read.is_array());
        assert_eq!(read, xff_val);
    }

    #[test]
    fn escape_chars() {
        let str_with_backspace = XffValue::from("hello\x08mom");
        let str_with_horizontal_tab = XffValue::from("hello\x09mom");
        let str_with_line_feed = XffValue::from("hello\x0Amom");
        let str_with_vertical_tab = XffValue::from("hello\x0Bmom");
        let str_with_carriage_return = XffValue::from("hello\x0Cmom");
        let str_with_form_feed = XffValue::from("hello\x0Dmom");
        let str_with_backslash = XffValue::from("hello\\mom");
        let str_with_double_quote = XffValue::from("hello\"mom");
        let str_with_single_quote = XffValue::from("hello'mom");

        let xff_val = XffValue::from(vec![
            str_with_backspace,
            str_with_horizontal_tab,
            str_with_line_feed,
            str_with_vertical_tab,
            str_with_carriage_return,
            str_with_form_feed,
            str_with_backslash,
            str_with_double_quote,
            str_with_single_quote,
        ]);
        assert!(xff_val.is_array());
        let write = serde::write_legacy("xff-example-data/v2_escape_chars.xff", xff_val.clone(), 2);
        assert!(write.is_ok());

        let read = serde::read("xff-example-data/v2_escape_chars.xff");
        assert!(read.is_ok());
        let read = read.unwrap().clone();
        assert!(read.is_array());
        assert_eq!(read, xff_val);
    }

    #[test]
    fn data() {
        let small_data = XffValue::from(Data::from(make_random_data_with_length(100)));
        let medium_data = XffValue::from(Data::from(make_random_data_with_length(10_000)));
        let large_data = XffValue::from(Data::from(make_random_data_with_length(1_000_000)));

        let xff_val = XffValue::from(vec![small_data, medium_data, large_data]);
        assert!(xff_val.is_array());
        let path: PathBuf = PathBuf::from("xff-example-data/v2_data_complex.xff");
        if !path.exists() {
            let write = serde::write_legacy(&path, xff_val.clone(), 2);
            assert!(write.is_ok());
        }

        let read = serde::read(&path);
        assert!(read.is_ok());
        let read = read.unwrap().clone();
        assert!(read.is_array());
        assert_eq!(read, xff_val);

        // cleanup
        let _ = fs::remove_file(path);
    }

    #[test]
    fn object() {
        let map_small = XffValue::from(BTreeMap::from([
            ("key0", XffValue::from("value0")),
            ("key1", XffValue::from(-42)),
            (
                "key2",
                XffValue::from(Data::from(vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9])),
            ),
        ]));
        let map_medium = XffValue::from(BTreeMap::from([
            ("key0", XffValue::from("value0")),
            ("key1", XffValue::from(-42)),
            ("key2", XffValue::from(-420.69)),
            (
                "key3",
                XffValue::from(Data::from(vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9])),
            ),
            (
                "key4",
                XffValue::from(BTreeMap::from([("key", XffValue::from(42.69))])),
            ),
            ("key5", XffValue::from(42.69)),
            ("key6", XffValue::from(true)),
            ("key7", XffValue::from(false)),
            (
                "key8",
                XffValue::from(vec![XffValue::from("hello"), XffValue::from(42.69)]),
            ),
            ("key9", XffValue::Null),
            ("key10", XffValue::from(54642.69)),
            ("key11", XffValue::from("lorem ipsum")),
            (
                "key12",
                XffValue::from(
                    "Lorem ipsum dolor sit amet, qui minim labore adipisicing minim sint cillum sint consectetur cupidatat.",
                ),
            ),
            (
                "key13",
                XffValue::from(
                    "Lorem ipsum dolor sit amet, officia excepteur ex fugiat reprehenderit enim labore culpa sint ad nisi Lorem pariatur mollit ex esse exercitation amet. Nisi anim cupidatat excepteur officia. Reprehenderit nostrud nostrud ipsum Lorem est aliquip amet voluptate voluptate dolor minim nulla est proident. Nostrud officia pariatur ut officia. Sit irure elit esse ea nulla sunt ex occaecat reprehenderit commodo officia dolor Lorem duis laboris cupidatat officia voluptate. Culpa proident adipisicing id nulla nisi laboris ex in Lorem sunt duis officia eiusmod. Aliqua reprehenderit commodo ex non excepteur duis sunt velit enim. Voluptate laboris sint cupidatat ullamco ut ea consectetur et est culpa et culpa duis.",
                ),
            ),
            (
                "key14",
                XffValue::from(
                    "Lorem ipsum dolor sit amet, consectetur adipisicing elit, sed do eiusmod tempor incididunt ut labore.",
                ),
            ),
            ("key15", XffValue::from(true)),
            ("key16", XffValue::from(false)),
            (
                "key17",
                XffValue::from(vec![XffValue::from("hello"), XffValue::from(42.69)]),
            ),
            (
                "key18",
                XffValue::from(BTreeMap::from([("key", XffValue::from(42.69))])),
            ),
            ("key19", XffValue::from(42.69)),
        ]));
        let xff_val = XffValue::from(BTreeMap::from([("key0", map_small), ("key1", map_medium)]));
        assert!(xff_val.is_object());
        let write = serde::write_legacy("xff-example-data/v2_object.xff", xff_val.clone(), 2);
        assert!(write.is_ok());
    }

    #[test]
    fn singeltons() {
        let xff_string = XffValue::from("hello mom");
        let xff_number_f = XffValue::from(42.69);
        let xff_number_i = XffValue::from(-42);
        let xff_number_u = XffValue::from(usize::MAX);
        let xff_boolean_t = XffValue::from(true);
        let xff_boolean_f = XffValue::from(false);
        let xff_null = XffValue::Null;
        let xff_data = XffValue::from(Data::from(vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9]));
        let xff_array = XffValue::from(vec![XffValue::from("hello"), XffValue::from(42.69)]);
        let xff_object = XffValue::from(BTreeMap::from([("key", XffValue::from(42.69))]));

        let write_string =
            serde::write_legacy("xff-example-data/v2_string.xff", xff_string.clone(), 2);
        let write_number_f =
            serde::write_legacy("xff-example-data/v2_number_f.xff", xff_number_f.clone(), 2);
        let write_number_i =
            serde::write_legacy("xff-example-data/v2_number_i.xff", xff_number_i.clone(), 2);
        let write_number_u =
            serde::write_legacy("xff-example-data/v2_number_u.xff", xff_number_u.clone(), 2);
        let write_boolean_t = serde::write_legacy(
            "xff-example-data/v2_boolean_t.xff",
            xff_boolean_t.clone(),
            2,
        );
        let write_boolean_f = serde::write_legacy(
            "xff-example-data/v2_boolean_f.xff",
            xff_boolean_f.clone(),
            2,
        );
        let write_null = serde::write_legacy("xff-example-data/v2_null.xff", xff_null.clone(), 2);
        let write_data = serde::write_legacy(
            "xff-example-data/v2_singleton_data.xff",
            xff_data.clone(),
            2,
        );
        let write_array =
            serde::write_legacy("xff-example-data/v2_array.xff", xff_array.clone(), 2);
        let write_object =
            serde::write_legacy("xff-example-data/v2_object.xff", xff_object.clone(), 2);

        assert!(write_string.is_ok());
        assert!(write_number_f.is_ok());
        assert!(write_number_i.is_ok());
        assert!(write_number_u.is_ok());
        assert!(write_boolean_t.is_ok());
        assert!(write_boolean_f.is_ok());
        assert!(write_null.is_ok());
        assert!(write_data.is_ok());
        assert!(write_array.is_ok());
        assert!(write_object.is_ok());

        let read_string = serde::read("xff-example-data/v2_string.xff");
        let read_number_f = serde::read("xff-example-data/v2_number_f.xff");
        let read_number_i = serde::read("xff-example-data/v2_number_i.xff");
        let read_number_u = serde::read("xff-example-data/v2_number_u.xff");
        let read_boolean_t = serde::read("xff-example-data/v2_boolean_t.xff");
        let read_boolean_f = serde::read("xff-example-data/v2_boolean_f.xff");
        let read_null = serde::read("xff-example-data/v2_null.xff");
        let read_data = serde::read("xff-example-data/v2_singleton_data.xff");
        let read_array = serde::read("xff-example-data/v2_array.xff");
        let read_object = serde::read("xff-example-data/v2_object.xff");

        assert!(read_string.is_ok());
        assert!(read_number_f.is_ok());
        assert!(read_number_i.is_ok());
        assert!(read_number_u.is_ok());
        assert!(read_boolean_t.is_ok());
        assert!(read_boolean_f.is_ok());
        assert!(read_null.is_ok());
        assert!(read_data.is_ok());
        assert!(read_array.is_ok());
        assert!(read_object.is_ok());

        // compare values

        assert_eq!(read_string.unwrap(), xff_string);
        assert_eq!(read_number_f.unwrap(), xff_number_f);
        assert_eq!(read_number_i.unwrap(), xff_number_i);
        assert_eq!(read_number_u.unwrap(), xff_number_u);
        assert_eq!(read_boolean_t.unwrap(), xff_boolean_t);
        assert_eq!(read_boolean_f.unwrap(), xff_boolean_f);
        assert_eq!(read_null.unwrap(), xff_null);
        assert_eq!(read_data.unwrap(), xff_data);
        assert_eq!(read_array.unwrap(), xff_array);
        assert_eq!(read_object.unwrap(), xff_object);
    }

    #[test]
    fn complete_array() {
        let tmp: Vec<u8> = vec![
            1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20,
        ];
        let values = vec![
            XffValue::from("hello"),
            XffValue::from(42.69),
            XffValue::from(true),
            XffValue::from(vec![XffValue::from("hello"), XffValue::from(42.69)]),
            XffValue::from(BTreeMap::from([("key", XffValue::from(42.69))])),
            XffValue::from(u8::MAX),
            XffValue::from(u16::MAX),
            XffValue::from(u32::MAX),
            XffValue::from(u64::MAX),
            XffValue::from(i8::MIN),
            XffValue::from(i16::MIN),
            XffValue::from(i32::MIN),
            XffValue::from(i64::MIN),
            XffValue::from(f32::from(42.69)),
            XffValue::from(f64::from(69.42)),
            XffValue::from("hello".to_string()),
            XffValue::from(tmp),
            XffValue::from(false),
            XffValue::Null,
        ];

        let write = serde::write_legacy(
            "xff-example-data/v2_complete_array.xff",
            XffValue::from(values.clone()),
            2,
        );
        assert!(write.is_ok());

        let read = serde::read("xff-example-data/v2_complete_array.xff");
        for (v1, v2) in values
            .iter()
            .zip(read.unwrap().into_array().unwrap().iter())
        {
            assert_eq!(v1, v2);
        }
    }

    #[test]
    fn read_sim_data() {
        let paths = vec![
            "xff-example-data/v2_simulated_data_1.xff",
            "xff-example-data/v2_simulated_data_4_small.xff",
            "xff-example-data/v2_simulated_data_4_large.xff",
            "xff-example-data/v2_simulated_data_5.xff",
            "xff-example-data/v2_simulated_data_10.xff",
        ];
        for path in paths {
            let read = serde::read(path);
            assert!(read.is_ok());
        }
    }

    #[test]
    fn create_simulated_data() {
        if false {
            let mut data: Vec<XffValue> = Vec::new();
            let mut gen_len = 4;
            while gen_len > 0 {
                //println!("gen_len: {}", gen_len);
                data.push(make_random_value(7));
                gen_len -= 1;
            }
            let write = serde::write_legacy(
                "tests/v2_simulated_data_4_2-ignore.xff",
                XffValue::from(data),
                2,
            );
            assert!(write.is_ok());
        }

        // 100MB file
        //let path = "tests/v2_simulated_data_100-ignore.xff";

        let path = "xff-example-data/v2_simulated_data_10.xff";
        let read = serde::read(path);
        assert!(read.is_ok());
    }

    fn make_random_value(end: usize) -> XffValue {
        let seed = random_from_range(1, end).unwrap();
        match seed {
            1 => make_random_string(),
            2 => make_random_number(),
            3 => make_random_boolean(),
            4 => XffValue::Null,
            5 => make_random_data(),
            6 => make_random_array(),
            7 => make_random_object(),
            _ => unreachable!(),
        }
    }

    fn make_random_object() -> XffValue {
        let mut out = BTreeMap::new();
        let seed = random_from_range(1, 1_000).unwrap();
        for _n in 0..seed {
            //println!("object k-v pair: {}", n);
            out.insert(
                make_random_string().into_string().unwrap(),
                make_random_value(5),
            );
        }
        //println!("obj made");
        XffValue::from(out)
    }

    fn make_random_boolean() -> XffValue {
        random_bool().unwrap().into()
    }

    fn make_random_array() -> XffValue {
        let seed = random_from_range(1, 1_000).unwrap();
        let mut out: Vec<XffValue> = Default::default();
        for _n in 0..seed {
            //println!("array element: {}", n);
            out.push(make_random_value(5));
        }
        //println!("arr made");
        XffValue::from(out)
    }

    fn make_random_data_with_length(len: usize) -> Vec<u8> {
        let mut out: Vec<u8> = Default::default();
        for _ in 0..len {
            out.push(random_u8().unwrap());
        }
        out
    }

    fn make_random_data() -> XffValue {
        let seed = random_from_range(1, 1_000).unwrap();
        let mut out: Vec<u8> = Default::default();
        for _n in 0..seed {
            //println!("data element: {}", n);
            out.push(random_u8().unwrap());
        }
        //println!("data made");
        XffValue::from(out)
    }

    fn make_random_number() -> XffValue {
        match random_from_range(0, 4).unwrap() {
            0 => {
                // negative
                let seed = random_from_range(1, 3524654654).unwrap();
                let bind = format!("-{}", seed);
                XffValue::from(bind.parse::<i64>().unwrap())
            }
            1 => {
                // positive
                let seed = random_from_range(1, 3524654654).unwrap();
                XffValue::from(seed)
            }
            2 => {
                // float
                let seed1 = random_from_range(1, 3524654).unwrap();
                let seed2 = random_from_range(1, 4564253).unwrap();
                if random_from_range(0, 1).unwrap() == 0 {
                    // negative
                    XffValue::from(-(seed1 as f64 / seed2 as f64))
                } else {
                    // positive
                    XffValue::from(seed1 as f64 / seed2 as f64)
                }
            }
            3 => XffValue::from(0),
            4 => XffValue::from(0.0),
            _ => unreachable!(),
        }
    }

    fn make_random_string() -> XffValue {
        let seed = random_from_range(1, 255).unwrap();
        let mut out: String = Default::default();
        for n in 0..seed {
            if n == 0 {
                out.push(random_latin_char(true).unwrap());
            } else {
                out.push(random_latin_char(false).unwrap());
            }
        }
        XffValue::from(out)
    }
}
