/// XFF v3 Marker Constants
///
/// Markers are even-parity bytes. The MSB is the parity bit.
// Simple Values
pub const NUL: u8 = 0x00;
pub const TRU: u8 = 0x05;
pub const FAL: u8 = 0x06;
pub const INF: u8 = 0x11;
pub const NINF: u8 = 0x12;
pub const NAN: u8 = 0x14;

// Complex Values
pub const TXT: u8 = 0xA0;
pub const DAT: u8 = 0x21;
pub const DUR: u8 = 0x22;
pub const UUID: u8 = 0xA3;
pub const DT: u8 = 0x24;
pub const SINT: u8 = 0xBD;
pub const UINT: u8 = 0xBE;
pub const FLT: u8 = 0x3F;

// Parent Values
pub const ARY: u8 = 0xC0;
pub const OBJ: u8 = 0x41;
pub const OOBJ: u8 = 0x42;
pub const TBL: u8 = 0xC3;
pub const META: u8 = 0x5F;

// Internal / Structural
pub const EV: u8 = 0x60;
pub const EM: u8 = 0xF0;

// File Signature
pub const MAGIC: [u8; 4] = [0x58, 0x46, 0x46, 0x56]; // 'XFFV'
