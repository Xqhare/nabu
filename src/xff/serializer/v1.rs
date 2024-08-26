use crate::{error::{NabuError, Result}, xff::value::XffValue};

pub fn serialize_xff_v1(data: Vec<XffValue>) -> Result<Vec<u8>> {
    let mut out: Vec<u8> = Default::default();
    // Version 1
    out.push(1);
    // only one value is permissable
    out.extend(serialize_xff_v1_value(&data[0])?);
    // EM
    out.push(25);
    Ok(out)
}

fn serialize_xff_v1_value(data: &XffValue) -> Result<Vec<u8>> {
    let mut out: Vec<u8> = Default::default();
    match data {
        XffValue::String(s) => {
            // first create the string
            let tmp_str: Vec<u8> = s.chars().map(|c| c as u8).collect();
            // now byte structure and push
            out.push(1);
            out.extend(encode_length(tmp_str.len()));
            out.extend(tmp_str);
            out.push(24);
        }
        XffValue::Number(n) => {
            // first create the string from the number
            let tmp_num: Vec<u8> = n.as_string().chars().map(|c| c as u8).collect();
            // now byte structure and push
            out.push(2);
            out.extend(encode_length(tmp_num.len()));
            out.extend(tmp_num);
            out.push(24);
        }
        XffValue::Array(a) => {
            // create the array
            let mut array_bytes: Vec<u8> = Default::default();
            println!("{:?}", a.values);
            for value in &a.values {
                let mut val = serialize_xff_v1_value(value)?;
                val.push(30);
                array_bytes.extend(val);
                // RS separator
                //array_bytes.push(30);
            }
            // byte structure and push
            out.push(3);
            out.extend(encode_length(array_bytes.len()));
            out.extend(array_bytes);
            out.push(24);
            println!("{:?}", out);
        }
        XffValue::Object(o) => {
            // create the object
            let mut object_bytes: Vec<u8> = Default::default();
            for (key, value) in o.map.iter() {
                // GS
                object_bytes.push(29);
                // key
                object_bytes.extend(serialize_xff_v1_value(&XffValue::String(key.clone()))?);
                // US
                object_bytes.push(31);
                // value
                out.extend(serialize_xff_v1_value(value)?);
                // Trailing GS
                object_bytes.push(29);
                // RS separator
                object_bytes.push(30);
            }
            // byte structure and push
            out.push(4);
            out.extend(encode_length(object_bytes.len()));
            out.extend(object_bytes);
            out.push(24);
        }
        XffValue::Data(d) => {
            out.push(5);
            out.extend(encode_length(d.len));
            out.extend(d.data.clone());
            out.push(24);
        }
        XffValue::Boolean(b) => {
            if *b {
                out.push(16);
            } else {
                out.push(17);
            }
        }
        XffValue::Null => {
            out.push(0);
        }
        _ => Err(NabuError::InvalidXFFVersion(data.clone(), 1))?
    }
    Ok(out)
}

fn encode_length(len: usize) -> Vec<u8> {
    if len <= 255 {
        let mut out: Vec<u8> = u8::from(1).to_le_bytes().to_vec();
        out.push(len.to_le_bytes().to_vec()[0]);
        return out;
    } if len <= 65_535 {
        let mut out: Vec<u8> = u8::from(2).to_le_bytes().to_vec();
        out.extend(len.to_le_bytes().to_vec()[0..2].to_vec());
        return out;
    } if len <= 16_777_215 {
        let mut out: Vec<u8> = u8::from(3).to_le_bytes().to_vec();
        out.extend(len.to_le_bytes().to_vec()[0..3].to_vec());
        return out;
    } if len <= 4_294_967_295 {
        let mut out: Vec<u8> = u8::from(4).to_le_bytes().to_vec();
        out.extend(len.to_le_bytes().to_vec()[0..4].to_vec());
        return out;
    } if len <= 1_099_511_627_775 {
        let mut out: Vec<u8> = u8::from(5).to_le_bytes().to_vec();
        out.extend(len.to_le_bytes().to_vec()[0..5].to_vec());
        return out;
    } if len <= 281_474_976_710_655 {
        let mut out: Vec<u8> = u8::from(6).to_le_bytes().to_vec();
        out.extend(len.to_le_bytes().to_vec()[0..6].to_vec());
        return out;
    } else if len <= 72_057_594_037_927_35 {
        let mut out: Vec<u8> = u8::from(7).to_le_bytes().to_vec();
        out.extend(len.to_le_bytes().to_vec()[0..7].to_vec());
        return out;
    } else {
        let mut out: Vec<u8> = u8::from(8).to_le_bytes().to_vec();
        out.extend(len.to_le_bytes().to_vec());
        return out;
    }
}

