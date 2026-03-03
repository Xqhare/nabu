# XFF v3 Byte-Map Reference

This document provides the complete 8-bit byte mapping for all markers and type identifiers in the `.xff` version 3 specification.

## Overview
The XFF v3 byte-map is designed with a two-tier integrity system. Every marker byte defined below maintains **even parity** via its most significant bit (MSB).

Control markers and type identifiers are grouped into four categories using bits 6 and 5 (the "Group Bits").

## Parity Rule
- **MSB (Bit 7)**: Parity bit. Adjusted so the total number of `1` bits in the byte is even.
- **Group Bits (Bits 6-5)**:
    - `00`: Simple Values
    - `01`: Complex Values
    - `10`: Parent Values
    - `11`: Internal / Control
- **Value Bits (Bits 4-0)**: 5-bit unique identifier for the marker.

## Marker Table

| DEC | HEX | BIN | Symbol | Description | Group |
| :---: | :---: | :--------: | :---: | :-- | :---: |
| 0 | 00 | 00000000 | NUL | Null | Simple |
| 129 | 81 | 10000001 | INF | Infinity | Simple |
| 130 | 82 | 10000010 | NINF | Negative Infinity | Simple |
| 3 | 03 | 00000011 | -- | Unused | Simple |
| 132 | 84 | 10000100 | NAN | Not a Number | Simple |
| 5 | 05 | 00000101 | TRU | True | Simple |
| 6 | 06 | 00000110 | FAL | False | Simple |
| 135 | 87 | 10000111 | -- | Unused | Simple |
| 136 | 88 | 10001000 | -- | Unused | Simple |
| 9 | 09 | 00001001 | -- | Unused | Simple |
| 10 | 0A | 00001010 | -- | Unused | Simple |
| 139 | 8B | 10001011 | -- | Unused | Simple |
| 12 | 0C | 00001100 | -- | Unused | Simple |
| 141 | 8D | 10001101 | -- | Unused | Simple |
| 142 | 8E | 10001110 | -- | Unused | Simple |
| 15 | 0F | 00001111 | -- | Unused | Simple |
| 144 | 90 | 10010000 | -- | Unused | Simple |
| 17 | 11 | 00010001 | -- | Unused | Simple |
| 18 | 12 | 00010010 | -- | Unused | Simple |
| 147 | 93 | 10010011 | -- | Unused | Simple |
| 20 | 14 | 00010100 | -- | Unused | Simple |
| 149 | 95 | 10010101 | -- | Unused | Simple |
| 150 | 96 | 10010110 | -- | Unused | Simple |
| 23 | 17 | 00010111 | -- | Unused | Simple |
| 24 | 18 | 00011000 | -- | Unused | Simple |
| 153 | 99 | 10011001 | -- | Unused | Simple |
| 154 | 9A | 10011010 | -- | Unused | Simple |
| 27 | 1B | 00011011 | -- | Unused | Simple |
| 156 | 9C | 10011100 | -- | Unused | Simple |
| 29 | 1D | 00011101 | -- | Unused | Simple |
| 30 | 1E | 00011110 | -- | Unused | Simple |
| 159 | 9F | 10011111 | -- | Unused | Simple |
| 160 | A0 | 10100000 | TXT | Text (UTF-8) | Complex |
| 33 | 21 | 00100001 | DAT | Data (Binary) | Complex |
| 34 | 22 | 00100010 | SINT | Signed Integer (LEB128) | Complex |
| 163 | A3 | 10100011 | FLT | Float (f64) | Complex |
| 36 | 24 | 00100100 | UINT | Unsigned Integer (LEB128) | Complex |
| 165 | A5 | 10100101 | DUR | Duration | Complex |
| 166 | A6 | 10100110 | UUID | UUID | Complex |
| 39 | 27 | 00100111 | DT | Date and Time | Complex |
| 40 | 28 | 00101000 | -- | Unused | Complex |
| 169 | A9 | 10101001 | -- | Unused | Complex |
| 170 | AA | 10101010 | -- | Unused | Complex |
| 43 | 2B | 00101011 | -- | Unused | Complex |
| 172 | AC | 10101100 | -- | Unused | Complex |
| 45 | 2D | 00101101 | -- | Unused | Complex |
| 46 | 2E | 00101110 | -- | Unused | Complex |
| 175 | AF | 10101111 | -- | Unused | Complex |
| 48 | 30 | 00110000 | -- | Unused | Complex |
| 177 | B1 | 10110001 | -- | Unused | Complex |
| 178 | B2 | 10110010 | -- | Unused | Complex |
| 51 | 33 | 00110011 | -- | Unused | Complex |
| 180 | B4 | 10110100 | -- | Unused | Complex |
| 53 | 35 | 00110101 | -- | Unused | Complex |
| 54 | 36 | 00110110 | -- | Unused | Complex |
| 183 | B7 | 10110111 | -- | Unused | Complex |
| 184 | B8 | 10111000 | -- | Unused | Complex |
| 57 | 39 | 00111001 | -- | Unused | Complex |
| 58 | 3A | 00111010 | -- | Unused | Complex |
| 187 | BB | 10111011 | -- | Unused | Complex |
| 60 | 3C | 00111100 | -- | Unused | Complex |
| 189 | BD | 10111101 | -- | Unused | Complex |
| 190 | BE | 10111110 | -- | Unused | Complex |
| 63 | 3F | 00111111 | -- | Unused | Complex |
| 192 | C0 | 11000000 | ARY | Array | Parent |
| 65 | 41 | 01000001 | OBJ | Object | Parent |
| 66 | 42 | 01000010 | OOBJ | Ordered Object | Parent |
| 195 | C3 | 11000011 | TBL | Table | Parent |
| 68 | 44 | 01000100 | -- | Unused | Parent |
| 197 | C5 | 11000101 | -- | Unused | Parent |
| 198 | C6 | 11000110 | -- | Unused | Parent |
| 71 | 47 | 01000111 | -- | Unused | Parent |
| 72 | 48 | 01001000 | -- | Unused | Parent |
| 201 | C9 | 11001001 | -- | Unused | Parent |
| 202 | CA | 11001010 | -- | Unused | Parent |
| 75 | 4B | 01001011 | -- | Unused | Parent |
| 204 | CC | 11001100 | -- | Unused | Parent |
| 77 | 4D | 01001101 | -- | Unused | Parent |
| 78 | 4E | 01001110 | -- | Unused | Parent |
| 207 | CF | 11001111 | -- | Unused | Parent |
| 80 | 50 | 01010000 | -- | Unused | Parent |
| 209 | D1 | 11010001 | -- | Unused | Parent |
| 210 | D2 | 11010010 | -- | Unused | Parent |
| 83 | 53 | 01010011 | -- | Unused | Parent |
| 212 | D4 | 11010100 | -- | Unused | Parent |
| 85 | 55 | 01010101 | -- | Unused | Parent |
| 86 | 56 | 01010110 | -- | Unused | Parent |
| 215 | D7 | 11010111 | -- | Unused | Parent |
| 216 | D8 | 11011000 | -- | Unused | Parent |
| 89 | 59 | 01011001 | -- | Unused | Parent |
| 90 | 5A | 01011010 | -- | Unused | Parent |
| 219 | DB | 11011011 | -- | Unused | Parent |
| 92 | 5C | 01011100 | -- | Unused | Parent |
| 221 | DD | 11011101 | -- | Unused | Parent |
| 222 | DE | 11011110 | -- | Unused | Parent |
| 95 | 5F | 01011111 | META | Metadata | Parent |
| 96 | 60 | 01100000 | EV | End of Value | Internal |
| 225 | E1 | 11100001 | -- | Unused | Internal |
| 226 | E2 | 11100010 | -- | Unused | Internal |
| 99 | 63 | 01100011 | -- | Unused | Internal |
| 228 | E4 | 11100100 | -- | Unused | Internal |
| 101 | 65 | 01100101 | -- | Unused | Internal |
| 102 | 66 | 01100110 | -- | Unused | Internal |
| 231 | E7 | 11100111 | -- | Unused | Internal |
| 232 | E8 | 11101000 | -- | Unused | Internal |
| 105 | 69 | 01101001 | -- | Unused | Internal |
| 106 | 6A | 01101010 | -- | Unused | Internal |
| 235 | EB | 11101011 | -- | Unused | Internal |
| 108 | 6C | 01101100 | -- | Unused | Internal |
| 237 | ED | 11101101 | -- | Unused | Internal |
| 238 | EE | 11101110 | -- | Unused | Internal |
| 111 | 6F | 01101111 | -- | Unused | Internal |
| 240 | F0 | 11110000 | EM | End of Medium | Internal |
| 113 | 71 | 01110001 | -- | Unused | Internal |
| 114 | 72 | 01110010 | -- | Unused | Internal |
| 243 | F3 | 11110011 | -- | Unused | Internal |
| 116 | 74 | 01110100 | -- | Unused | Internal |
| 245 | F5 | 11110101 | -- | Unused | Internal |
| 246 | F6 | 11110110 | -- | Unused | Internal |
| 119 | 77 | 01110111 | -- | Unused | Internal |
| 120 | 78 | 01111000 | -- | Unused | Internal |
| 249 | F9 | 11111001 | -- | Unused | Internal |
| 250 | FA | 11111010 | -- | Unused | Internal |
| 123 | 7B | 01111011 | -- | Unused | Internal |
| 252 | FC | 11111100 | -- | Unused | Internal |
| 125 | 7D | 01111101 | -- | Unused | Internal |
| 126 | 7E | 01111110 | -- | Unused | Internal |
| 255 | FF | 11111111 | CONT | Continuation Byte | Internal |
