def calculate_parity_bit(val_7_bits):
    count = bin(val_7_bits).count('1')
    return 1 if count % 2 != 0 else 0

def get_group_name(group_bits):
    return ["Simple", "Complex", "Parent", "Internal"][group_bits]

# Markers as defined in v4.md
markers = {
    # Group 00: Simple
    0x00: ("NUL", "Null / Infinity"),
    0x87: ("TRU", "True / Negative Infinity"),
    0x99: ("FAL", "False / Positive NaN"),
    0x1E: ("NAN", "NaN (Quiet) / Negative NaN"),
    
    # Group 01: Complex
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
    
    # Group 10: Parent
    0xC0: ("ARY", "Array"),
    0x41: ("OBJ", "Object"),
    0x42: ("OOBJ", "Ordered Object"),
    0xC3: ("TBL", "Table"),
    0xDD: ("GRPH", "Graph"),
    0x5F: ("META", "Metadata"),
    
    # Group 11: Internal
    0x60: ("EV", "End of Value"),
    0xF0: ("EM", "End of Medium"),
    0xFF: ("CONT", "Continuation Byte"),
}

if __name__ == "__main__":
    print("# XFF v4 Byte-Map Reference")
    print("")
    print("This document provides the complete 8-bit byte mapping for all markers and type identifiers in the .xff version 4 specification.")
    print("")
    print("## Overview")
    print("The XFF v4 byte-map is a part of the two-tier integrity system. Every marker byte defined below maintains **even parity** via its most significant bit (MSB).")
    print("")
    print("Control markers and type identifiers are grouped into four categories using bits 6 and 5 (the Group Bits).")
    print("")
    print("## Parity Rule")
    print("- **MSB (Bit 7)**: Parity bit. Adjusted so the total number of 1 bits in the byte is even.")
    print("- **Group Bits (Bits 6-5)**:")
    print("    - 00: Simple Values")
    print("    - 01: Complex Values")
    print("    - 10: Parent Values")
    print("    - 11: Internal / Control")
    print("- **Value Bits (Bits 4-0)**: 5-bit unique identifier for the marker.")
    print("")
    print("### Group Subdivisions")
    print("")
    print("The high-nibble of the marker’s HEX value can be used to quickly identify each group and the logical subdivisions within that group:")
    print("")
    print("- **Simple Values (Group 00)**: Logical* (0/8) vs. Maths* (1/9) - *Loose alignment for Hamming distance*")
    print("- **Complex Values (Group 01)**: Normal (2/A) vs. Maths (3/B)")
    print("- **Parent Values (Group 10)**: Normal (4/C) vs. Special (5/D)")
    print("- **Internal / Control (Group 11)**: Structural (6/E) vs. Foundational (7/F)")
    print("")
    print("This taxonomy allows for bit-level optimization and easy expansion within each category.")
    print("")
    print("## Marker Table")
    print("")
    print("| DEC | HEX | FULL BIN | PARITY | GROUP | VALUE BITS | SYMBOL | DESC | GROUP NAME |")
    print("| :---: | :---: | :--------: | :---: | :---: | :---: | :---: | :-- | :---: |")
    
    last_group = -1
    for i in range(128):
        val_7_bits = i
        parity = calculate_parity_bit(val_7_bits)
        full_byte = (parity << 7) | val_7_bits
        
        group = (full_byte & 0x60) >> 5
        value_bits = full_byte & 0x1F
        
        full_bin = f"{full_byte:08b}"
        full_bin_formatted = f"{full_bin[:1]} {full_bin[1:3]} {full_bin[3:]}"
        
        group_str = f"{group:02b}"
        value_bits_str = f"{value_bits:05b}"
        
        if full_byte in markers:
            symbol, desc = markers[full_byte]
            symbol_str = f"**{symbol}**"
        else:
            symbol_str, desc = "--", "Unused"
            
        if group != last_group and last_group != -1:
            print("| | | | | | | | | |")
        
        last_group = group
        
        row = f"| {full_byte} | {full_byte:02X} | {full_bin_formatted} | {parity} | {group_str} | {value_bits_str} | {symbol_str} | {desc} | {get_group_name(group)} |"
        print(row)
