use athena::XffValue;
use athena::encoding_and_decoding::{deserialize_leb128_unsigned, deserialize_leb128_signed_v3};
use athena::byte_bit::is_even_parity;
use athena::checksum::crc32;
use athena::{Data, Number, Uuid, Array, Object, Table, Metadata};

use crate::error::{NabuError, Result};
use crate::xff::v3_markers::{ARY, DAT, DT, DUR, EM, EV, FAL, FLT, INF, META, NAN, NINF, NUL, OBJ, OOBJ, SINT, TBL, TRU, TXT, UINT, UUID};

/// Deserializes a complete XFF v3 file from a byte slice.
///
/// # Errors
/// Errors if the file is malformed, truncated, or has invalid checksums/parity.
pub fn deserialize_xff_v3(content: &[u8], cursor: &mut usize) -> Result<XffValue> {
    // 1. Check for Head Metadata (optional)
    let mut head_metadata = None;
    if *cursor < content.len() && content[*cursor] == ensure_parity(META) {
        head_metadata = Some(deserialize_v3_value(content, cursor)?);
    }

    // 2. Body: Exactly one single XFF Value
    let value = deserialize_v3_value(content, cursor)?;

    // 3. Terminator: EM
    let em_marker = read_byte(content, cursor)?;
    if em_marker != ensure_parity(EM) {
        return Err(NabuError::MissingEM(*cursor));
    }

    // If metadata was present, return [Metadata, Value] to keep the API unified
    if let Some(meta) = head_metadata {
        Ok(XffValue::Array(Array::from(vec![meta, value])))
    } else {
        Ok(value)
    }
}

/// Internal recursive deserializer for XFF v3 values.
fn deserialize_v3_value(content: &[u8], cursor: &mut usize) -> Result<XffValue> {
    let marker_pos = *cursor;
    let marker = read_byte(content, cursor)?;
    if !is_even_parity(marker) {
        return Err(NabuError::InvalidMarkerParity(marker, marker_pos));
    }

    match marker {
        m if m == ensure_parity(NUL) => Ok(XffValue::Null),
        m if m == ensure_parity(TRU) => Ok(XffValue::Boolean(true)),
        m if m == ensure_parity(FAL) => Ok(XffValue::Boolean(false)),
        m if m == ensure_parity(NAN) => Ok(XffValue::NaN),
        m if m == ensure_parity(INF) => Ok(XffValue::Infinity),
        m if m == ensure_parity(NINF) => Ok(XffValue::NegInfinity),
        
        m if m == ensure_parity(TXT) => {
            let start = *cursor;
            let (len, leb_len) = deserialize_leb128_unsigned(&content[*cursor..])
                .map_err(|_| NabuError::InvalidXFFValueLength(marker_pos, 3))?;
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
            
            let s = std::str::from_utf8(&content[data_start..checksum_start])
                .map_err(|_| NabuError::StringContainsNonASCII(String::new(), 3))?
                .to_string();
            Ok(XffValue::String(s))
        }

        m if m == ensure_parity(UINT) => {
            let start = *cursor;
            let (val, leb_len) = deserialize_leb128_unsigned(&content[*cursor..])
                .map_err(|_| NabuError::InvalidXFFValueLength(marker_pos, 3))?;
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
                .map_err(|_| NabuError::InvalidXFFValueLength(marker_pos, 3))?;
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
            #[allow(clippy::cast_possible_truncation)]
            let num = Number::from(val as isize);
            Ok(XffValue::Number(num))
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
            let start = *cursor;
            let (len, leb_len) = deserialize_leb128_unsigned(&content[*cursor..])
                .map_err(|_| NabuError::InvalidXFFValueLength(marker_pos, 3))?;
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
                .map_err(|_| NabuError::InvalidXFFValueLength(marker_pos, 3))?;
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
                .map_err(|_| NabuError::InvalidXFFValueLength(marker_pos, 3))?;
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
                let key = elements[i].as_string().cloned().ok_or(NabuError::InvalidKey(*cursor, elements[i].clone(), 3))?;
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

        m if m == ensure_parity(META) => {
            let elements = deserialize_v3_parent_elements(content, cursor)?;
            if elements.len() % 2 != 0 {
                return Err(NabuError::InvalidObject(*cursor, 0, 3));
            }
            let mut pairs = Vec::with_capacity(elements.len() / 2);
            for i in (0..elements.len()).step_by(2) {
                let key = elements[i].as_string().cloned().ok_or(NabuError::InvalidKey(*cursor, elements[i].clone(), 3))?;
                pairs.push((key, elements[i+1].clone()));
            }
            Ok(XffValue::Metadata(Metadata::from(Object::from(pairs))))
        }

        _ => Err(NabuError::InvalidXFFByte(marker, marker_pos, 3)),
    }
}

/// Parses child elements for parent types using the index.
fn deserialize_v3_parent_elements(content: &[u8], cursor: &mut usize) -> Result<Vec<XffValue>> {
    let index_start = *cursor;
    let (element_count, leb_len) = deserialize_leb128_unsigned(&content[*cursor..])
        .map_err(|_| NabuError::InvalidXFFValueLength(index_start, 3))?;
    *cursor += leb_len as usize;
    
    // Read and skip offsets
    for _ in 0..element_count {
        let (_, leb_len) = deserialize_leb128_unsigned(&content[*cursor..])
            .map_err(|_| NabuError::InvalidXFFValueLength(*cursor, 3))?;
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
    
    let ev = read_byte(content, cursor)?;
    if ev != ensure_parity(EV) {
        return Err(NabuError::MissingEV(*cursor - 1));
    }
    
    Ok(elements)
}

/// Parses an optimized XFF v3 Table.
fn deserialize_v3_table(content: &[u8], cursor: &mut usize) -> Result<XffValue> {
    // 1. Column Index
    let col_index_start = *cursor;
    let (col_count, leb_len) = deserialize_leb128_unsigned(&content[*cursor..])
        .map_err(|_| NabuError::InvalidXFFValueLength(col_index_start, 3))?;
    *cursor += leb_len as usize;
    for _ in 0..col_count {
        let (_, leb_len) = deserialize_leb128_unsigned(&content[*cursor..])
            .map_err(|_| NabuError::InvalidXFFValueLength(*cursor, 3))?;
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
        .map_err(|_| NabuError::InvalidXFFValueLength(row_index_start, 3))?;
    *cursor += leb_len as usize;
    for _ in 0..row_count {
        let (_, leb_len) = deserialize_leb128_unsigned(&content[*cursor..])
            .map_err(|_| NabuError::InvalidXFFValueLength(*cursor, 3))?;
        *cursor += leb_len as usize;
    }
    let row_index_end = *cursor;
    let row_checksum = read_u32_le(content, cursor)?;
    if crc32(&content[row_index_start..row_index_end]) != row_checksum {
        return Err(NabuError::IndexChecksumMismatch(row_index_end));
    }

    // 4. Element Index
    let element_index_start = *cursor;
    for _ in 0..(row_count * col_count) {
        let (_, leb_len) = deserialize_leb128_unsigned(&content[*cursor..])
            .map_err(|_| NabuError::InvalidXFFValueLength(*cursor, 3))?;
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

/// Reads a single byte from the content and advances the cursor.
fn read_byte(content: &[u8], cursor: &mut usize) -> Result<u8> {
    if *cursor >= content.len() {
        return Err(NabuError::TruncatedXFF(*cursor, 3));
    }
    let byte = content[*cursor];
    *cursor += 1;
    Ok(byte)
}

/// Reads a 4-byte little-endian unsigned integer.
fn read_u32_le(content: &[u8], cursor: &mut usize) -> Result<u32> {
    if *cursor + 4 > content.len() {
        return Err(NabuError::TruncatedXFFValueChecksum(*cursor, 3));
    }
    let bytes = &content[*cursor..*cursor + 4];
    *cursor += 4;
    Ok(u32::from_le_bytes(bytes.try_into().unwrap()))
}

/// Reads an 8-byte little-endian float.
fn read_f64_le(content: &[u8], cursor: &mut usize) -> Result<f64> {
    if *cursor + 8 > content.len() {
        return Err(NabuError::TruncatedXFFValue(*cursor, 3));
    }
    let bytes = &content[*cursor..*cursor + 8];
    *cursor += 8;
    Ok(f64::from_le_bytes(bytes.try_into().unwrap()))
}

/// Simple proxy for athena's parity utility.
fn ensure_parity(marker: u8) -> u8 {
    athena::byte_bit::ensure_even_parity(marker)
}
