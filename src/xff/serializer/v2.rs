use athena::{byte_bit::byte_bit_encoder, checksum::{crc32_with_table, generate_crc32_lookuptable, Crc32Table}, encoding_and_decoding::serialize_leb128_unsigned};

use crate::{error::{NabuError, Result}, Array, Data, Number, Object, XffValue};

pub fn serialize_xff_v2(data: Vec<XffValue>) -> Result<Vec<u8>> {
    let table: Crc32Table = generate_crc32_lookuptable();
    let file_data = serialize_xff_v2_value(&data[0], &table)?;
    let file_checksum = crc32_with_table(&file_data, &table);
    let mut out: Vec<u8> = Vec::with_capacity(file_data.len() + 7);
    // Version 2
    out.push(byte_bit_encoder(&[1, 1, 0, 0, 0, 0, 0, 0]));
    out.extend(file_data);
    // CHK
    out.push(23);
    out.extend(file_checksum.to_le_bytes());
    // EM
    out.push(25);
    Ok(out)
}

fn serialize_xff_v2_value(data: &XffValue, table: &Crc32Table) -> Result<Vec<u8>> {
    match data {
        XffValue::String(s) => serialize_xff_v2_string(s, table),
        XffValue::Number(n) => serialize_xff_v2_number(n, table),
        XffValue::Array(a) => serialize_xff_v2_array(a, table),
        XffValue::Object(o) => serialize_xff_v2_object(o, table),
        XffValue::Data(d) => serialize_xff_v2_data(d, table),
        XffValue::Boolean(b) => {
            if *b {
                    Ok(vec![16])
                } else {
                    Ok(vec![17])
                }
        },
        XffValue::Null => Ok(vec![0]),
        _ => Err(NabuError::InvalidXFFVersion(data.clone(), 2))
    }
}

fn serialize_xff_v2_string(s: &str, table: &Crc32Table) -> Result<Vec<u8>> {
    // first create the string
    let tmp: Vec<u8> = {
        let mut out = Vec::new();
        for char in s.chars() {
            let tmp = char as u8;
            if tmp >= 8 && tmp <= 13 {
                out.push(tmp);
            } else if tmp >= 32 && tmp <= 126 {
                out.push(tmp);
            } else if tmp == 128 || tmp == 142 {
                out.push(tmp);
            } else if tmp >= 130 && tmp <= 140 {
                out.push(tmp);
            } else if tmp >= 145 && tmp <= 156 {
                out.push(tmp);
            } else if tmp >= 158 {
                out.push(tmp);
            } else {
                return Err(NabuError::StringContainsNonASCII(s.to_string(), 2));
            }
        }
        out

    };
    let checksum = crc32_with_table(&tmp, &table);
    // now byte structure and push
    let mut out: Vec<u8> = Vec::with_capacity(tmp.len());
    out.push(1);
    out.extend(serialize_leb128_unsigned(tmp.len()));
    out.extend(tmp);
    out.push(23);
    out.extend(checksum.to_le_bytes());
    out.push(24);
    Ok(out)
}

fn serialize_xff_v2_number(n: &Number, table: &Crc32Table) -> Result<Vec<u8>> {
    // first create the string from the number
    let tmp: Vec<u8> = {
        let mut out = Vec::new();
        let mut sep_used = false;
        let mut neg_used = false;
        for char in n.as_string().chars() {
            let tmp = char as u8;
            if tmp == 45 {
                if neg_used {
                    return Err(NabuError::NumberContainsInvalidCharacter(tmp, n.as_string(), 2));
                } else {
                    out.push(tmp);
                    neg_used = true;
                }
            } else if tmp == 44 || tmp == 46 {
                if sep_used {
                    return Err(NabuError::NumberContainsInvalidCharacter(tmp, n.as_string(), 2));
                } else {
                    out.push(tmp);
                    sep_used = true;
                }
            } else if tmp >= 48 && tmp <= 57 {
                out.push(tmp);
            } else {
                return Err(NabuError::NumberContainsInvalidCharacter(tmp, n.as_string(), 2));
            }
        }
        out

    };
    let checksum = crc32_with_table(&tmp, &table);
    // now byte structure and push - over allocate for length
    let mut out: Vec<u8> = Vec::with_capacity(tmp.len() + 10);
    out.push(2);
    out.extend(serialize_leb128_unsigned(tmp.len()));
    out.extend(tmp);
    out.push(23);
    out.extend(checksum.to_le_bytes());
    out.push(24);
    Ok(out)
}

fn serialize_xff_v2_array(a: &Array, table: &Crc32Table) -> Result<Vec<u8>> {
    // still inefficient, but will have to do
    // smallest value is NUL = 1 byte, other values have min sizes around 10
    let mut array_bytes: Vec<u8> = Vec::with_capacity(a.len() * 10);
    for value in &a.values {
        array_bytes.extend(serialize_xff_v2_value(value, table)?);
        // RS separator
        array_bytes.push(30);
    }
    // byte structure and push
    let checksum = crc32_with_table(&array_bytes, &table);
    let mut out: Vec<u8> = Vec::with_capacity(array_bytes.len() + 10);
    out.push(3);
    out.extend(serialize_leb128_unsigned(array_bytes.len()));
    out.extend(array_bytes);
    out.push(23);
    out.extend(checksum.to_le_bytes());
    out.push(24);
    Ok(out)
}

fn serialize_xff_v2_object(o: &Object, table: &Crc32Table) -> Result<Vec<u8>> {
    // still inefficient, but will have to do
    // smallest value is NUL = 1 byte, other values have min sizes around 10
    // also obj itself takes a few bytes
    let mut object_bytes: Vec<u8> = Vec::with_capacity(o.len() * 10);
    for (key, value) in o.map.iter() {
        // GS
        object_bytes.push(29);
        // key
        object_bytes.extend(serialize_xff_v2_value(&XffValue::from(key.as_str()), table)?);
        // US
        object_bytes.push(31);
        // value
        object_bytes.extend(serialize_xff_v2_value(value, table)?);
        // Trailing GS
        object_bytes.push(29);
        // RS separator
        object_bytes.push(30);
    }
    // byte structure and push
    let checksum = crc32_with_table(&object_bytes, &table);
    let mut out: Vec<u8> = Vec::with_capacity(object_bytes.len() + 10);
    out.push(4);
    out.extend(serialize_leb128_unsigned(object_bytes.len()));
    out.extend(object_bytes);
    out.push(23);
    out.extend(checksum.to_le_bytes());
    out.push(24);
    Ok(out)
}

fn serialize_xff_v2_data(d: &Data, table: &Crc32Table) -> Result<Vec<u8>> {
    // byte structure and push
    let checksum = crc32_with_table(&d.data, &table);
    let mut out: Vec<u8> = Vec::with_capacity(d.data.len() + 10);
    out.push(5);
    out.extend(serialize_leb128_unsigned(d.data.len()));
    out.extend(d.data.clone());
    out.push(23);
    out.extend(checksum.to_le_bytes());
    out.push(24);
    Ok(out)
}
