#[cfg(test)]
mod tests {
    use nabu::serde::{self};
    use nabu::xff::value::{Number, XffValue};

    #[test]
    fn test_v0_serializer_deserializer() {
        let path = std::path::Path::new("tests/v0.txt");
        let path_2 = std::path::Path::new("tests/v0.xff");
        let data = {
            vec![
                XffValue::String("hello".to_string()),
                XffValue::String("world".to_string()),
                XffValue::Number(Number::Unsigned(42)),
                XffValue::Number(Number::Integer(-42)),
                XffValue::Number(Number::Float(42.2)),
            ]
        };
        let tmp = serde::write(path, data.clone());
        assert!(tmp.is_ok());
        let tmp_2 = serde::read(path_2);
        assert!(tmp_2.is_ok());
        let ok = tmp_2.unwrap();
        println!("{:?}", ok);
        assert_eq!(ok[0], data[0]);
        assert_eq!(ok[1], data[1]);
        assert_eq!(ok[2], data[2]);
        assert_eq!(ok[3], data[3]);
        assert_eq!(ok[4], data[4]);

        println!("{:?}", ok);
    }

}
