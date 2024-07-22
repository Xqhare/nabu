#[cfg(test)]
mod tests {
    use std::fs;

    use nabu::serde::{self};
    use nabu::xff::value::{CommandCharacter, Data, Number, XffValue};

    #[test]
    fn v0_serializer_deserializer_bare_bones() {
        let path = std::path::Path::new("tests/v0.txt");
        let path_2 = std::path::Path::new("tests/v0.xff");
        let data = {
            vec![
                XffValue::String("hello mom".to_string()),
            ]
        };
        let tmp = serde::write(path, data.clone());
        assert!(tmp.is_ok());
        let tmp_2 = serde::read(path_2);
        assert!(tmp_2.is_ok());
        let ok = tmp_2.unwrap();
        assert_eq!(ok[0], data[0]);
        // delete file
        fs::remove_file(path_2).unwrap();
    }

    #[test]
    fn v0_serializer_deserializer_basic() {
        let path = std::path::Path::new("tests/v0_basic.xff");
        let data = {
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
            ]
        };
        let tmp = serde::write(path, data.clone());
        assert!(tmp.is_ok());
        let tmp_2 = serde::read(path);
        assert!(tmp_2.is_ok());
        let ok = tmp_2.unwrap();
        for n in 0..ok.len() {
            //println!("loop: {}", n);
            //println!("ok: {:?}", ok[n]);
            //println!("data: {:?}", data[n]);
            assert_eq!(ok[n], data[n]);
        }
        // delete file
        fs::remove_file(path).unwrap();
    }

    #[test]
    fn v0_string_to_number() {
        let path = std::path::Path::new("tests/v0_string_to_number.xff");
        let data = {
            vec![
                XffValue::String("42.22222E2".to_string()),
                XffValue::String("42.22222E+2".to_string()),
                XffValue::String("42.22222E-2".to_string()),
                XffValue::String("-42.22222E2".to_string()),
                XffValue::String("-42.22222E-2".to_string()),
                XffValue::String("-42.22222E+2".to_string()),

                XffValue::String("42.22222e2".to_string()),
                XffValue::String("42.22222e+2".to_string()),
                XffValue::String("42.22222e-2".to_string()),
                XffValue::String("-42.22222e2".to_string()),
                XffValue::String("-42.22222e-2".to_string()),
                XffValue::String("-42.22222e+2".to_string()),

                XffValue::String("42.22222E222".to_string()),
                XffValue::String("42.22222E+222".to_string()),
                XffValue::String("42.22222E-222".to_string()),
                XffValue::String("-42.22222E222".to_string()),
                XffValue::String("-42.22222E-222".to_string()),
                XffValue::String("-42.22222E+222".to_string()),
            ]
        };
        let checked_data = {
            vec![
                XffValue::Number(Number::from(42.22222E2)),
                XffValue::Number(Number::from(42.22222E+2)),
                XffValue::Number(Number::from(42.22222E-2)),
                XffValue::Number(Number::from(-42.22222E2)),
                XffValue::Number(Number::from(-42.22222E-2)),
                XffValue::Number(Number::from(-42.22222E+2)),

                XffValue::Number(Number::from(42.22222e2)),
                XffValue::Number(Number::from(42.22222e+2)),
                XffValue::Number(Number::from(42.22222e-2)),
                XffValue::Number(Number::from(-42.22222e2)),
                XffValue::Number(Number::from(-42.22222e-2)),
                XffValue::Number(Number::from(-42.22222e+2)),

                XffValue::Number(Number::from(42.22222E222)),
                XffValue::Number(Number::from(42.22222E+222)),
                XffValue::Number(Number::from(42.22222E-222)),
                XffValue::Number(Number::from(-42.22222E222)),
                XffValue::Number(Number::from(-42.22222E-222)),
                XffValue::Number(Number::from(-42.22222E+222)),
            ]
        };
        let tmp = serde::write(path, data.clone());
        assert!(tmp.is_ok());
        let tmp_2 = serde::read(path);
        assert!(tmp_2.is_ok());
        let ok = tmp_2.unwrap();
        for n in 0..ok.len() {
            //println!("loop: {}", n);
            //println!("ok: {:?}", ok[n]);
            //println!("data: {:?}", data[n]);
            assert_eq!(ok[n], checked_data[n]);
        }
        // delete file
        fs::remove_file(path).unwrap();
    }

    #[test]
    fn v0_serializer_deserializer_cmd_chars() {
        let path = std::path::Path::new("tests/v0_cmd_chars.xff");
        let data = {
            vec![
                // literally all the command characters
                XffValue::CommandCharacter(CommandCharacter::Null),
                XffValue::CommandCharacter(CommandCharacter::StartOfHeading),
                XffValue::CommandCharacter(CommandCharacter::StartOfText),
                XffValue::CommandCharacter(CommandCharacter::EndOfText),
                XffValue::CommandCharacter(CommandCharacter::EndOfTransmission),
                XffValue::CommandCharacter(CommandCharacter::Enquiry),
                XffValue::CommandCharacter(CommandCharacter::Acknowledge),
                XffValue::CommandCharacter(CommandCharacter::Bell),
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
                XffValue::CommandCharacter(CommandCharacter::GroupSeparator),
                XffValue::CommandCharacter(CommandCharacter::RecordSeparator),
                XffValue::CommandCharacter(CommandCharacter::UnitSeparator),
                XffValue::CommandCharacter(CommandCharacter::Space),
                XffValue::CommandCharacter(CommandCharacter::Delete),
                XffValue::CommandCharacter(CommandCharacter::NonBreakingSpace),
                XffValue::CommandCharacter(CommandCharacter::SoftHyphen),
            ]
        };
        let tmp = serde::write(path, data.clone());
        assert!(tmp.is_ok());
        let tmp_2 = serde::read(path);
        assert!(tmp_2.is_ok());
        let ok = tmp_2.unwrap();
        for n in 0..ok.len() {
            //println!("loop: {}", n);
            //println!("ok: {:?}", ok[n]);
            //println!("data: {:?}", data[n]);
            assert_eq!(ok[n], data[n]);
        }
        // delete file
        fs::remove_file(path).unwrap();
    }

    #[test]
    fn v0_data() {
        let path = std::path::Path::new("tests/v0_data.xff");
        let bin_data0 = std::fs::read("src/lib.rs").unwrap();
        let bin_data1 = std::fs::read("README.md").unwrap();
        let bin_data2 = std::fs::read("tests/v0.rs").unwrap();
        let data = {
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
        let tmp = serde::write(path, data.clone());
        assert!(tmp.is_ok());
        let tmp_2 = serde::read(path);
        assert!(tmp_2.is_ok());
        let ok = tmp_2.unwrap();
        for n in 0..ok.len() {
            //println!("loop: {}", n);
            //println!("ok: {:?}", ok[n]);
            //println!("data: {:?}", data[n]);
            assert_eq!(ok[n], data[n]);
        }
        // delete file
        fs::remove_file(path).unwrap();
    }
}

