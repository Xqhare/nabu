#[cfg(test)]
mod v1 {
    use std::collections::BTreeMap;
    use std::fs;

    use tyche::prelude::*;

    use nabu::serde::{self};
    use nabu::xff::value::{Array, XffValue};

    #[test]
    fn create_simulated_data() {
        if true {
            let mut data: Array = Array::new();
            let mut gen_len = 9;
            while gen_len > 0 {
                println!("gen_len: {}", gen_len);
                data.push(make_random_value(7));
                gen_len -= 1;
            }
            let write = serde::write(
                "tests/v1_simulated_data_31-ignore.xff",
                XffValue::from(data),
            );
            assert!(write.is_ok());
        }

        // 15MB file
        //let path = "xff-example-data/v1_simulated_data_15MB_ignore.xff";
        // 1MB file
        //let path = "xff-example-data/v1_simulated_data_1MB.xff";
        // 90KB file
        //let path = "xff-example-data/v1_simulated_data_90KB.xff";
        let path = "tests/v1_simulated_data_31-ignore.xff";
        let read = serde::read(path);
        if read.is_err() {
            println!("Failed to read {:?}", read);
        }
        assert!(read.is_ok());
        println!("read len: {:?}", read.unwrap().len());
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
