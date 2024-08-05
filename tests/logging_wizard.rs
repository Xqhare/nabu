#[cfg(test)]
#[cfg(feature = "logging_wizard")]
mod logging_wizard {
    use nabu::{logging_wizard::{LoggingWizard, Log, LogData}, xff::value::{CommandCharacter, Data, Number, XffValue}};
    #[test]
    fn logging_test_name() {
        let mut wizard = LoggingWizard::new("xff-example-data/logging_test_name.xff");
        let mut log = Log::new();
        log.add_log_data(LogData::new("name", XffValue::String("value".to_string()), None));
        wizard.add_log(log);
        let out = wizard.save();
        assert!(out.is_ok());
        let read_wizard = LoggingWizard::from_file("xff-example-data/logging_test_name.xff").unwrap();
        assert_eq!(read_wizard.logs[0].log_data.len(), 1);
        assert_eq!(read_wizard.logs[0].log_data[0].name, "name");
        assert_eq!(read_wizard.logs[0].log_data[0].value, XffValue::String("value".to_string()));
        assert_eq!(read_wizard.logs[0].log_data[0].optional_metadata.is_empty(), true);

        // clear the file
        std::fs::remove_file("xff-example-data/logging_test_name.xff").unwrap();
    }

    #[test]
    fn read_and_write_log() {
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
        assert_eq!(data1.log_data[2].value, XffValue::from(CommandCharacter::LineFeed));
        assert_eq!(data1.log_data[2].optional_metadata.is_empty(), true);

        assert_eq!(data1.log_data[3].name, "Data_point_4");
        assert_eq!(data1.log_data[3].value, XffValue::from(Data::from(vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9])));
        assert_eq!(data1.log_data[3].optional_metadata.is_empty(), true);
    }
}

