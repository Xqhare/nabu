#[cfg(test)]
#[cfg(feature = "logging_wizard")]
mod logging_wizard {
    use nabu::{
        logging_wizard::{Log, LogData, LoggingWizard},
        xff::value::{self, CommandCharacter, Data, Number, XffValue},
    };
    use std::collections::BTreeMap;
    #[test]
    fn logging_test_name() {
        let mut wizard = LoggingWizard::new("tests/logging_test_name.xff");
        let mut log = Log::new();
        log.add_log_data(LogData::new(
            "name",
            XffValue::String("value".to_string()),
            None,
        ));
        wizard.add_log(log);
        let out = wizard.save();
        assert!(out.is_ok());
        let read_wizard =
            LoggingWizard::from_file("tests/logging_test_name.xff").unwrap();
        assert_eq!(read_wizard.logs[0].log_data.len(), 1);
        assert_eq!(read_wizard.logs[0].log_data[0].name, "name");
        assert_eq!(
            read_wizard.logs[0].log_data[0].value,
            XffValue::String("value".to_string())
        );
        assert_eq!(
            read_wizard.logs[0].log_data[0].optional_metadata.is_empty(),
            true
        );

        // clear the file
        std::fs::remove_file("tests/logging_test_name.xff").unwrap();
    }

    #[test]
    fn read_log() {
        let read = LoggingWizard::from_file("xff-example-data/read_and_write_logging_wizard.xff");
        assert!(read.is_ok());
        let mut wizard = read.unwrap();
        assert!(wizard.logs_len == 1);
        let data1 = wizard.get_log(0).unwrap();
        assert_eq!(data1.log_data.len(), 4);

        assert_eq!(data1.log_data[0].name, "Data_point_1");
        assert_eq!(data1.log_data[0].value, XffValue::from("value"));
        assert_eq!(data1.log_data[0].optional_metadata.is_empty(), true);

        assert_eq!(data1.log_data[1].name, "Data_point_2");
        assert_eq!(data1.log_data[1].value, XffValue::from(-42));
        assert_eq!(data1.log_data[1].optional_metadata.is_empty(), true);

        assert_eq!(data1.log_data[2].name, "Data_point_3");
        assert_eq!(
            data1.log_data[2].value,
            XffValue::from(CommandCharacter::LineFeed)
        );
        assert_eq!(data1.log_data[2].optional_metadata.is_empty(), true);

        assert_eq!(data1.log_data[3].name, "Data_point_4");
        assert_eq!(
            data1.log_data[3].value,
            XffValue::from(Data::from(vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9]))
        );
        assert_eq!(data1.log_data[3].optional_metadata.is_empty(), true);
    }

    #[test]
    fn complex_read_and_write() {
        let mut wiz = LoggingWizard::new("tests/complex_read_and_write.xff");

        let mut log = Log::new();
        log.add_log_data(LogData::new("Data_point_1", XffValue::from("value"), None));
        log.add_log_data(LogData::new("Data_point_2", XffValue::from(-42), None));
        log.add_log_data(LogData::new(
            "Data_point_3",
            XffValue::from(CommandCharacter::LineFeed),
            None,
        ));
        let mut meta1: BTreeMap<String, String> = BTreeMap::new();
        meta1.insert("key1".to_string(), "value1".to_string());
        meta1.insert("key2".to_string(), "sequential data".to_string());
        meta1.insert("key3".to_string(), "420".to_string());
        meta1.insert("key4".to_string(), "meta data".to_string());
        log.add_log_data(LogData::new(
            "Data_point_4".to_string(),
            XffValue::from(Data::from(vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9])),
            Some(meta1),
        ));
        wiz.add_log(log);
        wiz.save().unwrap();

        let read = LoggingWizard::from_file("tests/complex_read_and_write.xff").unwrap();
        assert_eq!(read.logs[0].log_data[0].name, "Data_point_1");
        assert_eq!(read.logs[0].log_data[0].value, XffValue::from("value"));
        assert_eq!(read.logs[0].log_data[3].name, "Data_point_4");
        assert_eq!(read.logs[0].log_data[3].optional_metadata["key1"], "value1");
        assert_eq!(
            read.logs[0].log_data[3].optional_metadata["key2"],
            "sequential data"
        );
        assert_eq!(read.logs[0].log_data[3].optional_metadata["key3"], "420");
        assert_eq!(
            read.logs[0].log_data[3].optional_metadata["key4"],
            "meta data"
        );
        assert_eq!(
            read.logs[0].log_data[3].value,
            XffValue::from(Data::from(vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9]))
        );

        //clear the file
        std::fs::remove_file("tests/complex_read_and_write.xff").unwrap();
    }

    #[test]
    fn log_create_simulated_data() {

        if false {
            let mut wiz = LoggingWizard::new("tests/logging_wizard_simulated_data_1MB.xff");
            // generates 1MB of data
            for i in 1..510 {
                let mut log = Log::new();
                for j in 1..6 {
                    let mut meta: BTreeMap<String, String> = BTreeMap::new();
                    for k in 0..2 {
                        meta.insert(format!("key_{}_{}", j, k), format!("value_{}_{}", j, k));
                    }
                    log.add_log_data(LogData::new(
                        format!("Data_point_{}", j),
                        XffValue::from(-i),
                        Some(meta),
                    ));
                }
                wiz.add_log(log);
            }

            for i in 510..702 {
                let mut log = Log::new();
                for j in 1..14 {
                    let mut meta: BTreeMap<String, String> = BTreeMap::new();
                    for k in 0..12 {
                        meta.insert(format!("key_{}_{}", j, k), format!("value_{}_{}", j, k));
                    }
                    // I enjoyed writing this, dont ask me why
                    let vect = vec![
                        i + 25 * j - j + i / 15 % 120,
                        j + 986 - 56 * i + 65 % j * 58,
                        i * 2,
                        j / 3,
                        i - 4,
                        j / 2 * 5 * 99 * 6 % 255,
                        i / 6,
                        j - 7,
                        i / 8 / 9 * 48 % 255,
                        j / 7,
                        i - 8,
                        j + 9,
                        j + i * 25 * 3 % 255,
                        j % 11,
                        i % 12,
                        j % 13,
                        i + i * j % 89,
                        j % 15,
                        i * 16,
                        j * 17,
                    ];
                    log.add_log_data(LogData::new(
                        format!("Data_point_{}", j),
                        XffValue::from(vect.iter().map(|x| *x as u8).collect::<Vec<u8>>()),
                        Some(meta),
                    ));
                }
                wiz.add_log(log);
            }

            let check = wiz.clone();

            wiz.save();
        }

        let read = LoggingWizard::from_file("xff-example-data/logging_wizard_simulated_data_1MB.xff");

        assert!(read.is_ok());
        // check every log in read against the one in check
        /* for i in 0..check.logs.len().saturating_sub(1) {
            assert_eq!(read.logs[i].log_data.len(), check.logs[i].log_data.len());
            for j in 0..check.logs[i].log_data.len().saturating_sub(1) {
                assert_eq!(
                    read.logs[i].log_data[j].name,
                    check.logs[i].log_data[j].name
                );
                assert_eq!(
                    read.logs[i].log_data[j].value,
                    check.logs[i].log_data[j].value
                );
                assert_eq!(
                    read.logs[i].log_data[j].optional_metadata,
                    check.logs[i].log_data[j].optional_metadata
                );
            }
        } */
    }
}
