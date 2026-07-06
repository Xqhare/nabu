use athena::byte_bit::is_even_parity;
use athena::checksum::crc32;
use athena::encoding_and_decoding::{deserialize_leb128_signed_v3, deserialize_leb128_unsigned};
use athena::float::HpFloat;
use athena::graph::Graph;
use athena::{Array, Data, Metadata, Number, Object, Table, Uuid};
use athena::{LocalDate, LocalDateTime, LocalTime};
use athena::{OrderedObject, XffValue};

use crate::error::{NabuError, Result as NemesisResult};
use nemesis::NemesisResultExt;
type Result<T> = std::result::Result<T, NabuError>;
use crate::xff::v4_markers::{complex, internal, parent, simple};

/// Deserializes a complete XFF v4 file from a byte slice.
///
/// # Errors
/// Errors if the file is malformed, truncated, or has invalid checksums/parity.
pub fn deserialize_xff_v4(content: &[u8], cursor: &mut usize) -> NemesisResult<XffValue> {
    deserialize_xff_v4_inner(content, cursor).add_source("nabu::xff::deserializer::v4")
}

fn deserialize_xff_v4_inner(content: &[u8], cursor: &mut usize) -> Result<XffValue> {
    // 1. Check for Head Metadata (optional)
    let mut head_metadata = None;
    if *cursor < content.len() && content[*cursor] == parent::META {
        head_metadata = Some(deserialize_v4_value(content, cursor)?);
    }

    // 2. Body: Exactly one single XFF Value
    let value = deserialize_v4_value(content, cursor)?;

    // 3. Terminator: EM
    let em_marker = read_byte(content, cursor)?;
    if em_marker != internal::EM {
        return Err(NabuError::MissingEM(*cursor));
    }

    // If metadata was present, return [Metadata, Value] to keep the API unified
    if let Some(meta) = head_metadata {
        Ok(XffValue::Array(Array::from(vec![meta, value])))
    } else {
        Ok(value)
    }
}

/// Internal recursive deserializer for XFF v4 values.
#[allow(clippy::too_many_lines)]
fn deserialize_v4_value(content: &[u8], cursor: &mut usize) -> Result<XffValue> {
    let marker_pos = *cursor;
    let marker = read_byte(content, cursor)?;
    if !is_even_parity(marker) {
        return Err(NabuError::InvalidMarkerParity {
            actual: marker,
            pos: marker_pos,
        });
    }

    match marker {
        internal::CONT => {
            let second_byte = read_byte(content, cursor)?;
            match second_byte {
                simple::NUL => Ok(XffValue::Infinity),
                simple::TRU => Ok(XffValue::NegInfinity),
                simple::FAL => Ok(XffValue::PosNaN),
                simple::NAN => Ok(XffValue::NegNaN),
                _ => Err(NabuError::InvalidXFFByte(second_byte, marker_pos + 1, 4)),
            }
        }
        simple::NUL => Ok(XffValue::Null),
        simple::TRU => Ok(XffValue::from(true)),
        simple::FAL => Ok(XffValue::from(false)),
        simple::NAN => Ok(XffValue::NaN),

        complex::TXT | complex::ASCI => {
            let start = *cursor;
            let (len, leb_len) = deserialize_leb128_unsigned(&content[*cursor..])
                .map_err(|_| NabuError::InvalidXFFValueLength(marker_pos, 4))?;
            *cursor += leb_len as usize;
            let data_start = *cursor;
            *cursor += len as usize;

            let checksum_start = *cursor;
            let checksum = read_u32_le(content, cursor)?;

            let actual_crc = crc32(&content[start..checksum_start]);
            if actual_crc != checksum {
                return Err(NabuError::InvalidXFFValueChecksum {
                    expected: checksum,
                    actual: actual_crc,
                    pos: checksum_start,
                    version: 4,
                });
            }

            let ev = read_byte(content, cursor)?;
            if ev != internal::EV {
                return Err(NabuError::MissingEV(*cursor - 1));
            }

            if marker == complex::ASCI {
                let bytes: Vec<u8> = content[data_start..checksum_start]
                    .iter()
                    .map(|&b| b & 0x7F)
                    .collect();
                let s = String::from_utf8_lossy(&bytes).into_owned();
                Ok(XffValue::Ascii(athena::XffString::from(s)))
            } else {
                let s = std::str::from_utf8(&content[data_start..checksum_start])
                    .map_err(|_| NabuError::StringContainsNonASCII(String::new(), 4))?
                    .to_string();
                Ok(XffValue::from(s))
            }
        }

        complex::UINT => {
            let start = *cursor;
            let (val, leb_len) = deserialize_leb128_unsigned(&content[*cursor..])
                .map_err(|_| NabuError::InvalidXFFValueLength(marker_pos, 4))?;
            *cursor += leb_len as usize;
            let checksum_start = *cursor;
            let checksum = read_u32_le(content, cursor)?;

            let actual_crc = crc32(&content[start..checksum_start]);
            if actual_crc != checksum {
                return Err(NabuError::InvalidXFFValueChecksum {
                    expected: checksum,
                    actual: actual_crc,
                    pos: checksum_start,
                    version: 4,
                });
            }

            let ev = read_byte(content, cursor)?;
            if ev != internal::EV {
                return Err(NabuError::MissingEV(*cursor - 1));
            }
            Ok(XffValue::Number(Number::from(val as u64)))
        }

        complex::SINT => {
            let start = *cursor;
            let (val, leb_len) = deserialize_leb128_signed_v3(&content[*cursor..])
                .map_err(|_| NabuError::InvalidXFFValueLength(marker_pos, 4))?;
            *cursor += leb_len as usize;
            let checksum_start = *cursor;
            let checksum = read_u32_le(content, cursor)?;

            let actual_crc = crc32(&content[start..checksum_start]);
            if actual_crc != checksum {
                return Err(NabuError::InvalidXFFValueChecksum {
                    expected: checksum,
                    actual: actual_crc,
                    pos: checksum_start,
                    version: 4,
                });
            }

            let ev = read_byte(content, cursor)?;
            if ev != internal::EV {
                return Err(NabuError::MissingEV(*cursor - 1));
            }
            #[allow(clippy::cast_possible_truncation)]
            let num = Number::from(val as isize);
            Ok(XffValue::Number(num))
        }

        complex::FLT => {
            let start = *cursor;
            let val = read_f64_le(content, cursor)?;
            let checksum_start = *cursor;
            let checksum = read_u32_le(content, cursor)?;

            let actual_crc = crc32(&content[start..checksum_start]);
            if actual_crc != checksum {
                return Err(NabuError::InvalidXFFValueChecksum {
                    expected: checksum,
                    actual: actual_crc,
                    pos: checksum_start,
                    version: 4,
                });
            }

            let ev = read_byte(content, cursor)?;
            if ev != internal::EV {
                return Err(NabuError::MissingEV(*cursor - 1));
            }
            Ok(XffValue::Number(Number::from(val)))
        }

        complex::CFLT => {
            let start = *cursor;
            let (coefficient, leb_len_c) =
                athena::encoding_and_decoding::deserialize_leb128_signed_i128(&content[*cursor..])
                    .map_err(|_| NabuError::InvalidXFFValueLength(marker_pos, 4))?;
            *cursor += leb_len_c as usize;
            let (scale, leb_len_e) = deserialize_leb128_unsigned(&content[*cursor..])
                .map_err(|_| NabuError::InvalidXFFValueLength(marker_pos, 4))?;
            *cursor += leb_len_e as usize;

            let checksum_start = *cursor;
            let checksum = read_u32_le(content, cursor)?;
            let actual_crc = crc32(&content[start..checksum_start]);
            if actual_crc != checksum {
                return Err(NabuError::InvalidXFFValueChecksum {
                    expected: checksum,
                    actual: actual_crc,
                    pos: checksum_start,
                    version: 4,
                });
            }
            let ev = read_byte(content, cursor)?;
            if ev != internal::EV {
                return Err(NabuError::MissingEV(*cursor - 1));
            }
            Ok(XffValue::HpFloat(HpFloat::new(coefficient, scale as u32)))
        }

        complex::DAT => {
            let start = *cursor;
            let (len, leb_len) = deserialize_leb128_unsigned(&content[*cursor..])
                .map_err(|_| NabuError::InvalidXFFValueLength(marker_pos, 4))?;
            *cursor += leb_len as usize;
            let data_start = *cursor;
            *cursor += len as usize;

            let checksum_start = *cursor;
            let checksum = read_u32_le(content, cursor)?;

            let actual_crc = crc32(&content[start..checksum_start]);
            if actual_crc != checksum {
                return Err(NabuError::InvalidXFFValueChecksum {
                    expected: checksum,
                    actual: actual_crc,
                    pos: checksum_start,
                    version: 4,
                });
            }

            let ev = read_byte(content, cursor)?;
            if ev != internal::EV {
                return Err(NabuError::MissingEV(*cursor - 1));
            }

            Ok(XffValue::Data(Data::from(
                content[data_start..checksum_start].to_vec(),
            )))
        }

        complex::DT => {
            let start = *cursor;
            let (val, leb_len) = deserialize_leb128_unsigned(&content[*cursor..])
                .map_err(|_| NabuError::InvalidXFFValueLength(marker_pos, 4))?;
            *cursor += leb_len as usize;
            let checksum_start = *cursor;
            let checksum = read_u32_le(content, cursor)?;
            let actual_crc = crc32(&content[start..checksum_start]);
            if actual_crc != checksum {
                return Err(NabuError::InvalidXFFValueChecksum {
                    expected: checksum,
                    actual: actual_crc,
                    pos: checksum_start,
                    version: 4,
                });
            }
            let ev = read_byte(content, cursor)?;
            if ev != internal::EV {
                return Err(NabuError::MissingEV(*cursor - 1));
            }
            Ok(XffValue::from_unix_timestamp_millis(val as u64))
        }

        complex::DUR => {
            let start = *cursor;
            let (val, leb_len) = deserialize_leb128_unsigned(&content[*cursor..])
                .map_err(|_| NabuError::InvalidXFFValueLength(marker_pos, 4))?;
            *cursor += leb_len as usize;
            let checksum_start = *cursor;
            let checksum = read_u32_le(content, cursor)?;
            let actual_crc = crc32(&content[start..checksum_start]);
            if actual_crc != checksum {
                return Err(NabuError::InvalidXFFValueChecksum {
                    expected: checksum,
                    actual: actual_crc,
                    pos: checksum_start,
                    version: 4,
                });
            }
            let ev = read_byte(content, cursor)?;
            if ev != internal::EV {
                return Err(NabuError::MissingEV(*cursor - 1));
            }
            Ok(XffValue::from_duration_millis(val as u64))
        }

        complex::LD => {
            let start = *cursor;
            let year = read_u16_le(content, cursor)?;
            let month = read_byte(content, cursor)?;
            let day = read_byte(content, cursor)?;
            let checksum_start = *cursor;
            let checksum = read_u32_le(content, cursor)?;
            let actual_crc = crc32(&content[start..checksum_start]);
            if actual_crc != checksum {
                return Err(NabuError::InvalidXFFValueChecksum {
                    expected: checksum,
                    actual: actual_crc,
                    pos: checksum_start,
                    version: 4,
                });
            }
            let ev = read_byte(content, cursor)?;
            if ev != internal::EV {
                return Err(NabuError::MissingEV(*cursor - 1));
            }
            Ok(XffValue::LocalDate(LocalDate::new(year, month, day)))
        }

        complex::LT => {
            let start = *cursor;
            let hour = read_byte(content, cursor)?;
            let minute = read_byte(content, cursor)?;
            let second = read_byte(content, cursor)?;
            let (subseconds, leb_len) = deserialize_leb128_unsigned(&content[*cursor..])
                .map_err(|_| NabuError::InvalidXFFValueLength(marker_pos, 4))?;
            *cursor += leb_len as usize;
            let checksum_start = *cursor;
            let checksum = read_u32_le(content, cursor)?;
            let actual_crc = crc32(&content[start..checksum_start]);
            if actual_crc != checksum {
                return Err(NabuError::InvalidXFFValueChecksum {
                    expected: checksum,
                    actual: actual_crc,
                    pos: checksum_start,
                    version: 4,
                });
            }
            let ev = read_byte(content, cursor)?;
            if ev != internal::EV {
                return Err(NabuError::MissingEV(*cursor - 1));
            }
            Ok(XffValue::LocalTime(LocalTime::new(
                hour,
                minute,
                second,
                subseconds as u64,
            )))
        }

        complex::LDT => {
            let start = *cursor;
            let year = read_u16_le(content, cursor)?;
            let month = read_byte(content, cursor)?;
            let day = read_byte(content, cursor)?;
            let hour = read_byte(content, cursor)?;
            let minute = read_byte(content, cursor)?;
            let second = read_byte(content, cursor)?;
            let (subseconds, leb_len) = deserialize_leb128_unsigned(&content[*cursor..])
                .map_err(|_| NabuError::InvalidXFFValueLength(marker_pos, 4))?;
            *cursor += leb_len as usize;
            let checksum_start = *cursor;
            let checksum = read_u32_le(content, cursor)?;
            let actual_crc = crc32(&content[start..checksum_start]);
            if actual_crc != checksum {
                return Err(NabuError::InvalidXFFValueChecksum {
                    expected: checksum,
                    actual: actual_crc,
                    pos: checksum_start,
                    version: 4,
                });
            }
            let ev = read_byte(content, cursor)?;
            if ev != internal::EV {
                return Err(NabuError::MissingEV(*cursor - 1));
            }
            Ok(XffValue::LocalDateTime(LocalDateTime::new(
                LocalDate::new(year, month, day),
                LocalTime::new(hour, minute, second, subseconds as u64),
            )))
        }

        complex::UUID => {
            let start = *cursor;
            if *cursor + 16 > content.len() {
                return Err(NabuError::TruncatedXFFValue(*cursor, 4));
            }
            let uuid_bytes = &content[*cursor..*cursor + 16];
            *cursor += 16;
            let checksum_start = *cursor;
            let checksum = read_u32_le(content, cursor)?;
            let actual_crc = crc32(&content[start..checksum_start]);
            if actual_crc != checksum {
                return Err(NabuError::InvalidXFFValueChecksum {
                    expected: checksum,
                    actual: actual_crc,
                    pos: checksum_start,
                    version: 4,
                });
            }
            let ev = read_byte(content, cursor)?;
            if ev != internal::EV {
                return Err(NabuError::MissingEV(*cursor - 1));
            }
            Ok(XffValue::Uuid(Uuid::new(uuid_bytes.try_into().unwrap())))
        }

        parent::ARY => {
            let elements = deserialize_v4_parent_elements(content, cursor)?;
            Ok(XffValue::Array(Array::from(elements)))
        }

        parent::OBJ | parent::OOBJ => {
            let elements = deserialize_v4_parent_elements(content, cursor)?;
            if elements.len() % 2 != 0 {
                return Err(NabuError::InvalidObject(*cursor, 0, 4));
            }
            let mut pairs = Vec::with_capacity(elements.len() / 2);
            for i in (0..elements.len()).step_by(2) {
                let key = match &elements[i] {
                    XffValue::String(s) => s.value.clone(),
                    XffValue::Ascii(s) => s.value.clone(),
                    _ => return Err(NabuError::InvalidKey(*cursor, elements[i].clone(), 4)),
                };
                pairs.push((key, elements[i + 1].clone()));
            }
            if marker == parent::OBJ {
                Ok(XffValue::Object(Object::from(pairs)))
            } else {
                Ok(XffValue::OrderedObject(OrderedObject::from(pairs)))
            }
        }

        parent::TBL => deserialize_v4_table(content, cursor),

        parent::GRPH => deserialize_v4_graph(content, cursor),

        parent::META => {
            let elements = deserialize_v4_parent_elements(content, cursor)?;
            if elements.len() % 2 != 0 {
                return Err(NabuError::InvalidObject(*cursor, 0, 4));
            }
            let mut pairs = Vec::with_capacity(elements.len() / 2);
            for i in (0..elements.len()).step_by(2) {
                let key = match &elements[i] {
                    XffValue::String(s) => s.value.clone(),
                    XffValue::Ascii(s) => s.value.clone(),
                    _ => return Err(NabuError::InvalidKey(*cursor, elements[i].clone(), 4)),
                };
                pairs.push((key, elements[i + 1].clone()));
            }
            Ok(XffValue::Metadata(Metadata::from(Object::from(pairs))))
        }

        _ => Err(NabuError::InvalidXFFByte(marker, marker_pos, 4)),
    }
}

/// Parses child elements for parent types using the index.
fn deserialize_v4_parent_elements(content: &[u8], cursor: &mut usize) -> Result<Vec<XffValue>> {
    let index_start = *cursor;
    let (element_count, leb_len) = deserialize_leb128_unsigned(&content[*cursor..])
        .map_err(|_| NabuError::InvalidXFFValueLength(index_start, 4))?;
    *cursor += leb_len as usize;

    // Read and skip deltas
    for _ in 0..element_count {
        let (_, leb_len) = deserialize_leb128_unsigned(&content[*cursor..])
            .map_err(|_| NabuError::InvalidXFFValueLength(*cursor, 4))?;
        *cursor += leb_len as usize;
    }

    let index_end = *cursor;
    let checksum = read_u32_le(content, cursor)?;
    let actual_crc = crc32(&content[index_start..index_end]);
    if actual_crc != checksum {
        return Err(NabuError::IndexChecksumMismatch {
            expected: checksum,
            actual: actual_crc,
            pos: index_end,
        });
    }

    let mut elements = Vec::with_capacity(element_count as usize);
    for _ in 0..element_count {
        elements.push(deserialize_v4_value(content, cursor)?);
    }

    let ev = read_byte(content, cursor)?;
    if ev != internal::EV {
        return Err(NabuError::MissingEV(*cursor - 1));
    }

    Ok(elements)
}

/// Parses an optimized XFF v4 Table.
fn deserialize_v4_table(content: &[u8], cursor: &mut usize) -> Result<XffValue> {
    // 1. Column Index
    let col_index_start = *cursor;
    let (col_count, leb_len) = deserialize_leb128_unsigned(&content[*cursor..])
        .map_err(|_| NabuError::InvalidXFFValueLength(col_index_start, 4))?;
    *cursor += leb_len as usize;
    for _ in 0..col_count {
        let (_, leb_len) = deserialize_leb128_unsigned(&content[*cursor..])
            .map_err(|_| NabuError::InvalidXFFValueLength(*cursor, 4))?;
        *cursor += leb_len as usize;
    }
    let col_index_end = *cursor;
    let col_checksum = read_u32_le(content, cursor)?;
    let col_actual_crc = crc32(&content[col_index_start..col_index_end]);
    if col_actual_crc != col_checksum {
        return Err(NabuError::IndexChecksumMismatch {
            expected: col_checksum,
            actual: col_actual_crc,
            pos: col_index_end,
        });
    }

    // 2. Column Names
    let mut columns = Vec::with_capacity(col_count as usize);
    for _ in 0..col_count {
        let val = deserialize_v4_value(content, cursor)?;
        columns.push(val.into_string().ok_or(NabuError::InvalidTableSchema {
            row_index: 0,
            expected_cols: col_count as usize,
            actual_cols: 0,
            pos: *cursor,
        })?);
    }

    // 3. Row Index
    let row_index_start = *cursor;
    let (row_count, leb_len) = deserialize_leb128_unsigned(&content[*cursor..])
        .map_err(|_| NabuError::InvalidXFFValueLength(row_index_start, 4))?;
    *cursor += leb_len as usize;
    for _ in 0..row_count {
        let (_, leb_len) = deserialize_leb128_unsigned(&content[*cursor..])
            .map_err(|_| NabuError::InvalidXFFValueLength(*cursor, 4))?;
        *cursor += leb_len as usize;
    }
    let row_index_end = *cursor;
    let row_checksum = read_u32_le(content, cursor)?;
    let row_actual_crc = crc32(&content[row_index_start..row_index_end]);
    if row_actual_crc != row_checksum {
        return Err(NabuError::IndexChecksumMismatch {
            expected: row_checksum,
            actual: row_actual_crc,
            pos: row_index_end,
        });
    }

    // 4. Element Index
    let element_index_start = *cursor;
    for _ in 0..(row_count * col_count) {
        let (_, leb_len) = deserialize_leb128_unsigned(&content[*cursor..])
            .map_err(|_| NabuError::InvalidXFFValueLength(*cursor, 4))?;
        *cursor += leb_len as usize;
    }
    let element_index_end = *cursor;
    let element_checksum = read_u32_le(content, cursor)?;
    let element_actual_crc = crc32(&content[element_index_start..element_index_end]);
    if element_actual_crc != element_checksum {
        return Err(NabuError::IndexChecksumMismatch {
            expected: element_checksum,
            actual: element_actual_crc,
            pos: element_index_end,
        });
    }

    // 5. Row Data
    let mut rows = Vec::with_capacity(row_count as usize);
    for i in 0..row_count {
        let mut row = Vec::with_capacity(col_count as usize);
        for _ in 0..col_count {
            row.push(deserialize_v4_value(content, cursor)?);
        }
        rows.push(row);
        let i_usize = i as usize;
        if rows[i_usize].len() != col_count as usize {
            return Err(NabuError::InvalidTableSchema {
                row_index: i_usize,
                expected_cols: col_count as usize,
                actual_cols: rows[i_usize].len(),
                pos: *cursor,
            });
        }
    }

    let ev = read_byte(content, cursor)?;
    if ev != internal::EV {
        return Err(NabuError::MissingEV(*cursor - 1));
    }

    Ok(XffValue::Table(Table { columns, rows }))
}

fn deserialize_v4_graph(content: &[u8], cursor: &mut usize) -> Result<XffValue> {
    let mut g = Graph::new();

    // 1. Nodes Block
    let nodes_index_start = *cursor;
    let (node_count, leb_len) = deserialize_leb128_unsigned(&content[*cursor..])
        .map_err(|_| NabuError::InvalidXFFValueLength(nodes_index_start, 4))?;
    *cursor += leb_len as usize;
    for _ in 0..node_count {
        let (_, leb_len) = deserialize_leb128_unsigned(&content[*cursor..])
            .map_err(|_| NabuError::InvalidXFFValueLength(*cursor, 4))?;
        *cursor += leb_len as usize;
    }
    let nodes_index_end = *cursor;
    let nodes_checksum = read_u32_le(content, cursor)?;
    let nodes_actual_crc = crc32(&content[nodes_index_start..nodes_index_end]);
    if nodes_actual_crc != nodes_checksum {
        return Err(NabuError::IndexChecksumMismatch {
            expected: nodes_checksum,
            actual: nodes_actual_crc,
            pos: nodes_index_end,
        });
    }

    // We need to store nodes first to add connections later
    // Actually, Graph::add_node assigns indices sequentially if not using free list.
    // Serializer used g.get_all_nodes_indices() which should match.
    for _ in 0..node_count {
        let payload = deserialize_v4_value(content, cursor)?;
        let metadata = deserialize_v4_value(content, cursor)?;
        let _inbound = deserialize_v4_value(content, cursor)?; // We'll rebuild connections
        let _outbound = deserialize_v4_value(content, cursor)?;

        g.add_node(payload, metadata);
    }

    // 2. Connections Block
    let conns_index_start = *cursor;
    let (conn_count, leb_len) = deserialize_leb128_unsigned(&content[*cursor..])
        .map_err(|_| NabuError::InvalidXFFValueLength(conns_index_start, 4))?;
    *cursor += leb_len as usize;
    for _ in 0..conn_count {
        let (_, leb_len) = deserialize_leb128_unsigned(&content[*cursor..])
            .map_err(|_| NabuError::InvalidXFFValueLength(*cursor, 4))?;
        *cursor += leb_len as usize;
    }
    let conns_index_end = *cursor;
    let conns_checksum = read_u32_le(content, cursor)?;
    let conns_actual_crc = crc32(&content[conns_index_start..conns_index_end]);
    if conns_actual_crc != conns_checksum {
        return Err(NabuError::IndexChecksumMismatch {
            expected: conns_checksum,
            actual: conns_actual_crc,
            pos: conns_index_end,
        });
    }

    for _ in 0..conn_count {
        let c_start = *cursor;
        let (from, leb_len_f) = deserialize_leb128_unsigned(&content[*cursor..])
            .map_err(|_| NabuError::InvalidXFFValueLength(*cursor, 4))?;
        *cursor += leb_len_f as usize;
        let (to, leb_len_t) = deserialize_leb128_unsigned(&content[*cursor..])
            .map_err(|_| NabuError::InvalidXFFValueLength(*cursor, 4))?;
        *cursor += leb_len_t as usize;

        let c_payload_end = *cursor;
        let c_checksum = read_u32_le(content, cursor)?;
        let c_actual_crc = crc32(&content[c_start..c_payload_end]);
        if c_actual_crc != c_checksum {
            return Err(NabuError::InvalidXFFValueChecksum {
                expected: c_checksum,
                actual: c_actual_crc,
                pos: c_payload_end,
                version: 4,
            });
        }

        let metadata = deserialize_v4_value(content, cursor)?;
        g.add_connection(from as u32, to as u32, metadata)
            .map_err(|_| NabuError::InvalidObject(*cursor, 0, 4))?;
    }

    let ev = read_byte(content, cursor)?;
    if ev != internal::EV {
        return Err(NabuError::MissingEV(*cursor - 1));
    }

    Ok(XffValue::Graph(g))
}

/// Reads a single byte from the content and advances the cursor.
fn read_byte(content: &[u8], cursor: &mut usize) -> Result<u8> {
    if *cursor >= content.len() {
        return Err(NabuError::TruncatedXFF(*cursor, 4));
    }
    let byte = content[*cursor];
    *cursor += 1;
    Ok(byte)
}

/// Reads a 4-byte little-endian unsigned integer.
fn read_u32_le(content: &[u8], cursor: &mut usize) -> Result<u32> {
    if *cursor + 4 > content.len() {
        return Err(NabuError::TruncatedXFFValueChecksum(*cursor, 4));
    }
    let bytes = &content[*cursor..*cursor + 4];
    *cursor += 4;
    Ok(u32::from_le_bytes(bytes.try_into().unwrap()))
}

/// Reads a 2-byte little-endian unsigned integer.
fn read_u16_le(content: &[u8], cursor: &mut usize) -> Result<u16> {
    if *cursor + 2 > content.len() {
        return Err(NabuError::TruncatedXFFValue(*cursor, 4));
    }
    let bytes = &content[*cursor..*cursor + 2];
    *cursor += 2;
    Ok(u16::from_le_bytes(bytes.try_into().unwrap()))
}

/// Reads an 8-byte little-endian float.
fn read_f64_le(content: &[u8], cursor: &mut usize) -> Result<f64> {
    if *cursor + 8 > content.len() {
        return Err(NabuError::TruncatedXFFValue(*cursor, 4));
    }
    let bytes = &content[*cursor..*cursor + 8];
    *cursor += 8;
    Ok(f64::from_le_bytes(bytes.try_into().unwrap()))
}
