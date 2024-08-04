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
        assert_eq!(read_wizard.logs.len(), 1);
        assert_eq!(read_wizard.logs[0].log_data.len(), 1);
        assert_eq!(read_wizard.logs[0].log_data[0].name, "name");
        assert_eq!(read_wizard.logs[0].log_data[0].value, XffValue::String("value".to_string()));
        assert_eq!(read_wizard.logs[0].log_data[0].optional_metadata.is_empty(), true);

        // clear the file
        std::fs::remove_file("xff-example-data/logging_test_name.xff").unwrap();
    }

    #[test]
    fn read_and_write_log() {
        let read = LoggingWizard::from_file("xff-example-data/logging_wizard.xff");
        println!("{:?}", read);
        assert!(read.is_ok());
        let mut wizard = read.unwrap();
        assert!(wizard.logs_len == 4);
        let data1 = wizard.get_log(0).unwrap();
        assert_eq!(data1.log_data.len(), 1);
    }
}

