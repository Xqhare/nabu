use athena::checksum::crc32;
use athena::encoding_and_decoding::{
    serialize_leb128_signed_v3, serialize_leb128_unsigned, serialize_version_bit_chain,
};
use athena::{Array, Data, Metadata, XffValue};

use crate::error::Result;
use crate::xff::v3_markers::{
    ARY, DAT, DT, DUR, EM, EV, FAL, FLT, INF, MAGIC, META, NAN, NINF, NUL, OBJ, OOBJ, SINT, TBL,
    TRU, TXT, UINT, UUID,
};

/// Serializes XFF values into a byte vector using the v3 specification.
///
/// # Errors
/// Errors if serialization fails or if metadata is invalid.
pub fn serialize_xff_v3(data: &[XffValue]) -> Result<Vec<u8>> {
    // If the first value is Metadata, move it to the head
    if let Some(XffValue::Metadata(meta)) = data.first() {
        let body = data.get(1).cloned().unwrap_or(XffValue::Null);
        serialize_xff_v3_with_metadata(&[body], Some(meta.clone().map))
    } else {
        serialize_xff_v3_with_metadata(data, None)
    }
}

/// Serializes XFF values with optional metadata into a byte vector using the v3 specification.
///
/// # Errors
/// Errors if serialization fails or if metadata is invalid.
pub fn serialize_xff_v3_with_metadata(
    data: &[XffValue],
    metadata: Option<athena::Object>,
) -> Result<Vec<u8>> {
    let mut out = Vec::with_capacity(1024); // Start with a reasonable baseline

    // 1. File Signature
    out.extend_from_slice(&MAGIC);

    // 2. Head: Version Encoding (Version 3)
    out.extend(serialize_version_bit_chain(3));

    // 2.1 Head Metadata (Optional)
    if let Some(meta) = metadata {
        let metadata_wrapped = Metadata::from(meta);
        if !metadata_wrapped.is_strict_v3_compliant() {
            return Err(crate::error::NabuError::InvalidMetadata(
                "Metadata must not contain nested parent types".to_string(),
            ));
        }

        let mut pairs: Vec<XffValue> = Vec::with_capacity(metadata_wrapped.len() * 2);
        for (k, v) in &metadata_wrapped.map.map {
            pairs.push(XffValue::from(k.clone()));
            pairs.push(v.clone());
        }
        out.extend(serialize_v3_parent(META, &pairs)?);
    }

    // 3. Body: Exactly one single XFF Value (as per v3 spec)
    if let Some(first_value) = data.first() {
        out.extend(serialize_v3_value(first_value)?);
    }

    // 4. Terminator: EM
    out.push(EM);

    Ok(out)
}

#[allow(clippy::too_many_lines)]
fn serialize_v3_value(value: &XffValue) -> Result<Vec<u8>> {
    match value {
        XffValue::Null => Ok(vec![NUL]),
        XffValue::Boolean(b) => {
            if b.value() {
                Ok(vec![TRU])
            } else {
                Ok(vec![FAL])
            }
        }
        XffValue::String(s) => {
            let s = s.as_str();
            let utf8_bytes = s.as_bytes();
            let len_bytes = serialize_leb128_unsigned(utf8_bytes.len() as u128);

            // Pre-allocate: Marker (1) + Len + Data + Checksum (4) + EV (1)
            let mut buf = Vec::with_capacity(6 + len_bytes.len() + utf8_bytes.len());
            buf.push(TXT);

            let mut payload = len_bytes;
            payload.extend_from_slice(utf8_bytes);

            let checksum = crc32(&payload);

            buf.extend(payload);
            buf.extend_from_slice(&checksum.to_le_bytes());
            buf.push(EV);
            Ok(buf)
        }
        XffValue::Number(n) => Ok(serialize_v3_number(n)),
        XffValue::Metadata(meta) => {
            if !meta.is_strict_v3_compliant() {
                return Err(crate::error::NabuError::InvalidMetadata(
                    "Metadata must not contain nested parent types".to_string(),
                ));
            }
            let mut pairs: Vec<XffValue> = Vec::with_capacity(meta.len() * 2);
            for (k, v) in &meta.map.map {
                pairs.push(XffValue::from(k.clone()));
                pairs.push(v.clone());
            }
            serialize_v3_parent(META, &pairs)
        }
        XffValue::Data(d) => {
            let raw_bytes = &d.data;
            let len_bytes = serialize_leb128_unsigned(raw_bytes.len() as u128);

            let mut buf = Vec::with_capacity(6 + len_bytes.len() + raw_bytes.len());
            buf.push(DAT);

            let mut payload = len_bytes;
            payload.extend_from_slice(raw_bytes);

            let checksum = crc32(&payload);

            buf.extend(payload);
            buf.extend_from_slice(&checksum.to_le_bytes());
            buf.push(EV);
            Ok(buf)
        }
        XffValue::DateTime(dt) => {
            #[allow(clippy::cast_possible_truncation)]
            let payload = serialize_leb128_unsigned(u128::from(dt.as_millis()));
            let mut buf = Vec::with_capacity(6 + payload.len());
            buf.push(DT);
            let checksum = crc32(&payload);
            buf.extend(payload);
            buf.extend_from_slice(&checksum.to_le_bytes());
            buf.push(EV);
            Ok(buf)
        }
        XffValue::Duration(d) => {
            #[allow(clippy::cast_possible_truncation)]
            let payload = serialize_leb128_unsigned(u128::from(d.as_millis()));
            let mut buf = Vec::with_capacity(6 + payload.len());
            buf.push(DUR);
            let checksum = crc32(&payload);
            buf.extend(payload);
            buf.extend_from_slice(&checksum.to_le_bytes());
            buf.push(EV);
            Ok(buf)
        }
        XffValue::Uuid(u) => {
            let bytes = u.as_bytes();
            let mut buf = Vec::with_capacity(22); // 1 + 16 + 4 + 1
            buf.push(UUID);
            let checksum = crc32(bytes);
            buf.extend_from_slice(bytes);
            buf.extend_from_slice(&checksum.to_le_bytes());
            buf.push(EV);
            Ok(buf)
        }
        XffValue::Array(a) => serialize_v3_parent(ARY, &a.values),
        XffValue::Object(o) => {
            let mut pairs = Vec::with_capacity(o.len() * 2);
            for (k, v) in &o.map {
                pairs.push(XffValue::from(k.clone()));
                pairs.push(v.clone());
            }
            serialize_v3_parent(OBJ, &pairs)
        }
        XffValue::OrderedObject(o) => {
            let mut pairs = Vec::with_capacity(o.len() * 2);
            for (k, v) in o {
                pairs.push(XffValue::from(k.clone()));
                pairs.push(v.clone());
            }
            serialize_v3_parent(OOBJ, &pairs)
        }
        XffValue::Table(t) => serialize_v3_table(t),
        XffValue::NaN => Ok(vec![NAN]),
        XffValue::Infinity => Ok(vec![INF]),
        XffValue::NegInfinity => Ok(vec![NINF]),
        XffValue::CommandCharacter(c) => {
            serialize_v3_value(&XffValue::Data(Data::from(vec![c.as_u8()])))
        }
        XffValue::ArrayCmdChar(ac) => {
            let values: Vec<XffValue> = ac
                .iter()
                .map(|c| XffValue::Data(Data::from(vec![c.as_u8()])))
                .collect();
            serialize_v3_value(&XffValue::Array(Array::from(values)))
        }
    }
}

fn serialize_v3_parent(marker: u8, elements: &[XffValue]) -> Result<Vec<u8>> {
    let element_count_bytes = serialize_leb128_unsigned(elements.len() as u128);

    let mut serialized_elements = Vec::with_capacity(elements.len());
    let mut offsets = Vec::with_capacity(elements.len());
    let mut current_offset: u128 = 0;
    let mut total_child_size = 0;

    for el in elements {
        let ser = serialize_v3_value(el)?;
        offsets.push(serialize_leb128_unsigned(current_offset));
        current_offset += ser.len() as u128;
        total_child_size += ser.len();
        serialized_elements.push(ser);
    }

    let mut index_data = Vec::with_capacity(element_count_bytes.len() + (offsets.len() * 2));
    index_data.extend(element_count_bytes);
    for offset_bytes in offsets {
        index_data.extend(offset_bytes);
    }

    let checksum = crc32(&index_data);

    // Final buffer pre-allocation
    let mut buf = Vec::with_capacity(1 + index_data.len() + 4 + total_child_size + 1);
    buf.push(marker);
    buf.extend(index_data);
    buf.extend_from_slice(&checksum.to_le_bytes());

    for ser in serialized_elements {
        buf.extend(ser);
    }

    buf.push(EV);
    Ok(buf)
}

fn serialize_v3_table(t: &athena::Table) -> Result<Vec<u8>> {
    let col_count = t.columns.len();
    let col_count_bytes = serialize_leb128_unsigned(col_count as u128);

    let mut col_names_ser = Vec::with_capacity(col_count);
    let mut col_offsets = Vec::with_capacity(col_count);
    let mut current_col_offset: u128 = 0;
    let mut total_col_names_size = 0;

    for col in &t.columns {
        let ser = serialize_v3_value(&XffValue::from(col.clone()))?;
        col_offsets.push(serialize_leb128_unsigned(current_col_offset));
        current_col_offset += ser.len() as u128;
        total_col_names_size += ser.len();
        col_names_ser.push(ser);
    }

    let mut col_index_data = Vec::with_capacity(col_count_bytes.len() + (col_offsets.len() * 2));
    col_index_data.extend(col_count_bytes);
    for offset in col_offsets {
        col_index_data.extend(offset);
    }
    let col_checksum = crc32(&col_index_data);

    // Row parsing
    let row_count = t.rows.len();
    let row_count_bytes = serialize_leb128_unsigned(row_count as u128);
    let mut row_data_ser = Vec::with_capacity(row_count * col_count);
    let mut row_offsets = Vec::with_capacity(row_count);
    let mut element_offsets = Vec::with_capacity(row_count * col_count);
    let mut current_row_offset: u128 = 0;
    let mut total_row_data_size = 0;

    for row in &t.rows {
        row_offsets.push(serialize_leb128_unsigned(current_row_offset));
        for cell in row {
            let ser = serialize_v3_value(cell)?;
            element_offsets.push(serialize_leb128_unsigned(current_row_offset));
            current_row_offset += ser.len() as u128;
            total_row_data_size += ser.len();
            row_data_ser.push(ser);
        }
    }

    let mut row_index_data = Vec::with_capacity(row_count_bytes.len() + (row_offsets.len() * 2));
    row_index_data.extend(row_count_bytes);
    for offset in row_offsets {
        row_index_data.extend(offset);
    }
    let row_checksum = crc32(&row_index_data);

    let mut element_index_data = Vec::with_capacity(element_offsets.len() * 2);
    for offset in element_offsets {
        element_index_data.extend(offset);
    }
    let element_checksum = crc32(&element_index_data);

    // Final buffer pre-allocation
    let total_size = 1
        + (col_index_data.len() + 4 + total_col_names_size)
        + (row_index_data.len() + 4)
        + (element_index_data.len() + 4)
        + total_row_data_size
        + 1;

    let mut buf = Vec::with_capacity(total_size);
    buf.push(TBL);

    // Column Index + Names
    buf.extend(col_index_data);
    buf.extend_from_slice(&col_checksum.to_le_bytes());
    for ser in col_names_ser {
        buf.extend(ser);
    }

    // Row Index
    buf.extend(row_index_data);
    buf.extend_from_slice(&row_checksum.to_le_bytes());

    // Element Index
    buf.extend(element_index_data);
    buf.extend_from_slice(&element_checksum.to_le_bytes());

    // Row Data
    for ser in row_data_ser {
        buf.extend(ser);
    }

    buf.push(EV);
    Ok(buf)
}

fn serialize_v3_number(n: &athena::Number) -> Vec<u8> {
    if n.is_float() {
        let val = n.into_f64().unwrap();
        if val.is_nan() {
            vec![NAN]
        } else if val.is_infinite() {
            if val.is_sign_positive() {
                vec![INF]
            } else {
                vec![NINF]
            }
        } else {
            let mut buf = Vec::with_capacity(14); // 1 + 8 + 4 + 1
            buf.push(FLT);
            let bytes = val.to_le_bytes();
            let checksum = crc32(&bytes);
            buf.extend_from_slice(&bytes);
            buf.extend_from_slice(&checksum.to_le_bytes());
            buf.push(EV);
            buf
        }
    } else if n.is_unsigned() {
        let val = n.into_usize().unwrap();
        let payload = serialize_leb128_unsigned(val as u128);
        let mut buf = Vec::with_capacity(6 + payload.len());
        buf.push(UINT);
        let checksum = crc32(&payload);
        buf.extend(payload);
        buf.extend_from_slice(&checksum.to_le_bytes());
        buf.push(EV);
        buf
    } else {
        let val = n.into_isize().unwrap() as i64;
        let payload = serialize_leb128_signed_v3(val);
        let mut buf = Vec::with_capacity(6 + payload.len());
        buf.push(SINT);
        let checksum = crc32(&payload);
        buf.extend(payload);
        buf.extend_from_slice(&checksum.to_le_bytes());
        buf.push(EV);
        buf
    }
}
