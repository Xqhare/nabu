use std::{
    borrow::Borrow, cell::Cell, collections::{BTreeMap, VecDeque}, io::Read, usize
};

use athena::tools::checksum::crc32::{crc32, generate_crc32_lookuptable};

use crate::{
    error::NabuError,
    xff::value::{Number, XffValue},
    Data,
};

pub fn deserialize_xff_v2(contents: &mut VecDeque<u8>) -> Result<XffValue, NabuError> {
    // version is byte 0; 
    let byte_pos: Cell<usize> = Cell::new(1);

    // check file checksum
    if !check_file_checksum(contents) {
        return Err(NabuError::InvalidFileChecksum);
    }

    let out = deserialize_xff_v2_value(contents, byte_pos.borrow())?;

    // remove the file checksum ( 5 bytes )
    for _ in 0..5 {
        let _ = contents.pop_front();
    }

    // EM check
    if contents.len() > 0 {
        if contents[0] == 25 {
            Ok(out)
        } else {
            Err(NabuError::TruncatedXFF(byte_pos.get()))
        }
    } else {
        Err(NabuError::TruncatedXFF(byte_pos.get()))
    }
}

fn check_file_checksum(contents: &mut VecDeque<u8>) -> bool {
    let mut tmp = contents.clone();
    // pop EM
    tmp.pop_back();
    // pop checksum, 4 bytes
    let mut checksum: [u8; 4] = Default::default();
    // I am reading in the checksum backwards, so I reverse it here
    for i in 3..=0 {
        checksum[i] = tmp.pop_back().unwrap();
    }
    // pop CHK
    tmp.pop_back();

    let file_checksum: u32 = u32::from_le_bytes(checksum);
    if file_checksum == crc32(&tmp.into()) {
        true
    } else {
        false
    }
}

fn deserialize_xff_v2_value(contents: &mut VecDeque<u8>, byte_pos: &Cell<usize>) -> Result<XffValue, NabuError> {
    let table = generate_crc32_lookuptable();
    // TODO: remove later
    Ok(XffValue::Number(Number::from(42)))
}

