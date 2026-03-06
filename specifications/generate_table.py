def calculate_parity_bit(val_7_bits):
    count = bin(val_7_bits).count('1')
    return 1 if count % 2 != 0 else 0

def get_group_name(group_bits):
    return ["Simple", "Complex", "Parent", "Internal"][group_bits]

markers = {
    # (Group, Value): (Symbol, Description)
    
    # --- Group 00: Simple ---
    # Logical (0/8)
    (0, 0): ("NUL", "Null"),
    (0, 5): ("TRU", "True"),
    (0, 6): ("FAL", "False"),
    # Maths (1/9)
    (0, 17): ("INF", "Infinity"),
    (0, 18): ("NINF", "Negative Infinity"),
    (0, 20): ("NAN", "Not a Number"),

    # --- Group 01: Complex ---
    # Normal (2/A)
    (1, 0): ("TXT", "Text (UTF-8)"),
    (1, 1): ("DAT", "Data (Binary)"),
    (1, 2): ("DUR", "Duration"),
    (1, 3): ("UUID", "UUID"),
    (1, 4): ("DT", "DateTime"),
    # Maths (3/B)
    (1, 29): ("SINT", "Signed Integer (LEB128)"),
    (1, 30): ("UINT", "Unsigned Integer (LEB128)"),
    (1, 31): ("FLT", "Float (f64)"),

    # --- Group 10: Parent ---
    # Normal (4/C)
    (2, 0): ("ARY", "Array"),
    (2, 1): ("OBJ", "Object"),
    (2, 2): ("OOBJ", "Ordered Object"),
    (2, 3): ("TBL", "Table"),
    # Special (5/D)
    (2, 31): ("META", "Metadata"),

    # --- Group 11: Internal ---
    # Structural (6/E)
    (3, 0): ("EV", "End of Value"),
    (3, 16): ("EM", "End of Medium"),
    # Foundational (7/F)
    (3, 31): ("CONT", "Continuation Byte"),
}

header = "| DEC | HEX | FULL BIN | PARITY | GROUP | VALUE BITS | SYMBOL | DESC | GROUP NAME |"
separator = "| :---: | :---: | :--------: | :---: | :---: | :---: | :---: | :-- | :---: |"

print(header)
print(separator)

for group in range(4):
    for value in range(32):
        group_bits = group
        val_7_bits = (group_bits << 5) | value
        parity_bit = calculate_parity_bit(val_7_bits)
        full_byte = (parity_bit << 7) | val_7_bits

        symbol, desc = markers.get((group, value), ("--", "Unused"))
        
        full_bin = format(full_byte, '08b')
        full_bin_formatted = f"{full_bin[0]} {full_bin[1:3]} {full_bin[3:]}"
        
        parity = str(parity_bit)
        group_str = format(group_bits, '02b')
        value_bits = format(value, '05b')
        
        row = f"| {full_byte} | {full_byte:02X} | {full_bin_formatted} | {parity} | {group_str} | {value_bits} | {symbol} | {desc} | {get_group_name(group)} |"
        print(row)
    # Double separate between groups
    print("| | | | | | | | | |")
