use crate::{
    Array, Data, XffValue,
    error::{NabuError, Result},
};

/// Serializes XFF version 1 data.
///
/// # Errors
/// Errors if the data contains non-ASCII characters or is invalid for v1.
pub fn serialize_xff_v1(data: &[XffValue]) -> Result<Vec<u8>> {
    let mut out: Vec<u8> = Vec::default();
    // Version 1
    out.push(1);
    // only one value is permissable
    out.extend(serialize_xff_v1_value(&data[0])?);
    // EM
    out.push(25);
    Ok(out)
}

#[allow(clippy::too_many_lines)]
fn serialize_xff_v1_value(data: &XffValue) -> Result<Vec<u8>> {
    let mut out: Vec<u8> = Vec::default();
    match data {
        XffValue::String(s) => {
            let s = s.as_str();
            let tmp_str: Vec<u8> = {
                let mut out = Vec::new();
                for char in s.chars() {
                    let tmp = char as u8;
                    if (8..=13).contains(&tmp)
                        || (32..=126).contains(&tmp)
                        || tmp == 128
                        || tmp == 142
                        || (130..=140).contains(&tmp)
                        || (145..=156).contains(&tmp)
                        || tmp >= 158
                    {
                        out.push(tmp);
                    } else {
                        return Err(NabuError::StringContainsNonASCII(s.to_string(), 1));
                    }
                }
                out
            };
            // now byte structure and push
            out.push(1);
            out.extend(encode_length(tmp_str.len()));
            out.extend(tmp_str);
            out.push(24);
        }
        XffValue::Number(n) => {
            // first create the string from the number
            let tmp_num: Vec<u8> = {
                let mut out = Vec::new();
                let mut sep_used = false;
                let mut neg_used = false;
                for char in n.as_string().chars() {
                    let tmp = char as u8;
                    if tmp == 45 {
                        if neg_used {
                            return Err(NabuError::NumberContainsInvalidCharacter(
                                tmp,
                                n.as_string(),
                                1,
                            ));
                        }
                        out.push(tmp);
                        neg_used = true;
                    } else if tmp == 44 || tmp == 46 {
                        if sep_used {
                            return Err(NabuError::NumberContainsInvalidCharacter(
                                tmp,
                                n.as_string(),
                                1,
                            ));
                        }
                        out.push(tmp);
                        sep_used = true;
                    } else if (48..=57).contains(&tmp) {
                        out.push(tmp);
                    } else {
                        return Err(NabuError::NumberContainsInvalidCharacter(
                            tmp,
                            n.as_string(),
                            1,
                        ));
                    }
                }
                out
            };
            // now byte structure and push
            out.push(2);
            out.extend(encode_length(tmp_num.len()));
            out.extend(tmp_num);
            out.push(24);
        }
        XffValue::Array(a) => {
            // create the array
            let mut array_bytes: Vec<u8> = Vec::default();
            for value in &a.values {
                array_bytes.extend(serialize_xff_v1_value(value)?);
                // RS separator
                array_bytes.push(30);
            }
            // byte structure and push
            out.push(3);
            out.extend(encode_length(array_bytes.len()));
            out.extend(array_bytes);
            out.push(24);
        }
        XffValue::Object(o) => {
            // create the object
            let mut object_bytes: Vec<u8> = Vec::default();
            for (key, value) in &o.map {
                // GS
                object_bytes.push(29);
                // key
                object_bytes.extend(serialize_xff_v1_value(&XffValue::from(key.as_str()))?);
                // US
                object_bytes.push(31);
                // value
                object_bytes.extend(serialize_xff_v1_value(value)?);
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
            if b.value() {
                out.push(16);
            } else {
                out.push(17);
            }
        }
        XffValue::Null => {
            out.push(0);
        }
        XffValue::CommandCharacter(c) => {
            out.extend(serialize_xff_v1_value(&XffValue::Data(Data::from(vec![
                c.as_u8(),
            ])))?);
        }
        XffValue::ArrayCmdChar(ac) => {
            let values: Vec<XffValue> = ac
                .iter()
                .map(|c| XffValue::Data(Data::from(vec![c.as_u8()])))
                .collect();
            out.extend(serialize_xff_v1_value(&XffValue::Array(Array::from(
                values,
            )))?);
        }
        _ => return Err(NabuError::InvalidXFFVersion(data.clone(), 1)),
    }
    Ok(out)
}

fn encode_length(len: usize) -> Vec<u8> {
    if len <= 255 {
        let mut out: Vec<u8> = vec![1];
        out.push(len.to_le_bytes()[0]);
        return out;
    }
    if len <= 65_535 {
        let mut out: Vec<u8> = vec![2];
        out.extend_from_slice(&len.to_le_bytes()[0..2]);
        return out;
    }
    if len <= 16_777_215 {
        let mut out: Vec<u8> = vec![3];
        out.extend_from_slice(&len.to_le_bytes()[0..3]);
        return out;
    }
    if len <= 4_294_967_295 {
        let mut out: Vec<u8> = vec![4];
        out.extend_from_slice(&len.to_le_bytes()[0..4]);
        return out;
    }
    if len <= 1_099_511_627_775 {
        let mut out: Vec<u8> = vec![5];
        out.extend_from_slice(&len.to_le_bytes()[0..5]);
        return out;
    }
    if len <= 281_474_976_710_655 {
        let mut out: Vec<u8> = vec![6];
        out.extend_from_slice(&len.to_le_bytes()[0..6]);
        return out;
    }
    if len <= 7_205_759_403_792_735 {
        let mut out: Vec<u8> = vec![7];
        out.extend_from_slice(&len.to_le_bytes()[0..7]);
        return out;
    }
    let mut out: Vec<u8> = vec![8];
    out.extend_from_slice(&len.to_le_bytes());
    out
}
