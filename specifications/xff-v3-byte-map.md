# XFF v3 Byte-Map Reference

This document provides the complete 8-bit byte mapping for all markers and type identifiers in the `.xff` version 3 specification.

## Overview
The XFF v3 byte-map is a part of the two-tier integrity system. Every marker byte defined below maintains **even parity** via its most significant bit (MSB).

Control markers and type identifiers are grouped into four categories using bits 6 and 5 (the "Group Bits").

## Parity Rule
- **MSB (Bit 7)**: Parity bit. Adjusted so the total number of `1` bits in the byte is even.
- **Group Bits (Bits 6-5)**:
    - `00`: Simple Values
    - `01`: Complex Values
    - `10`: Parent Values
    - `11`: Internal / Control
- **Value Bits (Bits 4-0)**: 5-bit unique identifier for the marker.

### Group Subdivisions

The high-nibble of the marker’s HEX value can be used to quickly identify each group and the logical subdivisions within that group:

- **Simple Values (Group 00)**: Logical (`0`/`8`) vs. Maths (`1`/`9`)
- **Complex Values (Group 01)**: Normal (`2`/`A`) vs. Maths (`3`/`B`)
- **Parent Values (Group 10)**: Normal (`4`/`C`) vs. Special (`5`/`D`)
- **Internal / Control (Group 11)**: Structural (`6`/`E`) vs. Foundational (`7`/`F`)

This taxonomy allows for bit-level optimization and easy expansion within each category.

## Marker Table

| DEC | HEX | FULL BIN | PARITY | GROUP | VALUE BITS | SYMBOL | DESC | GROUP NAME |
| :---: | :---: | :--------: | :---: | :---: | :---: | :---: | :-- | :---: |
| 0 | 00 | 0 00 00000 | 0 | 00 | 00000 | NUL | Null | Simple |
| 129 | 81 | 1 00 00001 | 1 | 00 | 00001 | -- | Unused | Simple |
| 130 | 82 | 1 00 00010 | 1 | 00 | 00010 | -- | Unused | Simple |
| 3 | 03 | 0 00 00011 | 0 | 00 | 00011 | -- | Unused | Simple |
| 132 | 84 | 1 00 00100 | 1 | 00 | 00100 | -- | Unused | Simple |
| 5 | 05 | 0 00 00101 | 0 | 00 | 00101 | TRU | True | Simple |
| 6 | 06 | 0 00 00110 | 0 | 00 | 00110 | FAL | False | Simple |
| 135 | 87 | 1 00 00111 | 1 | 00 | 00111 | -- | Unused | Simple |
| 136 | 88 | 1 00 01000 | 1 | 00 | 01000 | -- | Unused | Simple |
| 9 | 09 | 0 00 01001 | 0 | 00 | 01001 | -- | Unused | Simple |
| 10 | 0A | 0 00 01010 | 0 | 00 | 01010 | -- | Unused | Simple |
| 139 | 8B | 1 00 01011 | 1 | 00 | 01011 | -- | Unused | Simple |
| 12 | 0C | 0 00 01100 | 0 | 00 | 01100 | -- | Unused | Simple |
| 141 | 8D | 1 00 01101 | 1 | 00 | 01101 | -- | Unused | Simple |
| 142 | 8E | 1 00 01110 | 1 | 00 | 01110 | -- | Unused | Simple |
| 15 | 0F | 0 00 01111 | 0 | 00 | 01111 | -- | Unused | Simple |
| 144 | 90 | 1 00 10000 | 1 | 00 | 10000 | -- | Unused | Simple |
| 17 | 11 | 0 00 10001 | 0 | 00 | 10001 | INF | Infinity | Simple |
| 18 | 12 | 0 00 10010 | 0 | 00 | 10010 | NINF | Negative Infinity | Simple |
| 147 | 93 | 1 00 10011 | 1 | 00 | 10011 | -- | Unused | Simple |
| 20 | 14 | 0 00 10100 | 0 | 00 | 10100 | NAN | Not a Number | Simple |
| 149 | 95 | 1 00 10101 | 1 | 00 | 10101 | -- | Unused | Simple |
| 150 | 96 | 1 00 10110 | 1 | 00 | 10110 | -- | Unused | Simple |
| 23 | 17 | 0 00 10111 | 0 | 00 | 10111 | -- | Unused | Simple |
| 24 | 18 | 0 00 11000 | 0 | 00 | 11000 | -- | Unused | Simple |
| 153 | 99 | 1 00 11001 | 1 | 00 | 11001 | -- | Unused | Simple |
| 154 | 9A | 1 00 11010 | 1 | 00 | 11010 | -- | Unused | Simple |
| 27 | 1B | 0 00 11011 | 0 | 00 | 11011 | -- | Unused | Simple |
| 156 | 9C | 1 00 11100 | 1 | 00 | 11100 | -- | Unused | Simple |
| 29 | 1D | 0 00 11101 | 0 | 00 | 11101 | -- | Unused | Simple |
| 30 | 1E | 0 00 11110 | 0 | 00 | 11110 | -- | Unused | Simple |
| 159 | 9F | 1 00 11111 | 1 | 00 | 11111 | -- | Unused | Simple |
| | | | | | | | | |
| 160 | A0 | 1 01 00000 | 1 | 01 | 00000 | TXT | Text (UTF-8) | Complex |
| 33 | 21 | 0 01 00001 | 0 | 01 | 00001 | DAT | Data (Binary) | Complex |
| 34 | 22 | 0 01 00010 | 0 | 01 | 00010 | DUR | Duration | Complex |
| 163 | A3 | 1 01 00011 | 1 | 01 | 00011 | UUID | UUID | Complex |
| 36 | 24 | 0 01 00100 | 0 | 01 | 00100 | DT | DateTime | Complex |
| 165 | A5 | 1 01 00101 | 1 | 01 | 00101 | -- | Unused | Complex |
| 166 | A6 | 1 01 00110 | 1 | 01 | 00110 | -- | Unused | Complex |
| 39 | 27 | 0 01 00111 | 0 | 01 | 00111 | -- | Unused | Complex |
| 40 | 28 | 0 01 01000 | 0 | 01 | 01000 | -- | Unused | Complex |
| 169 | A9 | 1 01 01001 | 1 | 01 | 01001 | -- | Unused | Complex |
| 170 | AA | 1 01 01010 | 1 | 01 | 01010 | -- | Unused | Complex |
| 43 | 2B | 0 01 01011 | 0 | 01 | 01011 | -- | Unused | Complex |
| 172 | AC | 1 01 01100 | 1 | 01 | 01100 | -- | Unused | Complex |
| 45 | 2D | 0 01 01101 | 0 | 01 | 01101 | -- | Unused | Complex |
| 46 | 2E | 0 01 01110 | 0 | 01 | 01110 | -- | Unused | Complex |
| 175 | AF | 1 01 01111 | 1 | 01 | 01111 | -- | Unused | Complex |
| 48 | 30 | 0 01 10000 | 0 | 01 | 10000 | -- | Unused | Complex |
| 177 | B1 | 1 01 10001 | 1 | 01 | 10001 | -- | Unused | Complex |
| 178 | B2 | 1 01 10010 | 1 | 01 | 10010 | -- | Unused | Complex |
| 51 | 33 | 0 01 10011 | 0 | 01 | 10011 | -- | Unused | Complex |
| 180 | B4 | 1 01 10100 | 1 | 01 | 10100 | -- | Unused | Complex |
| 53 | 35 | 0 01 10101 | 0 | 01 | 10101 | -- | Unused | Complex |
| 54 | 36 | 0 01 10110 | 0 | 01 | 10110 | -- | Unused | Complex |
| 183 | B7 | 1 01 10111 | 1 | 01 | 10111 | -- | Unused | Complex |
| 184 | B8 | 1 01 11000 | 1 | 01 | 11000 | -- | Unused | Complex |
| 57 | 39 | 0 01 11001 | 0 | 01 | 11001 | -- | Unused | Complex |
| 58 | 3A | 0 01 11010 | 0 | 01 | 11010 | -- | Unused | Complex |
| 187 | BB | 1 01 11011 | 1 | 01 | 11011 | -- | Unused | Complex |
| 60 | 3C | 0 01 11100 | 0 | 01 | 11100 | --- | Unused | Complex |
| 189 | BD | 1 01 11101 | 1 | 01 | 11101 | SINT | Signed Integer (LEB128) | Complex |
| 190 | BE | 1 01 11110 | 1 | 01 | 11110 | UINT | Unsigned Integer (LEB128) | Complex |
| 63 | 3F | 0 01 11111 | 0 | 01 | 11111 | FLT | Float (f64) | Complex |
| | | | | | | | | |
| 192 | C0 | 1 10 00000 | 1 | 10 | 00000 | ARY | Array | Parent |
| 65 | 41 | 0 10 00001 | 0 | 10 | 00001 | OBJ | Object | Parent |
| 66 | 42 | 0 10 00010 | 0 | 10 | 00010 | OOBJ | Ordered Object | Parent |
| 195 | C3 | 1 10 00011 | 1 | 10 | 00011 | TBL | Table | Parent |
| 68 | 44 | 0 10 00100 | 0 | 10 | 00100 | -- | Unused | Parent |
| 197 | C5 | 1 10 00101 | 1 | 10 | 00101 | -- | Unused | Parent |
| 198 | C6 | 1 10 00110 | 1 | 10 | 00110 | -- | Unused | Parent |
| 71 | 47 | 0 10 00111 | 0 | 10 | 00111 | -- | Unused | Parent |
| 72 | 48 | 0 10 01000 | 0 | 10 | 01000 | -- | Unused | Parent |
| 201 | C9 | 1 10 01001 | 1 | 10 | 01001 | -- | Unused | Parent |
| 202 | CA | 1 10 01010 | 1 | 10 | 01010 | -- | Unused | Parent |
| 75 | 4B | 0 10 01011 | 0 | 10 | 01011 | -- | Unused | Parent |
| 204 | CC | 1 10 01100 | 1 | 10 | 01100 | -- | Unused | Parent |
| 77 | 4D | 0 10 01101 | 0 | 10 | 01101 | -- | Unused | Parent |
| 78 | 4E | 0 10 01110 | 0 | 10 | 01110 | -- | Unused | Parent |
| 207 | CF | 1 10 01111 | 1 | 10 | 01111 | -- | Unused | Parent |
| 80 | 50 | 0 10 10000 | 0 | 10 | 10000 | -- | Unused | Parent |
| 209 | D1 | 1 10 10001 | 1 | 10 | 10001 | -- | Unused | Parent |
| 210 | D2 | 1 10 10010 | 1 | 10 | 10010 | -- | Unused | Parent |
| 83 | 53 | 0 10 10011 | 0 | 10 | 10011 | -- | Unused | Parent |
| 212 | D4 | 1 10 10100 | 1 | 10 | 10100 | -- | Unused | Parent |
| 85 | 55 | 0 10 10101 | 0 | 10 | 10101 | -- | Unused | Parent |
| 86 | 56 | 0 10 10110 | 0 | 10 | 10110 | -- | Unused | Parent |
| 215 | D7 | 1 10 10111 | 1 | 10 | 10111 | -- | Unused | Parent |
| 216 | D8 | 1 10 11000 | 1 | 10 | 11000 | -- | Unused | Parent |
| 89 | 59 | 0 10 11001 | 0 | 10 | 11001 | -- | Unused | Parent |
| 90 | 5A | 0 10 11010 | 0 | 10 | 11010 | -- | Unused | Parent |
| 219 | DB | 1 10 11011 | 1 | 10 | 11011 | -- | Unused | Parent |
| 92 | 5C | 0 10 11100 | 0 | 10 | 11100 | -- | Unused | Parent |
| 221 | DD | 1 10 11101 | 1 | 10 | 11101 | -- | Unused | Parent |
| 222 | DE | 1 10 11110 | 1 | 10 | 11110 | -- | Unused | Parent |
| 95 | 5F | 0 10 11111 | 0 | 10 | 11111 | META | Metadata | Parent |
| | | | | | | | | |
| 96 | 60 | 0 11 00000 | 0 | 11 | 00000 | EV | End of Value | Internal |
| 225 | E1 | 1 11 00001 | 1 | 11 | 00001 | -- | Unused | Internal |
| 226 | E2 | 1 11 00010 | 1 | 11 | 00010 | -- | Unused | Internal |
| 99 | 63 | 0 11 00011 | 0 | 11 | 00011 | -- | Unused | Internal |
| 228 | E4 | 1 11 00100 | 1 | 11 | 00100 | -- | Unused | Internal |
| 101 | 65 | 0 11 00101 | 0 | 11 | 00101 | -- | Unused | Internal |
| 102 | 66 | 0 11 00110 | 0 | 11 | 00110 | -- | Unused | Internal |
| 231 | E7 | 1 11 00111 | 1 | 11 | 00111 | -- | Unused | Internal |
| 232 | E8 | 1 11 01000 | 1 | 11 | 01000 | -- | Unused | Internal |
| 105 | 69 | 0 11 01001 | 0 | 11 | 01001 | -- | Unused | Internal |
| 106 | 6A | 0 11 01010 | 0 | 11 | 01010 | -- | Unused | Internal |
| 235 | EB | 1 11 01011 | 1 | 11 | 01011 | -- | Unused | Internal |
| 108 | 6C | 0 11 01100 | 0 | 11 | 01100 | -- | Unused | Internal |
| 237 | ED | 1 11 01101 | 1 | 11 | 01101 | -- | Unused | Internal |
| 238 | EE | 1 11 01110 | 1 | 11 | 01110 | -- | Unused | Internal |
| 111 | 6F | 0 11 01111 | 0 | 11 | 01111 | -- | Unused | Internal |
| 240 | F0 | 1 11 10000 | 1 | 11 | 10000 | EM | End of Medium | Internal |
| 113 | 71 | 0 11 10001 | 0 | 11 | 10001 | -- | Unused | Internal |
| 114 | 72 | 0 11 10010 | 0 | 11 | 10010 | -- | Unused | Internal |
| 243 | F3 | 1 11 10011 | 1 | 11 | 10011 | -- | Unused | Internal |
| 116 | 74 | 0 11 10100 | 0 | 11 | 10100 | -- | Unused | Internal |
| 245 | F5 | 1 11 10101 | 1 | 11 | 10101 | -- | Unused | Internal |
| 246 | F6 | 1 11 10110 | 1 | 11 | 10110 | -- | Unused | Internal |
| 119 | 77 | 0 11 10111 | 0 | 11 | 10111 | -- | Unused | Internal |
| 120 | 78 | 0 11 11000 | 0 | 11 | 11000 | -- | Unused | Internal |
| 249 | F9 | 1 11 11001 | 1 | 11 | 11001 | -- | Unused | Internal |
| 250 | FA | 1 11 11010 | 1 | 11 | 11010 | -- | Unused | Internal |
| 123 | 7B | 0 11 11011 | 0 | 11 | 11011 | -- | Unused | Internal |
| 252 | FC | 1 11 11100 | 1 | 11 | 11100 | -- | Unused | Internal |
| 125 | 7D | 0 11 11101 | 0 | 11 | 11101 | -- | Unused | Internal |
| 126 | 7E | 0 11 11110 | 0 | 11 | 11110 | -- | Unused | Internal |
| 255 | FF | 1 11 11111 | 1 | 11 | 11111 | CONT | Continuation Byte | Internal |
| | | | | | | | | |
