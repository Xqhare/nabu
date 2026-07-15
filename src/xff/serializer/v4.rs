use athena::checksum::crc32;
use athena::encoding_and_decoding::{
    serialize_leb128_signed_v3, serialize_leb128_unsigned, serialize_version_bit_chain,
};
use athena::{Array, Data, Metadata, XffValue};

use crate::error::{NabuError, Result as NemesisResult};
use nemesis::NemesisResultExt;
type Result<T> = std::result::Result<T, NabuError>;
use crate::xff::v4_markers::{complex, internal, parent, simple};

/// Serializes XFF values into a byte vector using the v4 specification.
///
/// # Errors
/// Errors if serialization fails or if metadata is invalid.
pub fn serialize_xff_v4(data: &[XffValue]) -> NemesisResult<Vec<u8>> {
    serialize_xff_v4_inner(data).add_source("nabu::xff::serializer::v4")
}

fn serialize_xff_v4_inner(data: &[XffValue]) -> Result<Vec<u8>> {
    // If the first value is Metadata, move it to the head
    if let Some(XffValue::Metadata(meta)) = data.first() {
        let body = data.get(1).cloned().unwrap_or(XffValue::Null);
        serialize_xff_v4_with_metadata(&[body], Some(meta.clone().map))
    } else {
        serialize_xff_v4_with_metadata(data, None)
    }
}

/// Serializes XFF values with optional metadata into a byte vector using the v4 specification.
///
/// # Errors
/// Errors if serialization fails or if metadata is invalid.
pub fn serialize_xff_v4_with_metadata(
    data: &[XffValue],
    metadata: Option<athena::Object>,
) -> Result<Vec<u8>> {
    let mut out = Vec::with_capacity(1024); // Start with a reasonable baseline

    // 1. File Signature
    out.extend_from_slice(b"XFFV");

    // 2. Head: Version Encoding (Version 4)
    out.extend(serialize_version_bit_chain(4));

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
        serialize_v4_parent(parent::META, &pairs, &mut out)?;
    }

    // 3. Body: Exactly one single XFF Value (as per v4 spec)
    if let Some(first_value) = data.first() {
        serialize_v4_value(first_value, &mut out)?;
    }

    // 4. Terminator: EM
    out.push(internal::EM);

    Ok(out)
}

#[allow(clippy::too_many_lines)]
fn serialize_v4_value(value: &XffValue, out: &mut Vec<u8>) -> Result<()> {
    match value {
        XffValue::Null => {
            out.push(simple::NUL);
            Ok(())
        }
        XffValue::Boolean(b) => {
            if b.0 {
                out.push(simple::TRU);
            } else {
                out.push(simple::FAL);
            }
            Ok(())
        }
        XffValue::String(s) => serialize_v4_text(s.as_str(), complex::TXT, out),
        XffValue::Ascii(s) => serialize_v4_text(s.as_str(), complex::ASCI, out),
        XffValue::Number(n) => serialize_v4_number(n, out),
        XffValue::HpFloat(hp) => {
            out.push(complex::CFLT);
            let mut payload =
                athena::encoding_and_decoding::serialize_leb128_signed_i128(hp.get_value());
            payload.extend(serialize_leb128_unsigned(u128::from(hp.get_scale())));
            let checksum = crc32(&payload);
            out.extend(payload);
            out.extend_from_slice(&checksum.to_le_bytes());
            out.push(internal::EV);
            Ok(())
        }

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
            serialize_v4_parent(parent::META, &pairs, out)
        }
        XffValue::Data(d) => {
            let raw_bytes = &d.data;
            let len_bytes = serialize_leb128_unsigned(raw_bytes.len() as u128);
            out.push(complex::DAT);
            let payload_start = out.len();
            out.extend_from_slice(&len_bytes);
            out.extend_from_slice(raw_bytes);
            let payload_end = out.len();
            let checksum = crc32(&out[payload_start..payload_end]);
            out.extend_from_slice(&checksum.to_le_bytes());
            out.push(internal::EV);
            Ok(())
        }
        XffValue::DateTime(dt) => {
            let payload = serialize_leb128_unsigned(u128::from(dt.0));
            out.push(complex::DT);
            let checksum = crc32(&payload);
            out.extend(payload);
            out.extend_from_slice(&checksum.to_le_bytes());
            out.push(internal::EV);
            Ok(())
        }
        XffValue::Duration(d) => {
            let payload = serialize_leb128_unsigned(u128::from(d.0));
            out.push(complex::DUR);
            let checksum = crc32(&payload);
            out.extend(payload);
            out.extend_from_slice(&checksum.to_le_bytes());
            out.push(internal::EV);
            Ok(())
        }
        XffValue::LocalDate(ld) => {
            out.push(complex::LD);
            let payload_start = out.len();
            out.extend_from_slice(&ld.year.to_le_bytes());
            out.push(ld.month);
            out.push(ld.day);
            let checksum = crc32(&out[payload_start..]);
            out.extend_from_slice(&checksum.to_le_bytes());
            out.push(internal::EV);
            Ok(())
        }
        XffValue::LocalTime(lt) => {
            out.push(complex::LT);
            let payload_start = out.len();
            out.push(lt.hour);
            out.push(lt.minute);
            out.push(lt.second);
            out.extend(serialize_leb128_unsigned(lt.subseconds as u128));
            let checksum = crc32(&out[payload_start..]);
            out.extend_from_slice(&checksum.to_le_bytes());
            out.push(internal::EV);
            Ok(())
        }
        XffValue::LocalDateTime(ldt) => {
            out.push(complex::LDT);
            let payload_start = out.len();
            out.extend_from_slice(&ldt.date.year.to_le_bytes());
            out.push(ldt.date.month);
            out.push(ldt.date.day);
            out.push(ldt.time.hour);
            out.push(ldt.time.minute);
            out.push(ldt.time.second);
            out.extend(serialize_leb128_unsigned(ldt.time.subseconds as u128));
            let checksum = crc32(&out[payload_start..]);
            out.extend_from_slice(&checksum.to_le_bytes());
            out.push(internal::EV);
            Ok(())
        }
        XffValue::Uuid(u) => {
            let bytes = u.as_bytes();
            out.push(complex::UUID);
            let checksum = crc32(bytes);
            out.extend_from_slice(bytes);
            out.extend_from_slice(&checksum.to_le_bytes());
            out.push(internal::EV);
            Ok(())
        }
        XffValue::Array(a) => serialize_v4_parent(parent::ARY, &a.values, out),
        XffValue::Object(o) => {
            let mut pairs = Vec::with_capacity(o.len() * 2);
            for (k, v) in &o.map {
                pairs.push(XffValue::from(k.clone()));
                pairs.push(v.clone());
            }
            serialize_v4_parent(parent::OBJ, &pairs, out)
        }
        XffValue::OrderedObject(o) => {
            let mut pairs = Vec::with_capacity(o.len() * 2);
            for (k, v) in o {
                pairs.push(XffValue::from(k.clone()));
                pairs.push(v.clone());
            }
            serialize_v4_parent(parent::OOBJ, &pairs, out)
        }
        XffValue::Table(t) => serialize_v4_table(t, out),
        XffValue::Graph(g) => serialize_v4_graph(g, out),
        XffValue::NaN => {
            out.push(simple::NAN);
            Ok(())
        }
        XffValue::PosNaN => {
            out.extend_from_slice(&simple::PNAN);
            Ok(())
        }
        XffValue::NegNaN => {
            out.extend_from_slice(&simple::NNAN);
            Ok(())
        }
        XffValue::Infinity => {
            out.extend_from_slice(&simple::INF);
            Ok(())
        }
        XffValue::NegInfinity => {
            out.extend_from_slice(&simple::NINF);
            Ok(())
        }
        XffValue::CommandCharacter(c) => {
            serialize_v4_value(&XffValue::Data(Data::from(vec![c.as_u8()])), out)
        }
        XffValue::ArrayCmdChar(ac) => {
            let values: Vec<XffValue> = ac
                .iter()
                .map(|c| XffValue::Data(Data::from(vec![c.as_u8()])))
                .collect();
            serialize_v4_value(&XffValue::Array(Array::from(values)), out)
        }
    }
}

fn serialize_v4_text(s: &str, marker: u8, out: &mut Vec<u8>) -> Result<()> {
    let utf8_bytes = s.as_bytes();
    let len_bytes = serialize_leb128_unsigned(utf8_bytes.len() as u128);
    out.push(marker);
    let payload_start = out.len();
    out.extend_from_slice(&len_bytes);
    out.extend_from_slice(utf8_bytes);
    let payload_end = out.len();
    let checksum = crc32(&out[payload_start..payload_end]);
    out.extend_from_slice(&checksum.to_le_bytes());
    out.push(internal::EV);
    Ok(())
}

fn serialize_v4_parent(marker: u8, elements: &[XffValue]) -> Result<Vec<u8>> {
    let element_count_bytes = serialize_leb128_unsigned(elements.len() as u128);

    let mut serialized_elements = Vec::with_capacity(elements.len());
    let mut deltas = Vec::with_capacity(elements.len());
    let mut prev_offset: u128 = 0;
    let mut current_offset: u128 = 0;
    let mut total_child_size = 0;

    for el in elements {
        let ser = serialize_v4_value(el)?;
        let delta = current_offset - prev_offset;
        deltas.push(serialize_leb128_unsigned(delta));

        prev_offset = current_offset;
        current_offset += ser.len() as u128;
        total_child_size += ser.len();
        serialized_elements.push(ser);
    }

    let mut index_data = Vec::with_capacity(element_count_bytes.len() + (deltas.len() * 2));
    index_data.extend(element_count_bytes);
    for delta_bytes in deltas {
        index_data.extend(delta_bytes);
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

    buf.push(internal::EV);
    Ok(buf)
}

fn serialize_v4_table(t: &athena::Table) -> Result<Vec<u8>> {
    let col_count = t.columns.len();
    let col_count_bytes = serialize_leb128_unsigned(col_count as u128);

    let mut col_names_ser = Vec::with_capacity(col_count);
    let mut col_deltas = Vec::with_capacity(col_count);
    let mut current_col_offset: u128 = 0;
    let mut prev_col_offset: u128 = 0;
    let mut total_col_names_size = 0;

    for col in &t.columns {
        let ser = serialize_v4_value(&XffValue::from(col.clone()))?;
        let delta = current_col_offset - prev_col_offset;
        col_deltas.push(serialize_leb128_unsigned(delta));

        prev_col_offset = current_col_offset;
        current_col_offset += ser.len() as u128;
        total_col_names_size += ser.len();
        col_names_ser.push(ser);
    }

    let mut col_index_data = Vec::with_capacity(col_count_bytes.len() + (col_deltas.len() * 2));
    col_index_data.extend(col_count_bytes);
    for delta in col_deltas {
        col_index_data.extend(delta);
    }
    let col_checksum = crc32(&col_index_data);

    // Row parsing
    let row_count = t.rows.len();
    let row_count_bytes = serialize_leb128_unsigned(row_count as u128);
    let mut row_data_ser = Vec::with_capacity(row_count * col_count);
    let mut row_deltas = Vec::with_capacity(row_count);
    let mut element_deltas = Vec::with_capacity(row_count * col_count);
    let mut current_row_offset: u128 = 0;
    let mut prev_row_offset: u128 = 0;
    let mut prev_element_offset: u128 = 0;
    let mut total_row_data_size = 0;

    for row in &t.rows {
        let row_delta = current_row_offset - prev_row_offset;
        row_deltas.push(serialize_leb128_unsigned(row_delta));
        prev_row_offset = current_row_offset;

        for cell in row {
            let ser = serialize_v4_value(cell)?;
            let el_delta = current_row_offset - prev_element_offset;
            element_deltas.push(serialize_leb128_unsigned(el_delta));

            prev_element_offset = current_row_offset;
            current_row_offset += ser.len() as u128;
            total_row_data_size += ser.len();
            row_data_ser.push(ser);
        }
    }

    let mut row_index_data = Vec::with_capacity(row_count_bytes.len() + (row_deltas.len() * 2));
    row_index_data.extend(row_count_bytes);
    for delta in row_deltas {
        row_index_data.extend(delta);
    }
    let row_checksum = crc32(&row_index_data);

    let mut element_index_data = Vec::with_capacity(element_deltas.len() * 2);
    for delta in element_deltas {
        element_index_data.extend(delta);
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
    buf.push(parent::TBL);

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

    buf.push(internal::EV);
    Ok(buf)
}

fn serialize_v4_graph(g: &athena::graph::Graph) -> Result<Vec<u8>> {
    let mut buf = Vec::new();
    buf.push(parent::GRPH);

    // Nodes Block
    let mut nodes_ser = Vec::new();
    let mut node_deltas = Vec::new();
    let mut prev_node_offset: u128 = 0;
    let mut current_node_offset: u128 = 0;

    let all_node_indices: Vec<u32> = g.get_all_nodes_indices().collect();
    let node_count = all_node_indices.len();

    for &idx in &all_node_indices {
        let node = g.get_node(idx).unwrap();
        let mut n_ser = Vec::new();
        n_ser.extend(serialize_v4_value(&node.payload)?);
        n_ser.extend(serialize_v4_value(&node.metadata)?);
        n_ser.extend(serialize_v4_value(&XffValue::Array(Array::from(
            node.inbound_connections
                .iter()
                .map(|&i| XffValue::from(i as usize))
                .collect::<Vec<_>>(),
        )))?);
        n_ser.extend(serialize_v4_value(&XffValue::Array(Array::from(
            node.outbound_connections
                .iter()
                .map(|&i| XffValue::from(i as usize))
                .collect::<Vec<_>>(),
        )))?);

        let delta = current_node_offset - prev_node_offset;
        node_deltas.push(serialize_leb128_unsigned(delta));
        prev_node_offset = current_node_offset;
        current_node_offset += n_ser.len() as u128;
        nodes_ser.push(n_ser);
    }

    let mut nodes_index = serialize_leb128_unsigned(node_count as u128);
    for d in node_deltas {
        nodes_index.extend(d);
    }
    let nodes_checksum = crc32(&nodes_index);
    buf.extend(nodes_index);
    buf.extend_from_slice(&nodes_checksum.to_le_bytes());
    for n in nodes_ser {
        buf.extend(n);
    }

    // Connections Block
    let mut conns_ser = Vec::new();
    let mut conn_deltas = Vec::new();
    let mut prev_conn_offset: u128 = 0;
    let mut current_conn_offset: u128 = 0;

    let all_conn_indices: Vec<u32> = g.get_all_connections_indices().collect();
    let conn_count = all_conn_indices.len();

    for &idx in &all_conn_indices {
        let conn = g.get_connection(idx).unwrap();
        let mut c_payload = serialize_leb128_unsigned(conn.from as u128);
        c_payload.extend(serialize_leb128_unsigned(conn.to as u128));
        let c_checksum = crc32(&c_payload);

        let mut c_ser = c_payload;
        c_ser.extend_from_slice(&c_checksum.to_le_bytes());
        c_ser.extend(serialize_v4_value(&conn.metadata)?);

        let delta = current_conn_offset - prev_conn_offset;
        conn_deltas.push(serialize_leb128_unsigned(delta));
        prev_conn_offset = current_conn_offset;
        current_conn_offset += c_ser.len() as u128;
        conns_ser.push(c_ser);
    }

    let mut conns_index = serialize_leb128_unsigned(conn_count as u128);
    for d in conn_deltas {
        conns_index.extend(d);
    }
    let conns_checksum = crc32(&conns_index);
    buf.extend(conns_index);
    buf.extend_from_slice(&conns_checksum.to_le_bytes());
    for c in conns_ser {
        buf.extend(c);
    }

    buf.push(internal::EV);
    Ok(buf)
}

fn serialize_v4_number(n: &athena::Number, out: &mut Vec<u8>) -> Result<()> {
    if n.is_float() {
        let val = n.into_f64().unwrap();
        if val.is_nan() {
            let bits = val.to_bits();
            if bits == 0x7ff8_0000_0000_0000 {
                out.extend_from_slice(&simple::PNAN);
            } else if bits == 0xfff8_0000_0000_0000 {
                out.extend_from_slice(&simple::NNAN);
            } else {
                out.push(complex::FLT);
                let bytes = bits.to_le_bytes();
                let checksum = crc32(&bytes);
                out.extend_from_slice(&bytes);
                out.extend_from_slice(&checksum.to_le_bytes());
                out.push(internal::EV);
            }
        } else if val.is_infinite() {
            if val.is_sign_positive() {
                out.extend_from_slice(&simple::INF);
            } else {
                out.extend_from_slice(&simple::NINF);
            }
        } else {
            out.push(complex::FLT);
            let bytes = val.to_le_bytes();
            let checksum = crc32(&bytes);
            out.extend_from_slice(&bytes);
            out.extend_from_slice(&checksum.to_le_bytes());
            out.push(internal::EV);
        }
    } else if n.is_unsigned() {
        let val = n.into_usize().unwrap();
        let payload = serialize_leb128_unsigned(val as u128);
        out.push(complex::UINT);
        let checksum = crc32(&payload);
        out.extend(payload);
        out.extend_from_slice(&checksum.to_le_bytes());
        out.push(internal::EV);
    } else {
        let val = n.into_isize().unwrap() as i64;
        let payload = serialize_leb128_signed_v3(val);
        out.push(complex::SINT);
        let checksum = crc32(&payload);
        out.extend(payload);
        out.extend_from_slice(&checksum.to_le_bytes());
        out.push(internal::EV);
    }
    Ok(())
}
