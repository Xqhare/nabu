#[cfg(test)]
#[cfg(feature = "key_value_core")]
mod core {
    use std::path::Path;
    use nabu::{key_value_core::{new_core_store, read, write}, xff::value::{CommandCharacter, Data, Number, XffValue}};

    #[test]
    fn basic() {
        let path = Path::new("test_basic_kv_store.xff");
        let mut data = new_core_store();
        data.insert("key0".to_string(), XffValue::String("value".to_string()));
        data.insert("key1".to_string(), XffValue::String("value2".to_string()));
        data.insert("key2".to_string(), XffValue::String("value3".to_string()));
        data.insert("key3".to_string(), XffValue::String("value4".to_string()));
        data.insert("key4".to_string(), XffValue::String("value5".to_string()));

        write(path, data.clone()).unwrap();
        let read = read(path).unwrap();
        assert_eq!(data.len(), read.len());
        for (n, entry) in read.iter().enumerate() {
            assert_eq!(data.get(&format!("key{}", n)).unwrap(), entry.1);
        }

        // cleanup
        let _ = std::fs::remove_file(path);
    }

    #[test]
    fn basic_all_ascii() {
        let path = Path::new("test_basic_ascii_kv_store.xff");
        let mut data = new_core_store();
        let data_store = {
            vec![
                XffValue::String("hello mom".to_string()),
                XffValue::Number(Number::from(u8::from(42))),
                XffValue::Number(Number::from(i8::from(-42))),
                XffValue::Number(Number::from(u16::MAX)),
                XffValue::Number(Number::from(i16::MIN)),
                XffValue::Number(Number::from(u32::MAX)),
                XffValue::Number(Number::from(i32::MIN)),
                XffValue::Number(Number::from(u64::MAX)),
                XffValue::Number(Number::from(i64::MIN)),
                XffValue::Number(Number::from(42.2)),
                XffValue::Number(Number::from(-42.2)),
                XffValue::Number(Number::from(42.22222e2)),
                XffValue::Number(Number::from(42.22222e-2)),
                XffValue::Number(Number::from(-42.22222e2)),
                XffValue::Number(Number::from(-42.22222e-2)),

                XffValue::Number(Number::from(42.22222E2)),
                XffValue::Number(Number::from(42.22222E+2)),
                XffValue::Number(Number::from(42.22222E-2)),
                XffValue::Number(Number::from(-42.22222E2)),
                XffValue::Number(Number::from(-42.22222E-2)),
                XffValue::Number(Number::from(-42.22222E+2)),

                XffValue::CommandCharacter(CommandCharacter::Backspace),
                XffValue::CommandCharacter(CommandCharacter::HorizontalTab),
                XffValue::CommandCharacter(CommandCharacter::LineFeed),
                XffValue::CommandCharacter(CommandCharacter::VerticalTab),
                XffValue::CommandCharacter(CommandCharacter::FormFeed),
                XffValue::CommandCharacter(CommandCharacter::CarriageReturn),
                XffValue::CommandCharacter(CommandCharacter::ShiftOut),
                XffValue::CommandCharacter(CommandCharacter::ShiftIn),
                XffValue::CommandCharacter(CommandCharacter::DataLinkEscape),
                XffValue::CommandCharacter(CommandCharacter::DeviceControl1),
                XffValue::CommandCharacter(CommandCharacter::DeviceControl2),
                XffValue::CommandCharacter(CommandCharacter::DeviceControl3),
                XffValue::CommandCharacter(CommandCharacter::DeviceControl4),
                XffValue::CommandCharacter(CommandCharacter::NegativeAcknowledge),
                XffValue::CommandCharacter(CommandCharacter::SynchronousIdle),
                XffValue::CommandCharacter(CommandCharacter::EndOfTransmitBlock),
                XffValue::CommandCharacter(CommandCharacter::Cancel),
                XffValue::CommandCharacter(CommandCharacter::EndOfMedium),
                XffValue::CommandCharacter(CommandCharacter::Substitute),
                XffValue::CommandCharacter(CommandCharacter::Escape),
                XffValue::CommandCharacter(CommandCharacter::FileSeparator),
            ]
        };
        for (index, entry) in data_store.iter().enumerate() {
            data.insert(format!("key{}", index), entry.clone());
        }
        write(path, data.clone()).unwrap();
        let read = read(path).unwrap();
        assert_eq!(data.len(), read.len());
        for (n, entry) in data_store.iter().enumerate() {
            assert_eq!(read.get(&format!("key{}", n)).unwrap(), entry);
        }

        // cleanup
        let _ = std::fs::remove_file(path);
    }

    #[test]
    fn advanced() {
        let path = Path::new("test_advanced_kv_store.xff");
        let mut data = new_core_store();
        let bin_data0 = std::fs::read("src/lib.rs").unwrap();
        let bin_data1 = std::fs::read("README.md").unwrap();
        let bin_data2 = std::fs::read("tests/v0.rs").unwrap();
        let data_store = {
            vec![
                XffValue::String("Padding".to_string()),
                XffValue::Data(Data {len: bin_data0.len(), data: bin_data0}),
                XffValue::Number(Number::from(42.22222E-222)),
                XffValue::CommandCharacter(CommandCharacter::UnitSeparator),
                XffValue::CommandCharacter(CommandCharacter::Space),
                XffValue::CommandCharacter(CommandCharacter::Delete),
                XffValue::CommandCharacter(CommandCharacter::NonBreakingSpace),
                XffValue::CommandCharacter(CommandCharacter::SoftHyphen),
                XffValue::Number(Number::from(-42.22222E222)),
                XffValue::String("Padding".to_string()),
                XffValue::Data(Data {len: bin_data1.len(), data: bin_data1}),
                XffValue::String("Padding".to_string()),
                XffValue::Number(Number::from(-42.22222E-222)),
                XffValue::Number(Number::from(42.22222E+222)),
                XffValue::String("Padding".to_string()),
                XffValue::Data(Data {len: bin_data2.len(), data: bin_data2}),
                XffValue::String("Padding".to_string()),
                XffValue::Number(Number::from(42.22222E-222)),
                XffValue::Number(Number::from(-42.22222E+222)),
                XffValue::String("Padding".to_string()),
                XffValue::CommandCharacter(CommandCharacter::LineFeed),
                XffValue::CommandCharacter(CommandCharacter::VerticalTab),
                XffValue::CommandCharacter(CommandCharacter::FormFeed),
                XffValue::CommandCharacter(CommandCharacter::CarriageReturn),
                XffValue::CommandCharacter(CommandCharacter::ShiftOut),
                XffValue::CommandCharacter(CommandCharacter::ShiftIn),
                XffValue::CommandCharacter(CommandCharacter::DataLinkEscape),
                XffValue::CommandCharacter(CommandCharacter::DeviceControl1),
                XffValue::CommandCharacter(CommandCharacter::DeviceControl2),
                XffValue::CommandCharacter(CommandCharacter::DeviceControl3),
                XffValue::CommandCharacter(CommandCharacter::DeviceControl4),
                XffValue::String("Padding".to_string()),
                XffValue::CommandCharacter(CommandCharacter::Escape),
            ]
        };
        for (index, entry) in data_store.iter().enumerate() {
            data.insert(format!("key{}", index), entry.clone());
        }
        write(path, data.clone()).unwrap();
        let read = read(path).unwrap();
        assert_eq!(data.len(), read.len());
        for (n, entry) in data_store.iter().enumerate() {
            assert_eq!(read.get(&format!("key{}", n)).unwrap(), entry);
        }

        // cleanup
        let _ = std::fs::remove_file(path);
    }

    #[test]
    fn complex() {
        let path = Path::new("test_complex_kv_store.xff");
        let mut data = new_core_store();
        let bin_data0 = std::fs::read("pictures/xff-char-chart.jpeg").unwrap();
        let bin_data1 = std::fs::read("pictures/xff-cmd-char-chart.jpeg").unwrap();
        let bin_data2 = std::fs::read("pictures/xff-main-chart.jpeg").unwrap();
        let bin_data3 = std::fs::read("LICENSE").unwrap();
        let bin_data5 = std::fs::read("README.md").unwrap();
        let bin_data6 = std::fs::read("tests/v0.rs").unwrap();
        let data_store = {
            vec![
                XffValue::String("Padding".to_string()),
                XffValue::Data(Data {len: bin_data0.len(), data: bin_data0}),
                XffValue::Number(Number::from(42.22222E-222)),
                XffValue::CommandCharacter(CommandCharacter::UnitSeparator),
                XffValue::Data(Data {len: bin_data6.len(), data: bin_data6}),
                XffValue::CommandCharacter(CommandCharacter::SoftHyphen),
                XffValue::Number(Number::from(-42.22222E222)),
                XffValue::String("Padding".to_string()),
                XffValue::Data(Data::from(bin_data1)),
                XffValue::String("Padding".to_string()),
                XffValue::Number(Number::from(42.22222E+222)),
                XffValue::String("Padding".to_string()),
                XffValue::Data(Data::from(bin_data2)),
                XffValue::String("Padding".to_string()),
                XffValue::Number(Number::from(42.22222E-222)),
                XffValue::String("Padding".to_string()),
                XffValue::CommandCharacter(CommandCharacter::LineFeed),
                XffValue::CommandCharacter(CommandCharacter::DeviceControl4),
                XffValue::String("Padding".to_string()),
                XffValue::CommandCharacter(CommandCharacter::Escape),
                XffValue::Data(Data::from(bin_data3)),
                XffValue::Data(Data::from(bin_data5)),
            ]
        };
        for (index, entry) in data_store.iter().enumerate() {
            data.insert(format!("key{}", index), entry.clone());
        }
        write(path, data.clone()).unwrap();
        let read = read(path).unwrap();
        assert_eq!(data.len(), read.len());
        for (n, entry) in data_store.iter().enumerate() {
            assert_eq!(read.get(&format!("key{}", n)).unwrap(), entry);
        }

        // cleanup
        let _ = std::fs::remove_file(path);
    }
}
