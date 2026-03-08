use athena::XffValue;
use athena::encoding_and_decoding::{deserialize_leb128_unsigned, deserialize_leb128_signed_v3};
use athena::byte_bit::is_even_parity;
use athena::checksum::crc32;
use athena::{Data, Number, Uuid, Array, Object, Table};

use crate::error::{NabuError, Result};
use crate::xff::v3_markers::*;

pub fn deserialize_xff_v3(content: &[u8], cursor: &mut usize) -> Result<XffValue> {
    // 1. Check for Head Metadata (optional)
    if *cursor < content.len() && content[*cursor] == ensure_parity(META) {
        let _metadata = deserialize_v3_value(content, cursor)?;
        // We currently just skip the metadata in the head and return the body value
        // unless we want to return a tuple or attach it to the body value somehow.
    }

    // 2. Body: Exactly one single XFF Value
    let value = deserialize_v3_value(content, cursor)?;

    // 3. Terminator: EM
    let em_marker = read_byte(content, cursor)?;
    if em_marker != ensure_parity(EM) {
        return Err(NabuError::MissingEM(*cursor));
    }

    Ok(value)
}

fn deserialize_v3_value(content: &[u8], cursor: &mut usize) -> Result<XffValue> {
    let marker = read_byte(content, cursor)?;
    if !is_even_parity(marker) {
        return Err(NabuError::InvalidMarkerParity(marker, *cursor - 1));
    }

    match marker {
        m if m == ensure_parity(NUL) => Ok(XffValue::Null),
        m if m == ensure_parity(TRU) => Ok(XffValue::Boolean(true)),
        m if m == ensure_parity(FAL) => Ok(XffValue::Boolean(false)),
        m if m == ensure_parity(NAN) => Ok(XffValue::NaN),
        m if m == ensure_parity(INF) => Ok(XffValue::Infinity),
        m if m == ensure_parity(NINF) => Ok(XffValue::NegInfinity),
        
        m if m == ensure_parity(TXT) => {
            let (len, leb_len) = deserialize_leb128_unsigned(&content[*cursor..])
                .map_err(|_| NabuError::InvalidXFFValueLength(0, 3))?;
            let start = *cursor;
            *cursor += leb_len as usize;
            let data_start = *cursor;
            *cursor += len;
            
            let checksum_start = *cursor;
            let checksum = read_u32_le(content, cursor)?;
            
            // Verify checksum over [Length] + [Payload]
            if crc32(&content[start..checksum_start]) != checksum {
                return Err(NabuError::InvalidXFFValueChecksum(checksum_start, 3));
            }
            
            let ev = read_byte(content, cursor)?;
            if ev != ensure_parity(EV) {
                return Err(NabuError::MissingEV(*cursor - 1));
            }
            
            let s = String::from_utf8(content[data_start..checksum_start].to_vec())
                .map_err(|_| NabuError::StringContainsNonASCII(String::new(), 3))?;
            Ok(XffValue::String(s))
        }

        m if m == ensure_parity(UINT) => {
            let start = *cursor;
            let (val, leb_len) = deserialize_leb128_unsigned(&content[*cursor..])
                .map_err(|_| NabuError::InvalidXFFValueLength(0, 3))?;
            *cursor += leb_len as usize;
            let checksum_start = *cursor;
            let checksum = read_u32_le(content, cursor)?;
            
            if crc32(&content[start..checksum_start]) != checksum {
                return Err(NabuError::InvalidXFFValueChecksum(checksum_start, 3));
            }
            
            let ev = read_byte(content, cursor)?;
            if ev != ensure_parity(EV) {
                return Err(NabuError::MissingEV(*cursor - 1));
            }
            Ok(XffValue::Number(Number::from(val)))
        }

        m if m == ensure_parity(SINT) => {
            let start = *cursor;
            let (val, leb_len) = deserialize_leb128_signed_v3(&content[*cursor..])
                .map_err(|_| NabuError::InvalidXFFValueLength(0, 3))?;
            *cursor += leb_len as usize;
            let checksum_start = *cursor;
            let checksum = read_u32_le(content, cursor)?;
            
            if crc32(&content[start..checksum_start]) != checksum {
                return Err(NabuError::InvalidXFFValueChecksum(checksum_start, 3));
            }
            
            let ev = read_byte(content, cursor)?;
            if ev != ensure_parity(EV) {
                return Err(NabuError::MissingEV(*cursor - 1));
            }
            Ok(XffValue::Number(Number::from(val as isize)))
        }

        m if m == ensure_parity(FLT) => {
            let start = *cursor;
            let val = read_f64_le(content, cursor)?;
            let checksum_start = *cursor;
            let checksum = read_u32_le(content, cursor)?;
            
            if crc32(&content[start..checksum_start]) != checksum {
                return Err(NabuError::InvalidXFFValueChecksum(checksum_start, 3));
            }
            
            let ev = read_byte(content, cursor)?;
            if ev != ensure_parity(EV) {
                return Err(NabuError::MissingEV(*cursor - 1));
            }
            Ok(XffValue::Number(Number::from(val)))
        }

        m if m == ensure_parity(DAT) => {
            let (len, leb_len) = deserialize_leb128_unsigned(&content[*cursor..])
                .map_err(|_| NabuError::InvalidXFFValueLength(0, 3))?;
            let start = *cursor;
            *cursor += leb_len as usize;
            let data_start = *cursor;
            *cursor += len;
            
            let checksum_start = *cursor;
            let checksum = read_u32_le(content, cursor)?;
            
            if crc32(&content[start..checksum_start]) != checksum {
                return Err(NabuError::InvalidXFFValueChecksum(checksum_start, 3));
            }
            
            let ev = read_byte(content, cursor)?;
            if ev != ensure_parity(EV) {
                return Err(NabuError::MissingEV(*cursor - 1));
            }
            
            Ok(XffValue::Data(Data::from(content[data_start..checksum_start].to_vec())))
        }

        m if m == ensure_parity(DT) => {
            let start = *cursor;
            let (val, leb_len) = deserialize_leb128_unsigned(&content[*cursor..])
                .map_err(|_| NabuError::InvalidXFFValueLength(0, 3))?;
            *cursor += leb_len as usize;
            let checksum_start = *cursor;
            let checksum = read_u32_le(content, cursor)?;
            if crc32(&content[start..checksum_start]) != checksum {
                return Err(NabuError::InvalidXFFValueChecksum(checksum_start, 3));
            }
            let ev = read_byte(content, cursor)?;
            if ev != ensure_parity(EV) { return Err(NabuError::MissingEV(*cursor - 1)); }
            Ok(XffValue::DateTime(val as u64))
        }

        m if m == ensure_parity(DUR) => {
            let start = *cursor;
            let (val, leb_len) = deserialize_leb128_unsigned(&content[*cursor..])
                .map_err(|_| NabuError::InvalidXFFValueLength(0, 3))?;
            *cursor += leb_len as usize;
            let checksum_start = *cursor;
            let checksum = read_u32_le(content, cursor)?;
            if crc32(&content[start..checksum_start]) != checksum {
                return Err(NabuError::InvalidXFFValueChecksum(checksum_start, 3));
            }
            let ev = read_byte(content, cursor)?;
            if ev != ensure_parity(EV) { return Err(NabuError::MissingEV(*cursor - 1)); }
            Ok(XffValue::Duration(val as u64))
        }

        m if m == ensure_parity(UUID) => {
            let start = *cursor;
            if *cursor + 16 > content.len() { return Err(NabuError::TruncatedXFFValue(*cursor, 3)); }
            let uuid_bytes = &content[*cursor..*cursor + 16];
            *cursor += 16;
            let checksum_start = *cursor;
            let checksum = read_u32_le(content, cursor)?;
            if crc32(&content[start..checksum_start]) != checksum {
                return Err(NabuError::InvalidXFFValueChecksum(checksum_start, 3));
            }
            let ev = read_byte(content, cursor)?;
            if ev != ensure_parity(EV) { return Err(NabuError::MissingEV(*cursor - 1)); }
            Ok(XffValue::Uuid(Uuid::new(uuid_bytes.try_into().unwrap())))
        }

        m if m == ensure_parity(ARY) => {
            let elements = deserialize_v3_parent_elements(content, cursor)?;
            Ok(XffValue::Array(Array::from(elements)))
        }

        m if m == ensure_parity(OBJ) || m == ensure_parity(OOBJ) => {
            let elements = deserialize_v3_parent_elements(content, cursor)?;
            if elements.len() % 2 != 0 {
                return Err(NabuError::InvalidObject(*cursor, 0, 3));
            }
            let mut pairs = Vec::with_capacity(elements.len() / 2);
            for i in (0..elements.len()).step_by(2) {
                let key = elements[i].into_string().ok_or(NabuError::InvalidKey(*cursor, elements[i].clone(), 3))?;
                pairs.push((key, elements[i+1].clone()));
            }
            if marker == ensure_parity(OBJ) {
                Ok(XffValue::Object(Object::from(pairs)))
            } else {
                Ok(XffValue::OrderedObject(pairs))
            }
        }

        m if m == ensure_parity(TBL) => {
            deserialize_v3_table(content, cursor)
        }

        _ => Err(NabuError::InvalidXFFByte(marker, *cursor - 1, 3)),
    }
}

fn deserialize_v3_parent_elements(content: &[u8], cursor: &mut usize) -> Result<Vec<XffValue>> {
    let index_start = *cursor;
    let (element_count, leb_len) = deserialize_leb128_unsigned(&content[*cursor..])
        .map_err(|_| NabuError::InvalidXFFValueLength(0, 3))?;
    *cursor += leb_len as usize;
    
    let mut offsets = Vec::with_capacity(element_count);
    for _ in 0..element_count {
        let (off, leb_len) = deserialize_leb128_unsigned(&content[*cursor..])
            .map_err(|_| NabuError::InvalidXFFValueLength(0, 3))?;
        offsets.push(off);
        *cursor += leb_len as usize;
    }
    
    let index_end = *cursor;
    let checksum = read_u32_le(content, cursor)?;
    if crc32(&content[index_start..index_end]) != checksum {
        return Err(NabuError::IndexChecksumMismatch(index_end));
    }
    
    let mut elements = Vec::with_capacity(element_count);
    for _ in 0..element_count {
        elements.push(deserialize_v3_value(content, cursor)?);
    }
    
    // Validate offsets (optional but good for boundary safety)
    // We could use slice-based parsing here more strictly if we wanted.

    let ev = read_byte(content, cursor)?;
    if ev != ensure_parity(EV) {
        return Err(NabuError::MissingEV(*cursor - 1));
    }
    
    Ok(elements)
}

fn deserialize_v3_table(content: &[u8], cursor: &mut usize) -> Result<XffValue> {
    // 1. Column Index
    let col_index_start = *cursor;
    let (col_count, leb_len) = deserialize_leb128_unsigned(&content[*cursor..])
        .map_err(|_| NabuError::InvalidXFFValueLength(0, 3))?;
    *cursor += leb_len as usize;
    let mut col_offsets = Vec::with_capacity(col_count);
    for _ in 0..col_count {
        let (off, leb_len) = deserialize_leb128_unsigned(&content[*cursor..])
            .map_err(|_| NabuError::InvalidXFFValueLength(0, 3))?;
        col_offsets.push(off);
        *cursor += leb_len as usize;
    }
    let col_index_end = *cursor;
    let col_checksum = read_u32_le(content, cursor)?;
    if crc32(&content[col_index_start..col_index_end]) != col_checksum {
        return Err(NabuError::IndexChecksumMismatch(col_index_end));
    }

    // 2. Column Names
    let mut columns = Vec::with_capacity(col_count);
    for _ in 0..col_count {
        let val = deserialize_v3_value(content, cursor)?;
        columns.push(val.into_string().ok_or(NabuError::InvalidTableSchema(*cursor))?);
    }

    // 3. Row Index
    let row_index_start = *cursor;
    let (row_count, leb_len) = deserialize_leb128_unsigned(&content[*cursor..])
        .map_err(|_| NabuError::InvalidXFFValueLength(0, 3))?;
    *cursor += leb_len as usize;
    let mut row_offsets = Vec::with_capacity(row_count);
    for _ in 0..row_count {
        let (off, leb_len) = deserialize_leb128_unsigned(&content[*cursor..])
            .map_err(|_| NabuError::InvalidXFFValueLength(0, 3))?;
        row_offsets.push(off);
        *cursor += leb_len as usize;
    }
    let row_index_end = *cursor;
    let row_checksum = read_u32_le(content, cursor)?;
    if crc32(&content[row_index_start..row_index_end]) != row_checksum {
        return Err(NabuError::IndexChecksumMismatch(row_index_end));
    }

    // 4. Element Index
    let element_index_start = *cursor;
    let mut element_offsets = Vec::with_capacity(row_count * col_count);
    for _ in 0..(row_count * col_count) {
        let (off, leb_len) = deserialize_leb128_unsigned(&content[*cursor..])
            .map_err(|_| NabuError::InvalidXFFValueLength(0, 3))?;
        element_offsets.push(off);
        *cursor += leb_len as usize;
    }
    let element_index_end = *cursor;
    let element_checksum = read_u32_le(content, cursor)?;
    if crc32(&content[element_index_start..element_index_end]) != element_checksum {
        return Err(NabuError::IndexChecksumMismatch(element_index_end));
    }

    // 5. Row Data
    let mut rows = Vec::with_capacity(row_count);
    for _ in 0..row_count {
        let mut row = Vec::with_capacity(col_count);
        for _ in 0..col_count {
            row.push(deserialize_v3_value(content, cursor)?);
        }
        rows.push(row);
    }

    let ev = read_byte(content, cursor)?;
    if ev != ensure_parity(EV) {
        return Err(NabuError::MissingEV(*cursor - 1));
    }

    Ok(XffValue::Table(Table { columns, rows }))
}

// Helpers
fn read_byte(content: &[u8], cursor: &mut usize) -> Result<u8> {
    if *cursor >= content.len() {
        return Err(NabuError::TruncatedXFF(*cursor, 3));
    }
    let byte = content[*cursor];
    *cursor += 1;
    Ok(byte)
}

fn read_u32_le(content: &[u8], cursor: &mut usize) -> Result<u32> {
    if *cursor + 4 > content.len() {
        return Err(NabuError::TruncatedXFFValueChecksum(*cursor, 3));
    }
    let bytes = &content[*cursor..*cursor + 4];
    *cursor += 4;
    Ok(u32::from_le_bytes(bytes.try_into().unwrap()))
}

fn read_f64_le(content: &[u8], cursor: &mut usize) -> Result<f64> {
    if *cursor + 8 > content.len() {
        return Err(NabuError::TruncatedXFFValue(*cursor, 3));
    }
    let bytes = &content[*cursor..*cursor + 8];
    *cursor += 8;
    Ok(f64::from_le_bytes(bytes.try_into().unwrap()))
}

fn ensure_parity(marker: u8) -> u8 {
    athena::byte_bit::ensure_even_parity(marker)
}
