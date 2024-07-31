#[cfg(test)]
#[cfg(feature = "logging_wizard")]
mod tests {
    use std::path::Path;
    use nabu::{logging_wizard::{LoggingWizard, Log, LogData}, xff::value::{CommandCharacter, Data, Number, XffValue}};
    #[test]
    fn test_name() {
        let mut wizard = LoggingWizard::new(Path::new("test_name.xff"));
        let mut log = Log::new();
        log.add_log_data(LogData::new("name", XffValue::String("value".to_string()), None));
        wizard.add_log(log);
        let out = wizard.save();
        assert!(out.is_ok());
        let read_wizard = LoggingWizard::from_file(Path::new("test_name.xff")).unwrap();
        assert_eq!(read_wizard.logs.len(), 1);
        assert_eq!(read_wizard.logs[0].log_data.len(), 1);
        assert_eq!(read_wizard.logs[0].log_data[0].name, "name");
        assert_eq!(read_wizard.logs[0].log_data[0].value, XffValue::String("value".to_string()));
        assert_eq!(read_wizard.logs[0].log_data[0].optional_metadata.is_empty(), true);
    }
}



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
        let data_store = vec![
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
            ];
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

#[cfg(test)]
#[cfg(feature = "key_value_store")]
mod store {
    use nabu::{key_value_store::new_nabudb, xff::value::{CommandCharacter, Data, Number, XffValue}, features::key_value::store::NabuDB};

    #[test]
    fn basic() {
        let mut db: NabuDB = new_nabudb("basic_db.xff").unwrap();
        let data = vec![
            XffValue::String("Padding".to_string()),
            XffValue::Data(Data {len: 10, data: vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9]}),
            XffValue::CommandCharacter(CommandCharacter::UnitSeparator),
            XffValue::Number(Number::from(42.22222E+222)),
            XffValue::CommandCharacter(CommandCharacter::SoftHyphen),
            XffValue::Number(Number::from(-42.22222E222)),
            XffValue::String("Lorem ipsum dolor sit amet, qui minim labore adipisicing minim sint cillum sint consectetur cupidatat.".to_string())
        ];
        for (index, entry) in data.iter().enumerate() {
            db.insert(format!("key{}", index), entry.clone());
        }
        let _ = db.save();
        let read = new_nabudb("basic_db.xff").unwrap();
        assert_eq!(data.len(), read.len());
        for (n, entry) in data.iter().enumerate() {
            assert_eq!(read.get(&format!("key{}", n)).unwrap(), entry);
        }

        // cleanup
        let _ = std::fs::remove_file("basic_db.xff");
    }

    #[test]
    fn advanced() {
        let mut db: NabuDB = new_nabudb("advanced_db.xff").unwrap();
        let bin_data0 = std::fs::read("src/lib.rs").unwrap();
        let bin_data1 = std::fs::read("README.md").unwrap();
        let bin_data2 = std::fs::read("tests/v0.rs").unwrap();
        let data = vec![
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
            ];
        for (index, entry) in data.iter().enumerate() {
            db.insert(format!("key{}", index), entry.clone());
        }
        let _ = db.save();
        let read = new_nabudb("advanced_db.xff").unwrap();
        assert_eq!(data.len(), read.len());
        for (n, entry) in data.iter().enumerate() {
            assert_eq!(read.get(&format!("key{}", n)).unwrap(), entry);
        }

        // cleanup
        let _ = std::fs::remove_file("advanced_db.xff");
    }

    #[test]
    fn complex() {
        let mut db: NabuDB = new_nabudb("complex_db.xff").unwrap();
        let bin_data0 = std::fs::read("pictures/xff-char-chart.jpeg").unwrap();
        let bin_data1 = std::fs::read("pictures/xff-cmd-char-chart.jpeg").unwrap();
        let bin_data2 = std::fs::read("pictures/xff-main-chart.jpeg").unwrap();
        let bin_data3 = std::fs::read("LICENSE").unwrap();
        let bin_data5 = std::fs::read("README.md").unwrap();
        let bin_data6 = std::fs::read("tests/v0.rs").unwrap();
        let data = vec![
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
            ];
        for (index, entry) in data.iter().enumerate() {
            db.insert(format!("key{}", index), entry.clone());
        }
        let _ = db.save();
        let read = new_nabudb("complex_db.xff").unwrap();
        assert_eq!(data.len(), read.len());
        for (n, entry) in data.iter().enumerate() {
            assert_eq!(read.get(&format!("key{}", n)).unwrap(), entry);
        }

        // cleanup
        let _ = std::fs::remove_file("complex_db.xff");
    }
}
