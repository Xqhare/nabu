def calculate_parity_bit(val_7_bits):
    count = bin(val_7_bits).count('1')
    return 1 if count % 2 != 0 else 0

def get_group_name(group_bits):
    return ["Simple", "Complex", "Parent", "Internal"][group_bits]

markers = {
    # (Group, Value): (Symbol, Description)
    (0, 0): ("NUL", "Null"),
    (0, 1): ("INF", "Infinity"),
    (0, 2): ("NINF", "Negative Infinity"),
    (0, 4): ("NAN", "Not a Number"),
    (0, 5): ("TRU", "True"),
    (0, 6): ("FAL", "False"),

    # Complex - General (Top)
    (1, 0): ("TXT", "Text (UTF-8)"),
    (1, 1): ("DAT", "Data (Binary)"),
    (1, 2): ("DUR", "Duration"),
    (1, 3): ("UUID", "UUID"),
    
    # Complex - Mathematical (Bottom)
    (1, 28): ("SINT", "Signed Integer (LEB128)"),
    (1, 29): ("UINT", "Unsigned Integer (LEB128)"),
    (1, 30): ("FLT", "Float (f64)"),
    (1, 31): ("DT", "Date and Time"),

    (2, 0): ("ARY", "Array"),
    (2, 1): ("OBJ", "Object"),
    (2, 2): ("OOBJ", "Ordered Object"),
    (2, 3): ("TBL", "Table"),
    (2, 31): ("META", "Metadata"),

    (3, 0): ("EV", "End of Value"),
    (3, 16): ("EM", "End of Medium"),
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
