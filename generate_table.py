import binascii

def is_even_parity(b):
    return bin(b).count('1') % 2 == 0

def get_group(b):
    return (b >> 5) & 0x03

def get_value_bits(b):
    return b & 0x1F

groups = {
    0: "Simple",
    1: "Complex",
    2: "Parent",
    3: "Internal"
}

v4_markers = {
    0x00: ("NUL", "Null / Infinity"),
    0x87: ("TRU", "True / Negative Infinity"),
    0x99: ("FAL", "False / Positive NaN"),
    0x1E: ("NAN", "NaN (Quiet) / Negative NaN"),
    
    0xA0: ("TXT", "Text (UTF-8)"),
    0x21: ("DAT", "Data (Binary)"),
    0x22: ("DUR", "Duration"),
    0xA3: ("UUID", "UUID"),
    0x24: ("DT", "DateTime (UTC)"),
    0xA5: ("ASCI", "ASCII-TEXT (7-bit)"),
    0xA6: ("LDT", "LocalDateTime"),
    0x27: ("LD", "LocalDate"),
    0x28: ("LT", "LocalTime"),
    0xBD: ("SINT", "Signed Integer"),
    0xBE: ("UINT", "Unsigned Integer"),
    0x3F: ("FLT", "Float (f64)"),
    0xB7: ("CFLT", "Custom Decimal Float"),
    
    0xC0: ("ARY", "Array"),
    0x41: ("OBJ", "Object"),
    0x42: ("OOBJ", "Ordered Object"),
    0xC3: ("TBL", "Table"),
    0xDD: ("GRPH", "Graph"),
    0x5F: ("META", "Metadata"),
    
    0x60: ("EV", "End of Value"),
    0xF0: ("EM", "End of Medium"),
    0xFF: ("CONT", "Continuation Byte")
}

print("| DEC | HEX | FULL BIN | PARITY | GROUP | VALUE BITS | SYMBOL | DESC | GROUP NAME |")
print("| :---: | :---: | :--------: | :---: | :---: | :---: | :---: | :-- | :---: |")

for g_idx in range(4):
    for b in range(256):
        if is_even_parity(b) and get_group(b) == g_idx:
            symbol, desc = v4_markers.get(b, ("--", "Unused"))
            full_bin = f"{(b >> 7) & 1} {(b >> 5) & 3:02b} {b & 0x1F:05b}"
            print(f"| {b} | {b:02X} | {full_bin} | {int((b >> 7) & 1)} | {g_idx:02b} | {b & 0x1F:05b} | {symbol} | {desc} | {groups[g_idx]} |")
    if g_idx < 3:
        print("| | | | | | | | | |")
