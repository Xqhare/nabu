//! This module contains the marker byte definitions for XFF version 4.
//!
//! Version 4 redefines Group 00 (Simple Values) to ensure a 16/4 Hamming distance
//! and introduces new markers for naive date/time types and the Graph parent type.

/// Group 00: Simple Values
///
/// To ensure a minimum Hamming distance of 4, the Group 00 markers are
/// restricted to 4 optimal byte values. Double-byte values are prefixed
/// by the `CONT` (0xFF) byte.
pub mod simple {
    /// Null value (Base marker: 0x00)
    pub const NUL: u8 = 0x00;
    /// True value (Base marker: 0x87)
    pub const TRU: u8 = 0x87;
    /// False value (Base marker: 0x99)
    pub const FAL: u8 = 0x99;
    /// Not a Number (Base marker: 0x1E)
    pub const NAN: u8 = 0x1E;

    // Double-byte simple values (prefixed by CONT 0xFF)
    /// Infinity (CONT + NUL)
    pub const INF: [u8; 2] = [super::internal::CONT, NUL];
    /// Negative Infinity (CONT + TRU)
    pub const NINF: [u8; 2] = [super::internal::CONT, TRU];
    /// Positive NaN (CONT + FAL)
    pub const PNAN: [u8; 2] = [super::internal::CONT, FAL];
    /// Negative NaN (CONT + NAN)
    pub const NNAN: [u8; 2] = [super::internal::CONT, NAN];
}

/// Group 01: Complex Values
pub mod complex {
    /// Text (UTF-8)
    pub const TXT: u8 = 0xA0;
    /// Data (Binary)
    pub const DAT: u8 = 0x21;
    /// Duration
    pub const DUR: u8 = 0x22;
    /// UUID
    pub const UUID: u8 = 0xA3;
    /// DateTime (UTC)
    pub const DT: u8 = 0x24;
    /// ASCII-TEXT (7-bit)
    pub const ASCI: u8 = 0xA5;
    /// LocalDateTime
    pub const LDT: u8 = 0xA6;
    /// LocalDate
    pub const LD: u8 = 0x27;
    /// LocalTime
    pub const LT: u8 = 0x28;
    /// Signed Integer (LEB128)
    pub const SINT: u8 = 0xBD;
    /// Unsigned Integer (LEB128)
    pub const UINT: u8 = 0xBE;
    /// Float (f64)
    pub const FLT: u8 = 0x3F;
    /// Custom Decimal Float
    pub const CFLT: u8 = 0xB7;
}

/// Group 10: Parent Values
pub mod parent {
    /// Array
    pub const ARY: u8 = 0xC0;
    /// Object
    pub const OBJ: u8 = 0x41;
    /// Ordered Object
    pub const OOBJ: u8 = 0x42;
    /// Table
    pub const TBL: u8 = 0xC3;
    /// Graph
    pub const GRPH: u8 = 0xDD;
    /// Metadata
    pub const META: u8 = 0x5F;
}

/// Group 11: Internal / Control
pub mod internal {
    /// End of Value
    pub const EV: u8 = 0x60;
    /// End of Medium
    pub const EM: u8 = 0xF0;
    /// Continuation Byte
    pub const CONT: u8 = 0xFF;
}

/// Validates that a byte is a valid v4 marker with correct parity.
pub fn is_valid_v4_marker(byte: u8) -> bool {
    if !athena::byte_bit::is_even_parity(byte) {
        return false;
    }

    match byte {
        simple::NUL | simple::TRU | simple::FAL | simple::NAN => true,
        complex::TXT | complex::DAT | complex::DUR | complex::UUID | complex::DT | complex::ASCI | complex::LDT | complex::LD | complex::LT | complex::SINT | complex::UINT | complex::FLT | complex::CFLT => true,

        parent::ARY | parent::OBJ | parent::OOBJ | parent::TBL | parent::GRPH | parent::META => true,
        internal::EV | internal::EM | internal::CONT => true,
        _ => false,
    }
}
