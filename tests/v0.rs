#[cfg(test)]
mod tests {
    use std::fs;

    use tyche::prelude::*;

    use nabu::serde::{self};
    use nabu::xff::value::{CommandCharacter, Data, Number, XffValue};

    #[test]
    fn v0_create_simulated_data() {
        if false {
            let mut data: Vec<XffValue> = Default::default();
            let mut gen_len = 100_000;
            while gen_len > 0 {
                println!("gen_len: {}", gen_len);
                let seed = random_from_range(1, 4).unwrap();
                match seed {
                    1 => data.push(make_random_string()),
                    2 => data.push(make_random_number()),
                    3 => data.push(make_random_command_character()),
                    4 => data.push(make_random_data()),
                    _ => unreachable!(),
                }
                gen_len -= 1;
            }
            let write = serde::write("tests/v0_simulated_data_22ignore.xff", data);
            assert!(write.is_ok());
        }

        // 843MB file
        //let path = "xff-example-data/v0_simulated_data_843MB_v0_ignore.xff";
        // 290MB file
        //let path = "xff-example-data/v0_simulated_data_290MB_v0.xff";
        // 290MB 66.666byte max per file
        //let path = "xff-example-data/v0_simulated_data_large_data_66_666byte_max_per_v0_ignore.xff";
        // 145MB file
        //let path = "xff-example-data/v0_simulated_data_145MB_v0.xff";
        // 21MB file
        //let path = "xff-example-data/v0_simulated_data_21MB_v0.xff";
        // 1.5MB file
        let path = "xff-example-data/v0_simulated_data_1MB_v0.xff";
        let read = serde::read(path);
        if read.is_err() {
            println!("Failed to read {:?}", read);
        }
        assert!(read.is_ok());
    }

    fn make_random_command_character() -> XffValue {
        let data = vec![
            CommandCharacter::Null,
            CommandCharacter::StartOfHeading,
            CommandCharacter::StartOfText,
            CommandCharacter::EndOfText,
            CommandCharacter::EndOfTransmission,
            CommandCharacter::Enquiry,
            CommandCharacter::Acknowledge,
            CommandCharacter::Bell,
            CommandCharacter::Backspace,
            CommandCharacter::HorizontalTab,
            CommandCharacter::LineFeed,
            CommandCharacter::VerticalTab,
            CommandCharacter::FormFeed,
            CommandCharacter::CarriageReturn,
            CommandCharacter::ShiftOut,
            CommandCharacter::ShiftIn,
            CommandCharacter::DataLinkEscape,
            CommandCharacter::DeviceControl1,
            CommandCharacter::DeviceControl2,
            CommandCharacter::DeviceControl3,
            CommandCharacter::DeviceControl4,
            CommandCharacter::NegativeAcknowledge,
            CommandCharacter::SynchronousIdle,
            CommandCharacter::EndOfTransmitBlock,
            CommandCharacter::Cancel,
            CommandCharacter::EndOfMedium,
            CommandCharacter::Substitute,
            CommandCharacter::Escape,
            CommandCharacter::FileSeparator,
            CommandCharacter::GroupSeparator,
            CommandCharacter::RecordSeparator,
            CommandCharacter::UnitSeparator,
            CommandCharacter::Space,
            CommandCharacter::Delete,
            CommandCharacter::NonBreakingSpace,
            CommandCharacter::SoftHyphen,
        ];
        XffValue::CommandCharacter(data[random_index(data.len()).unwrap()].clone())
    }

    fn make_random_data() -> XffValue {
        let seed = random_from_range(1, 66666).unwrap();
        let mut out: Vec<u8> = Default::default();
        for _ in 0..seed {
            out.push(random_u8().unwrap());
        }
        XffValue::from(out)
    }

    fn make_random_number() -> XffValue {
        match random_from_range(0, 2).unwrap() {
            0 => {
                // negative
                let seed = random_from_range(1, 3524654654).unwrap();
                let bind = format!("-{}", seed);
                XffValue::from(bind)
            }
            1 => {
                // positive
                let seed = random_from_range(1, 3524654654).unwrap();
                XffValue::from(seed)
            }
            2 => {
                // float
                let seed1 = random_from_range(1, 352465).unwrap();
                let seed2 = random_from_range(1, 564253).unwrap();
                if random_from_range(0, 1).unwrap() == 0 {
                    // negative
                    XffValue::from(-(seed1 as f64 / seed2 as f64))
                } else {
                    // positive
                    XffValue::from(seed1 as f64 / seed2 as f64)
                }
            }
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

    #[test]
    fn v0_serializer_deserializer_bare_bones() {
        let path = "tests/v0.txt";
        let path_2 = "tests/v0.xff";
        let data = { vec![XffValue::String("hello mom".to_string())] };
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
                XffValue::Data(Data {
                    len: bin_data0.len(),
                    data: bin_data0,
                }),
                XffValue::Number(Number::from(42.22222E-222)),
                XffValue::CommandCharacter(CommandCharacter::UnitSeparator),
                XffValue::CommandCharacter(CommandCharacter::Space),
                XffValue::CommandCharacter(CommandCharacter::Delete),
                XffValue::CommandCharacter(CommandCharacter::NonBreakingSpace),
                XffValue::CommandCharacter(CommandCharacter::SoftHyphen),
                XffValue::Number(Number::from(-42.22222E222)),
                XffValue::String("Padding".to_string()),
                XffValue::Data(Data {
                    len: bin_data1.len(),
                    data: bin_data1,
                }),
                XffValue::String("Padding".to_string()),
                XffValue::Number(Number::from(-42.22222E-222)),
                XffValue::Number(Number::from(42.22222E+222)),
                XffValue::String("Padding".to_string()),
                XffValue::Data(Data {
                    len: bin_data2.len(),
                    data: bin_data2,
                }),
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

    #[test]
    fn v0_complex_data() {
        let path = std::path::Path::new("tests/v0_complex_data.xff");
        let bin_data0 = std::fs::read("pictures/xff_v0-char-chart.jpeg").unwrap();
        let bin_data1 = std::fs::read("pictures/xff_v0-cmd-char-chart.jpeg").unwrap();
        let bin_data2 = std::fs::read("pictures/xff_v0-main-chart.jpeg").unwrap();
        let bin_data3 = std::fs::read("LICENSE").unwrap();
        let bin_data5 = std::fs::read("README.md").unwrap();
        let bin_data6 = std::fs::read("tests/v0.rs").unwrap();
        let data = {
            vec![
                XffValue::String("Padding".to_string()),
                XffValue::Data(Data {
                    len: bin_data0.len(),
                    data: bin_data0,
                }),
                XffValue::Number(Number::from(42.22222E-222)),
                XffValue::CommandCharacter(CommandCharacter::UnitSeparator),
                XffValue::Data(Data {
                    len: bin_data6.len(),
                    data: bin_data6,
                }),
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
