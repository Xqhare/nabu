# `.xff` byte encoding reference table
The `.xff` byte encoding was created to be used to encode `.xff` files into bytes after the previously used `Windows-1252` control characters were found to be lacking for this specific use-case.

Control characters, or unprintable characters are now called Command characters.

It is based on `Windows-1252`. All differences can be found in the first 32 elements of the table below.

| DEC | HEX | BIN | Symbol | Description |
| :---: | :---: | :--------: | :---: | :-- |
| 0 | 00 | 00000000 | NUL | Null character |
| 1 | 01 | 00000001 | TXT | Text |
| 2 | 02 | 00000010 | NUM | Number |
| 3 | 03 | 00000011 | ARY | Array |
| 4 | 04 | 00000100 | OBJ | Object |
| 5 | 05 | 00000101 | CMD | Command |
| 6 | 06 | 00000110 | DAT | Data |
| 7 | 07 | 00000111 | -- | Unused |
| 8 | 08 | 00001000 | BS | Backspace
| 9 | 09 | 00001001 | HT | Horizontal Tab |
| 10 | 0A | 00001010 | LF | Line Feed
| 11 | 0B | 00001011 | VT | Vertical Tabulation |
| 12 | 0C | 00001100 | FF | Form Feed |
| 13 | 0D | 00001101 | CR | Carriage Return |
| 14 | 0E | 00001110 | TRU | True |
| 15 | 0F | 00001111 | FAL  | False |
| 16 | 10 | 00010000 | ESC | Escape  |
| 17 | 11 | 00010001 | -- | Unused |
| 18 | 12 | 00010010 | -- | Unused |
| 19 | 13 | 00010011 | -- | Unused | 
| 20 | 14 | 00010100 | -- | Unused |
| 21 | 15 | 00010101 | -- | Unused |
| 22 | 16 | 00010110 | -- | Unused |
| 23 | 17 | 00010111 | -- | Unused |
| 24 | 18 | 00011000 | -- | Unused |
| 25 | 19 | 00011001 | EM | End of medium |
| 26 | 1A | 00011010 | SEP | Separator |
| 27 | 1B | 00011011 | VS | Value Separator |
| 28 | 1C | 00011100 | FS | File Separator
| 29 | 1D | 00011101 | GS | Group Separator
| 30 | 1E | 00011110 | RS | Record Separator
| 31 | 1F | 00011111 | US | Unit Separator
| 32 | 20 | 00100000 | SP | Space
| 33 | 21 | 00100001 | ! | Exclamation mark
| 34 | 22 | 00100010 | " | Double quotes (or speech marks)
| 35 | 23 | 00100011 | # | Number sign
| 36 | 24 | 00100100 | $ | Dollar
| 37 | 25 | 00100101 | % | Per cent sign
| 38 | 26 | 00100110 | & | Ampersand
| 39 | 27 | 00100111 | ' | Single quote
| 40 | 28 | 00101000 | ( | Open parenthesis (or open bracket)
| 41 | 29 | 00101001 | ) | Close parenthesis (or close bracket)
| 42 | 2A | 00101010 | * | Asterisk
| 43 | 2B | 00101011 | + | Plus
| 44 | 2C | 00101100 | , | Comma
| 45 | 2D | 00101101 | - | Hyphen-minus
| 46 | 2E | 00101110 | . | Period, dot or full stop
| 47 | 2F | 00101111 | / | Slash or divide
| 48 | 30 | 00110000 | 0 | Zero
| 49 | 31 | 00110001 | 1 | One
| 50 | 32 | 00110010 | 2 | Two
| 51 | 33 | 00110011 | 3 | Three
| 52 | 34 | 00110100 | 4 | Four
| 53 | 35 | 00110101 | 5 | Five
| 54 | 36 | 00110110 | 6 | Six
| 55 | 37 | 00110111 | 7 | Seven
| 56 | 38 | 00111000 | 8 | Eight
| 57 | 39 | 00111001 | 9 | Nine
| 58 | 3A | 00111010 | : | Colon
| 59 | 3B | 00111011 | ; | Semicolon
| 60 | 3C | 00111100 | < | Less than (or open angled bracket)
| 61 | 3D | 00111101 | = | Equals
| 62 | 3E | 00111110 | > | Greater than (or close angled bracket)
| 63 | 3F | 00111111 | ? | Question mark
| 64 | 40 | 01000000 | @ | At sign
| 65 | 41 | 01000001 | A | Uppercase A
| 66 | 42 | 01000010 | B | Uppercase B
| 67 | 43 | 01000011 | C | Uppercase C
| 68 | 44 | 01000100 | D | Uppercase D
| 69 | 45 | 01000101 | E | Uppercase E
| 70 | 46 | 01000110 | F | Uppercase F
| 71 | 47 | 01000111 | G | Uppercase G
| 72 | 48 | 01001000 | H | Uppercase H
| 73 | 49 | 01001001 | I | Uppercase I
| 74 | 4A | 01001010 | J | Uppercase J
| 75 | 4B | 01001011 | K | Uppercase K
| 76 | 4C | 01001100 | L | Uppercase L
| 77 | 4D | 01001101 | M | Uppercase M
| 78 | 4E | 01001110 | N | Uppercase N
| 79 | 4F | 01001111 | O | Uppercase O
| 80 | 50 | 01010000 | P | Uppercase P
| 81 | 51 | 01010001 | Q | Uppercase Q
| 82 | 52 | 01010010 | R | Uppercase R
| 83 | 53 | 01010011 | S | Uppercase S
| 84 | 54 | 01010100 | T | Uppercase T
| 85 | 55 | 01010101 | U | Uppercase U
| 86 | 56 | 01010110 | V | Uppercase V
| 87 | 57 | 01010111 | W | Uppercase W
| 88 | 58 | 01011000 | X | Uppercase X
| 89 | 59 | 01011001 | Y | Uppercase Y
| 90 | 5A | 01011010 | Z | Uppercase Z
| 91 | 5B | 01011011 | [ | Opening bracket
| 92 | 5C | 01011100 | \ | Backslash
| 93 | 5D | 01011101 | ] | Closing bracket
| 94 | 5E | 01011110 | ^ | Caret - circumflex
| 95 | 5F | 01011111 | _ | Underscore
| 96 | 60 | 01100000 | ` | Grave | accent
| 97 | 61 | 01100001 | a | Lowercase a
| 98 | 62 | 01100010 | b | Lowercase b
| 99 | 63 | 01100011 | c | Lowercase c
| 100 | 64 | 01100100 | d | Lowercase d
| 101 | 65 | 01100101 | e | Lowercase e
| 102 | 66 | 01100110 | f | Lowercase f
| 103 | 67 | 01100111 | g | Lowercase g
| 104 | 68 | 01101000 | h | Lowercase h
| 105 | 69 | 01101001 | i | Lowercase i
| 106 | 6A | 01101010 | j | Lowercase j
| 107 | 6B | 01101011 | k | Lowercase k
| 108 | 6C | 01101100 | l | Lowercase l
| 109 | 6D | 01101101 | m | Lowercase m
| 110 | 6E | 01101110 | n | Lowercase n
| 111 | 6F | 01101111 | o | Lowercase o
| 112 | 70 | 01110000 | p | Lowercase p
| 113 | 71 | 01110001 | q | Lowercase q
| 114 | 72 | 01110010 | r | Lowercase r
| 115 | 73 | 01110011 | s | Lowercase s
| 116 | 74 | 01110100 | t | Lowercase t
| 117 | 75 | 01110101 | u | Lowercase u
| 118 | 76 | 01110110 | v | Lowercase v
| 119 | 77 | 01110111 | w | Lowercase w
| 120 | 78 | 01111000 | x | Lowercase x
| 121 | 79 | 01111001 | y | Lowercase y
| 122 | 7A | 01111010 | z | Lowercase z
| 123 | 7B | 01111011 | { | Opening brace
| 124 | 7C | 01111100 | \| | Vertical bar
| 125 | 7D | 01111101 | } | Closing brace
| 126 | 7E | 01111110 | ~ | Equivalency sign - tilde
| 127 | 7F | 01111111 | DEL | Delete
| 128 | 80 | 10000000 | € | Euro sign
| 129 | 81 | 10000001 | -- | Unused
| 130 | 82 | 10000010 | ‚ | Single low-9 quotation mark
| 131 | 83 | 10000011 | ƒ | Latin small letter f with hook
| 132 | 84 | 10000100 | „ | Double low-9 quotation mark
| 133 | 85 | 10000101 | … | Horizontal ellipsis
| 134 | 86 | 10000110 | † | Dagger
| 135 | 87 | 10000111 | ‡ | Double dagger
| 136 | 88 | 10001000 | ˆ | Modifier letter circumflex accent
| 137 | 89 | 10001001 | ‰ | Per mille sign
| 138 | 8A | 10001010 | Š | Latin capital letter S with caron
| 139 | 8B | 10001011 | ‹ | Single left-pointing angle quotation
| 140 | 8C | 10001100 | Œ | Latin capital ligature OE
| 141 | 8D | 10001101 | | Unused
| 142 | 8E | 10001110 | Ž | Latin capital letter Z with caron
| 143 | 8F | 10001111 | -- | Unused
| 144 | 90 | 10010000 | -- | Unused
| 145 | 91 | 10010001 | ‘ | Left single quotation mark
| 146 | 92 | 10010010 | ’ | Right single quotation mark
| 147 | 93 | 10010011 | “ | Left double quotation mark
| 148 | 94 | 10010100 | ” | Right double quotation mark
| 149 | 95 | 10010101 | • | Bullet
| 150 | 96 | 10010110 | – | En dash
| 151 | 97 | 10010111 | — | Em dash
| 152 | 98 | 10011000 | ˜ | Small tilde
| 153 | 99 | 10011001 | ™ | Trade mark sign
| 154 | 9A | 10011010 | š | Latin small letter S with caron
| 155 | 9B | 10011011 | › | Single right-pointing angle quotation mark
| 156 | 9C | 10011100 | œ | Latin small ligature oe
| 157 | 9D | 10011101 | -- | Unused
| 158 | 9E | 10011110 | ž | Latin small letter z with caron
| 159 | 9F | 10011111 | Ÿ | Latin capital letter Y with diaeresis
| 160 | A0 | 10100000 | NBSP | Non-breaking space
| 161 | A1 | 10100001 | ¡ | Inverted exclamation mark
| 162 | A2 | 10100010 | ¢ | Cent sign
| 163 | A3 | 10100011 | £ | Pound sign
| 164 | A4 | 10100100 | ¤ | Currency sign
| 165 | A5 | 10100101 | ¥ | Yen sign
| 166 | A6 | 10100110 | ¦ | Pipe, broken vertical bar
| 167 | A7 | 10100111 | § | Section sign
| 168 | A8 | 10101000 | ¨ | Spacing diaeresis - umlaut
| 169 | A9 | 10101001 | © | Copyright sign
| 170 | AA | 10101010 | ª | Feminine ordinal indicator
| 171 | AB | 10101011 | « | Left double angle quotes
| 172 | AC | 10101100 | ¬ | Negation
| 173 | AD | 10101101 | SHY | Soft hyphen
| 174 | AE | 10101110 | ® | Registered trade mark sign
| 175 | AF | 10101111 | ¯ | Spacing macron - over-line
| 176 | B0 | 10110000 | ° | Degree sign
| 177 | B1 | 10110001 | ± | Plus-or-minus sign
| 178 | B2 | 10110010 | ² | Superscript two - squared
| 179 | B3 | 10110011 | ³ | Superscript three - cubed
| 180 | B4 | 10110100 | ´ | Acute accent - spacing acute
| 181 | B5 | 10110101 | µ | Micro sign
| 182 | B6 | 10110110 | ¶ | Pilcrow sign - paragraph sign
| 183 | B7 | 10110111 | · | Middle dot - Georgian comma
| 184 | B8 | 10111000 | ¸ | Spacing cedilla
| 185 | B9 | 10111001 | ¹ | Superscript one
| 186 | BA | 10111010 | º | Masculine ordinal indicator
| 187 | BB | 10111011 | » | Right double angle quotes
| 188 | BC | 10111100 | ¼ | Fraction one quarter
| 189 | BD | 10111101 | ½ | Fraction one half
| 190 | BE | 10111110 | ¾ | Fraction three quarters
| 191 | BF | 10111111 | ¿ | Inverted question mark
| 192 | C0 | 11000000 | À | Latin capital letter A with grave
| 193 | C1 | 11000001 | Á | Latin capital letter A with acute
| 194 | C2 | 11000010 | Â | Latin capital letter A with circumflex
| 195 | C3 | 11000011 | Ã | Latin capital letter A with tilde
| 196 | C4 | 11000100 | Ä | Latin capital letter A with diaeresis
| 197 | C5 | 11000101 | Å | Latin capital letter A with ring above
| 198 | C6 | 11000110 | Æ | Latin capital letter AE
| 199 | C7 | 11000111 | Ç | Latin capital letter C with cedilla
| 200 | C8 | 11001000 | È | Latin capital letter E with grave
| 201 | C9 | 11001001 | É | Latin capital letter E with acute
| 202 | CA | 11001010 | Ê | Latin capital letter E with circumflex
| 203 | CB | 11001011 | Ë | Latin capital letter E with diaeresis
| 204 | CC | 11001100 | Ì | Latin capital letter I with grave
| 205 | CD | 11001101 | Í | Latin capital letter I with acute
| 206 | CE | 11001110 | Î | Latin capital letter I with circumflex
| 207 | CF | 11001111 | Ï | Latin capital letter I with diaeresis
| 208 | D0 | 11010000 | Ð | Latin capital letter ETH
| 209 | D1 | 11010001 | Ñ | Latin capital letter N with tilde
| 210 | D2 | 11010010 | Ò | Latin capital letter O with grave
| 211 | D3 | 11010011 | Ó | Latin capital letter O with acute
| 212 | D4 | 11010100 | Ô | Latin capital letter O with circumflex
| 213 | D5 | 11010101 | Õ | Latin capital letter O with tilde
| 214 | D6 | 11010110 | Ö | Latin capital letter O with diaeresis
| 215 | D7 | 11010111 | × | Multiplication sign
| 216 | D8 | 11011000 | Ø | Latin capital letter O with slash
| 217 | D9 | 11011001 | Ù | Latin capital letter U with grave
| 218 | DA | 11011010 | Ú | Latin capital letter U with acute
| 219 | DB | 11011011 | Û | Latin capital letter U with circumflex
| 220 | DC | 11011100 | Ü | Latin capital letter U with diaeresis
| 221 | DD | 11011101 | Ý | Latin capital letter Y with acute
| 222 | DE | 11011110 | Þ | Latin capital letter THORN
| 223 | DF | 11011111 | ß | Latin small letter sharp s - ess-zed
| 224 | E0 | 11100000 | à | Latin small letter a with grave
| 225 | E1 | 11100001 | á | Latin small letter a with acute
| 226 | E2 | 11100010 | â | Latin small letter a with circumflex
| 227 | E3 | 11100011 | ã | Latin small letter a with tilde
| 228 | E4 | 11100100 | ä | Latin small letter a with diaeresis
| 229 | E5 | 11100101 | å | Latin small letter a with ring above
| 230 | E6 | 11100110 | æ | Latin small letter ae
| 231 | E7 | 11100111 | ç | Latin small letter c with cedilla
| 232 | E8 | 11101000 | è | Latin small letter e with grave
| 233 | E9 | 11101001 | é | Latin small letter e with acute
| 234 | EA | 11101010 | ê | Latin small letter e with circumflex
| 235 | EB | 11101011 | ë | Latin small letter e with diaeresis
| 236 | EC | 11101100 | ì | Latin small letter i with grave
| 237 | ED | 11101101 | í | Latin small letter i with acute
| 238 | EE | 11101110 | î | Latin small letter i with circumflex
| 239 | EF | 11101111 | ï | Latin small letter i with diaeresis
| 240 | F0 | 11110000 | ð | Latin small letter eth
| 241 | F1 | 11110001 | ñ | Latin small letter n with tilde
| 242 | F2 | 11110010 | ò | Latin small letter o with grave
| 243 | F3 | 11110011 | ó | Latin small letter o with acute
| 244 | F4 | 11110100 | ô | Latin small letter o with circumflex
| 245 | F5 | 11110101 | õ | Latin small letter o with tilde
| 246 | F6 | 11110110 | ö | Latin small letter o with diaeresis
| 247 | F7 | 11110111 | ÷ | Division sign
| 248 | F8 | 11111000 | ø | Latin small letter o with slash
| 249 | F9 | 11111001 | ù | Latin small letter u with grave
| 250 | FA | 11111010 | ú | Latin small letter u with acute
| 251 | FB | 11111011 | û | Latin small letter u with circumflex
| 252 | FC | 11111100 | ü | Latin small letter u with diaeresis
| 253 | FD | 11111101 | ý | Latin small letter y with acute
| 254 | FE | 11111110 | þ | Latin small letter thorn
| 255 | FF | 11111111 | ÿ | Latin small letter y with diaeresis
