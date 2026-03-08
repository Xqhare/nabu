use athena::XffValue;
use athena::encoding_and_decoding::{serialize_version_bit_chain, serialize_leb128_unsigned, serialize_leb128_signed_v3};
use athena::byte_bit::ensure_even_parity;
use athena::checksum::crc32;

use crate::error::Result;
use crate::xff::v3_markers::*;

pub fn serialize_xff_v3(data: Vec<XffValue>) -> Result<Vec<u8>> {
    serialize_xff_v3_with_metadata(data, None)
}

pub fn serialize_xff_v3_with_metadata(data: Vec<XffValue>, metadata: Option<athena::Object>) -> Result<Vec<u8>> {
    let mut out = Vec::new();

    // 1. File Signature
    out.extend_from_slice(&MAGIC);

    // 2. Head: Version Encoding (Version 3)
    out.extend(serialize_version_bit_chain(3));

    // 2.1 Head Metadata (Optional)
    if let Some(meta) = metadata {
        let mut pairs = Vec::new();
        for (k, v) in &meta.map {
            pairs.push(XffValue::String(k.clone()));
            pairs.push(v.clone());
        }
        out.extend(serialize_v3_parent(META, &pairs)?);
    }

    // 3. Body: Exactly one single XFF Value (as per v3 spec)
    if let Some(first_value) = data.first() {
        out.extend(serialize_v3_value(first_value)?);
    }

    // 4. Terminator: EM
    out.push(ensure_even_parity(EM));

    Ok(out)
}

fn serialize_v3_value(value: &XffValue) -> Result<Vec<u8>> {
    match value {
        XffValue::Null => Ok(vec![ensure_even_parity(NUL)]),
        XffValue::Boolean(b) => {
            if *b {
                Ok(vec![ensure_even_parity(TRU)])
            } else {
                Ok(vec![ensure_even_parity(FAL)])
            }
        }
        XffValue::String(s) => {
            let mut buf = Vec::new();
            buf.push(ensure_even_parity(TXT));
            let utf8_bytes = s.as_bytes();
            
            // Length field
            let mut payload = serialize_leb128_unsigned(utf8_bytes.len());
            // Actual data
            payload.extend_from_slice(utf8_bytes);
            
            // Checksum (covers length + data)
            let checksum = crc32(&payload);
            
            buf.extend(payload);
            buf.extend_from_slice(&checksum.to_le_bytes());
            buf.push(ensure_even_parity(EV));
            Ok(buf)
        }
        XffValue::Number(n) => {
            serialize_v3_number(n)
        }
        XffValue::Data(d) => {
            let mut buf = Vec::new();
            buf.push(ensure_even_parity(DAT));
            let raw_bytes = &d.data;
            
            let mut payload = serialize_leb128_unsigned(raw_bytes.len());
            payload.extend_from_slice(raw_bytes);
            
            let checksum = crc32(&payload);
            
            buf.extend(payload);
            buf.extend_from_slice(&checksum.to_le_bytes());
            buf.push(ensure_even_parity(EV));
            Ok(buf)
        }
        XffValue::DateTime(dt) => {
            let mut buf = Vec::new();
            buf.push(ensure_even_parity(DT));
            let payload = serialize_leb128_unsigned(*dt as usize);
            let checksum = crc32(&payload);
            buf.extend(payload);
            buf.extend_from_slice(&checksum.to_le_bytes());
            buf.push(ensure_even_parity(EV));
            Ok(buf)
        }
        XffValue::Duration(d) => {
            let mut buf = Vec::new();
            buf.push(ensure_even_parity(DUR));
            let payload = serialize_leb128_unsigned(*d as usize);
            let checksum = crc32(&payload);
            buf.extend(payload);
            buf.extend_from_slice(&checksum.to_le_bytes());
            buf.push(ensure_even_parity(EV));
            Ok(buf)
        }
        XffValue::Uuid(u) => {
            let mut buf = Vec::new();
            buf.push(ensure_even_parity(UUID));
            let bytes = u.as_bytes();
            let checksum = crc32(bytes);
            buf.extend_from_slice(bytes);
            buf.extend_from_slice(&checksum.to_le_bytes());
            buf.push(ensure_even_parity(EV));
            Ok(buf)
        }
        XffValue::Array(a) => {
            serialize_v3_parent(ARY, &a.values)
        }
        XffValue::Object(o) => {
            let mut pairs = Vec::new();
            for (k, v) in &o.map {
                pairs.push(XffValue::String(k.clone()));
                pairs.push(v.clone());
            }
            serialize_v3_parent(OBJ, &pairs)
        }
        XffValue::OrderedObject(o) => {
            let mut pairs = Vec::new();
            for (k, v) in o {
                pairs.push(XffValue::String(k.clone()));
                pairs.push(v.clone());
            }
            serialize_v3_parent(OOBJ, &pairs)
        }
        XffValue::Table(t) => {
            serialize_v3_table(t)
        }
        XffValue::NaN => Ok(vec![ensure_even_parity(NAN)]),
        XffValue::Infinity => Ok(vec![ensure_even_parity(INF)]),
        XffValue::NegInfinity => Ok(vec![ensure_even_parity(NINF)]),
        _ => {
            Ok(Vec::new())
        }
    }
}

fn serialize_v3_parent(marker: u8, elements: &[XffValue]) -> Result<Vec<u8>> {
    let mut buf = Vec::new();
    buf.push(ensure_even_parity(marker));

    let element_count_bytes = serialize_leb128_unsigned(elements.len());
    
    let mut serialized_elements = Vec::with_capacity(elements.len());
    let mut offsets = Vec::with_capacity(elements.len());
    let mut current_offset = 0;

    for el in elements {
        let ser = serialize_v3_value(el)?;
        offsets.push(serialize_leb128_unsigned(current_offset));
        current_offset += ser.len();
        serialized_elements.push(ser);
    }

    let mut index_data = element_count_bytes;
    for offset_bytes in offsets {
        index_data.extend(offset_bytes);
    }

    let checksum = crc32(&index_data);
    
    buf.extend(index_data);
    buf.extend_from_slice(&checksum.to_le_bytes());
    
    for ser in serialized_elements {
        buf.extend(ser);
    }

    buf.push(ensure_even_parity(EV));
    Ok(buf)
}

fn serialize_v3_table(t: &athena::Table) -> Result<Vec<u8>> {
    let mut buf = Vec::new();
    buf.push(ensure_even_parity(TBL));

    // 1. Column Index
    let col_count = t.columns.len();
    let mut col_index_data = serialize_leb128_unsigned(col_count);
    
    let mut col_names_ser = Vec::with_capacity(col_count);
    let mut col_offsets = Vec::with_capacity(col_count);
    let mut current_col_offset = 0;

    for col in &t.columns {
        let ser = serialize_v3_value(&XffValue::String(col.clone()))?;
        col_offsets.push(serialize_leb128_unsigned(current_col_offset));
        current_col_offset += ser.len();
        col_names_ser.push(ser);
    }

    for offset in col_offsets {
        col_index_data.extend(offset);
    }
    let col_checksum = crc32(&col_index_data);
    buf.extend(col_index_data);
    buf.extend_from_slice(&col_checksum.to_le_bytes());

    // 2. Column Names
    for ser in col_names_ser {
        buf.extend(ser);
    }

    // 3. Row Index
    let row_count = t.rows.len();
    let mut row_index_data = serialize_leb128_unsigned(row_count);
    
    let mut row_data_ser = Vec::new();
    let mut row_offsets = Vec::with_capacity(row_count);
    let mut element_offsets = Vec::with_capacity(row_count * col_count);
    let mut current_row_offset = 0;

    for row in &t.rows {
        row_offsets.push(serialize_leb128_unsigned(current_row_offset));
        for cell in row {
            let ser = serialize_v3_value(cell)?;
            element_offsets.push(serialize_leb128_unsigned(current_row_offset));
            current_row_offset += ser.len();
            row_data_ser.push(ser);
        }
    }

    for offset in row_offsets {
        row_index_data.extend(offset);
    }
    let row_checksum = crc32(&row_index_data);
    buf.extend(row_index_data);
    buf.extend_from_slice(&row_checksum.to_le_bytes());

    // 4. Element Index
    let mut element_index_data = Vec::new();
    for offset in element_offsets {
        element_index_data.extend(offset);
    }
    let element_checksum = crc32(&element_index_data);
    buf.extend(element_index_data);
    buf.extend_from_slice(&element_checksum.to_le_bytes());

    // 5. Row Data
    for ser in row_data_ser {
        buf.extend(ser);
    }

    buf.push(ensure_even_parity(EV));
    Ok(buf)
}

fn serialize_v3_number(n: &athena::Number) -> Result<Vec<u8>> {
    let mut buf = Vec::new();
    if n.is_float() {
        let val = n.into_f64().unwrap();
        if val.is_nan() {
            buf.push(ensure_even_parity(NAN));
        } else if val.is_infinite() {
            if val.is_sign_positive() {
                buf.push(ensure_even_parity(INF));
            } else {
                buf.push(ensure_even_parity(NINF));
            }
        } else {
            buf.push(ensure_even_parity(FLT));
            let bytes = val.to_le_bytes();
            let checksum = crc32(&bytes);
            buf.extend_from_slice(&bytes);
            buf.extend_from_slice(&checksum.to_le_bytes());
            buf.push(ensure_even_parity(EV));
        }
    } else if n.is_unsigned() {
        buf.push(ensure_even_parity(UINT));
        let val = n.into_usize().unwrap();
        let payload = serialize_leb128_unsigned(val);
        let checksum = crc32(&payload);
        buf.extend(payload);
        buf.extend_from_slice(&checksum.to_le_bytes());
        buf.push(ensure_even_parity(EV));
    } else {
        buf.push(ensure_even_parity(SINT));
        let val = n.into_isize().unwrap() as i64;
        let payload = serialize_leb128_signed_v3(val);
        let checksum = crc32(&payload);
        buf.extend(payload);
        buf.extend_from_slice(&checksum.to_le_bytes());
        buf.push(ensure_even_parity(EV));
    }
    Ok(buf)
}
