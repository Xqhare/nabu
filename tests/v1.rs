#[cfg(test)]
mod v1 {
    use std::collections::BTreeMap;
    use std::usize;

    use tyche::prelude::*;

    use nabu::serde::{self};
    use nabu::xff::value::{XffValue, Data};

    #[test]
    fn primitive_values() {
        let string = XffValue::from("hello mom");
        let u_num = XffValue::from(u8::MAX);
        let i_num = XffValue::from(i8::MIN);
        let f_num = XffValue::from(f32::from(42.69));
        let t_bool = XffValue::from(true);
        let f_bool = XffValue::from(false);
        let non = XffValue::Null;
        let lorem_short = XffValue::from("Lorem ipsum dolor sit amet, qui minim labore adipisicing minim sint cillum sint consectetur cupidatat.");
        let lorem_long = XffValue::from("Lorem ipsum dolor sit amet, officia excepteur ex fugiat reprehenderit enim labore culpa sint ad nisi Lorem pariatur mollit ex esse exercitation amet. Nisi anim cupidatat excepteur officia. Reprehenderit nostrud nostrud ipsum Lorem est aliquip amet voluptate voluptate dolor minim nulla est proident. Nostrud officia pariatur ut officia. Sit irure elit esse ea nulla sunt ex occaecat reprehenderit commodo officia dolor Lorem duis laboris cupidatat officia voluptate. Culpa proident adipisicing id nulla nisi laboris ex in Lorem sunt duis officia eiusmod. Aliqua reprehenderit commodo ex non excepteur duis sunt velit enim. Voluptate laboris sint cupidatat ullamco ut ea consectetur et est culpa et culpa duis.");
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
        let write = serde::write("xff-example-data/v1_primitive_values.xff", xff_val.clone());
        assert!(write.is_ok());

        let read = serde::read("xff-example-data/v1_primitive_values.xff");
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
        let write = serde::write("xff-example-data/v1_escape_chars.xff", xff_val.clone());
        assert!(write.is_ok());

        let read = serde::read("xff-example-data/v1_escape_chars.xff");
        assert!(read.is_ok());
        let read = read.unwrap().clone();
        assert!(read.is_array());
        assert_eq!(read, xff_val);
    }

    #[test]
    fn data() {
        let small_data = XffValue::from(make_random_data_with_length(100));
        let medium_data = XffValue::from(make_random_data_with_length(10_000));
        let large_data = XffValue::from(make_random_data_with_length(1_000_000));

        let xff_val = XffValue::from(vec![small_data, medium_data, large_data]);
        assert!(xff_val.is_array());
        let write = serde::write("xff-example-data/v1_data.xff", xff_val.clone());
        assert!(write.is_ok());

        let read = serde::read("xff-example-data/v1_data.xff");
        assert!(read.is_ok());
        let read = read.unwrap().clone();
        assert!(read.is_array());
        assert_eq!(read, xff_val);
    }

    #[test]
    fn object() {
        let map_small = XffValue::from(BTreeMap::from([
            ("key0", XffValue::from("value0")),
            ("key1", XffValue::from(-42)),
            ("key2", XffValue::from(Data::from(vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9]))),
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
            ("key8", XffValue::from(vec![XffValue::from("hello"), XffValue::from(42.69)])),
            ("key9", XffValue::Null),
            ("key10", XffValue::from(54642.69)),
            ("key11", XffValue::from("lorem ipsum")),
            ("key12", XffValue::from("Lorem ipsum dolor sit amet, qui minim labore adipisicing minim sint cillum sint consectetur cupidatat.")),
            ("key13", XffValue::from("Lorem ipsum dolor sit amet, officia excepteur ex fugiat reprehenderit enim labore culpa sint ad nisi Lorem pariatur mollit ex esse exercitation amet. Nisi anim cupidatat excepteur officia. Reprehenderit nostrud nostrud ipsum Lorem est aliquip amet voluptate voluptate dolor minim nulla est proident. Nostrud officia pariatur ut officia. Sit irure elit esse ea nulla sunt ex occaecat reprehenderit commodo officia dolor Lorem duis laboris cupidatat officia voluptate. Culpa proident adipisicing id nulla nisi laboris ex in Lorem sunt duis officia eiusmod. Aliqua reprehenderit commodo ex non excepteur duis sunt velit enim. Voluptate laboris sint cupidatat ullamco ut ea consectetur et est culpa et culpa duis.")),
            ("key14", XffValue::from("Lorem ipsum dolor sit amet, consectetur adipisicing elit, sed do eiusmod tempor incididunt ut labore.")),
            ("key15", XffValue::from(true)),
            ("key16", XffValue::from(false)),
            ("key17", XffValue::from(vec![XffValue::from("hello"), XffValue::from(42.69)])),
            ("key18", XffValue::from(BTreeMap::from([("key", XffValue::from(42.69))])),),
            ("key19", XffValue::from(42.69)),
        ]));
        let xff_val = XffValue::from(BTreeMap::from([
            ("key0", map_small),
            ("key1", map_medium),
        ]));
        assert!(xff_val.is_object());
        let write = serde::write("xff-example-data/v1_object.xff", xff_val.clone());
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

        let write_string = serde::write("xff-example-data/v1_string.xff", xff_string.clone());
        let write_number_f = serde::write("xff-example-data/v1_number_f.xff", xff_number_f.clone());
        let write_number_i = serde::write("xff-example-data/v1_number_i.xff", xff_number_i.clone());
        let write_number_u = serde::write("xff-example-data/v1_number_u.xff", xff_number_u.clone());
        let write_boolean_t = serde::write("xff-example-data/v1_boolean_t.xff", xff_boolean_t.clone());
        let write_boolean_f = serde::write("xff-example-data/v1_boolean_f.xff", xff_boolean_f.clone());
        let write_null = serde::write("xff-example-data/v1_null.xff", xff_null.clone());
        let write_data = serde::write("xff-example-data/v1_data.xff", xff_data.clone());
        let write_array = serde::write("xff-example-data/v1_array.xff", xff_array.clone());
        let write_object = serde::write("xff-example-data/v1_object.xff", xff_object.clone());

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

        let read_string = serde::read("xff-example-data/v1_string.xff");
        let read_number_f = serde::read("xff-example-data/v1_number_f.xff");
        let read_number_i = serde::read("xff-example-data/v1_number_i.xff");
        let read_number_u = serde::read("xff-example-data/v1_number_u.xff");
        let read_boolean_t = serde::read("xff-example-data/v1_boolean_t.xff");
        let read_boolean_f = serde::read("xff-example-data/v1_boolean_f.xff");
        let read_null = serde::read("xff-example-data/v1_null.xff");
        let read_data = serde::read("xff-example-data/v1_data.xff");
        let read_array = serde::read("xff-example-data/v1_array.xff");
        let read_object = serde::read("xff-example-data/v1_object.xff");

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
            XffValue::from(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20]),
            XffValue::from(false),
            XffValue::Null,
        ];

        let write = serde::write("xff-example-data/v1_complete_array.xff", XffValue::from(values.clone()));
        assert!(write.is_ok());

        let read = serde::read("xff-example-data/v1_complete_array.xff");
        if read.is_err() {
            println!("Failed to read {}", read.err().unwrap());
        } else {
            for (v1, v2) in values.iter().zip(read.unwrap().into_array().unwrap().iter()) {
                println!("v1: {:?} v2: {:?}", v1, v2);
                assert_eq!(v1, v2);
            }
        }
       
    }

    #[test]
    fn create_simulated_data() {
        if false {
            let mut data: Vec<XffValue> = Vec::new();
            let mut gen_len = 9000;
            while gen_len > 0 {
                println!("gen_len: {}", gen_len);
                data.push(make_random_value(7));
                gen_len -= 1;
            }
            let write = serde::write(
                "tests/v1_simulated_data_40-ignore.xff",
                XffValue::from(data),
            );
            assert!(write.is_ok());
        }

        // 100MB file
        let path = "xff-example-data/v1_simulated_data_100MB_ignore.xff";
        // 1MB file
        //let path = "xff-example-data/v1_simulated_data_1MB.xff";
        let read = serde::read(path);
        if read.is_err() {
            println!("Failed to read {}", read.err().unwrap());
        } else {
            assert!(read.is_ok());
            println!("read len: {:?}", read.unwrap().into_array().unwrap().len());
        }
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
        for n in 0..seed {
            //println!("object k-v pair: {}", n);
            out.insert(random_string().unwrap(), make_random_value(5));
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
        for n in 0..seed {
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
        for n in 0..seed {
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
                out.push(random_latin_char().unwrap().to_uppercase().next().unwrap());
            } else {
                out.push(random_latin_char().unwrap().to_lowercase().next().unwrap());
            }
        }
        XffValue::String(out)
    }
}
