# Nabu

> [!warning]
> This is a hobby project. It is not intended nor ready to be used in production.

The overarching goal of this project is to create a rust library that can be used to build, read and write `.xff` files.
`.xff` files are going to be my attempt at a general purpose file format, with Nabu as a small wrapper around it, giving the opportunity for downstream projects to create their own files suiting their needs.
I also plan to try out the `Features` flag for the first time, and use that to extend the usability of `.xff` files for a wide range of use-cases.
The only one of these features that is planned, is a key-value store, but I am sure more will come to mind in due time.

## `.xff` specification v0
`.xff` stands for `xqhares file format` or `xqhared file format`, pronounced `squares file format` or `squared file format`.
Alternatively, it could be called `xtended file format` or `extended file format`.

The binary data in `.xff` is encoded in ASCII, specifically Windows-1252 and uses the Big-Endian byte-ordering.
Any mention of ASCII should be understood to be referring to Windows-1252 specifically. 
[See the reference table here.](#windows-1252-reference-table)

`.xff` is a very simple format. It uses a small subset of ASCII control characters to wrap arbitrary or ASCII data.
If used with ASCII data only, it can hold any String, Number or Control character and any amount of them.
The `.xff` format has no set maximum file size.

A `.xff`file starts with one byte (8 bits) of data, encoding the current version of the `.xff` specification.
The version number starts at 0 and increases by 1 each time a new version is added.
Any implementation of this specification should be able to read any version of the specification, but may choose to only support a specific set of versions or version.

![Chart of the composition of a `.xff` file in token form.](pictures/xff-main-chart.jpeg)

### Strings and numbers
Strings and numbers have to be encoded in ASCII, or they have to be treated like any other escaped data.
An entry can be of any length.

The command character codes 8 through 13 are permissible in a `String` without the need to be escaped.
No command characters should be used if a `Number` is represented.

`STX` and `ETX` are the start and end of any text data or numerical data encoded in ASCII.
The `ETX` also provides an inbuilt check of the length of the data to validate it.
The text wrapped by it should be returned as a fully decoded `String` or appropriate `Number`.
The difference between `String` and `Number` is in that a valid `Number` is any valid number as according to the 2nd Edition of `ECMA-404` from December 2017, and any different content is a `String`.

![Chart of the composition of a character in token form.](pictures/xff-char-chart.jpeg)

### Escaped general data
`DLE`, or Data Link Escape is the data escape character. 
Any data following `DLE`, until another `DLE` is encountered, is considered part of a unified data-stream. This makes it possible to save any kind of data inside.

Directly after the opening `DLE` are 5 bytes (36 bits) that encode the length of the data in bytes, followed by the data itself.
This makes the largest possible continuous strip of roughly 1 terabyte of data.
Any data-stream larger than that needs to be split up into several `DLE` enclosed data-streams.

The closing `DLE` is not part of the data, but it serves as a check of the length of the data to validate it.
The data inside the escape should be stored as-is, meaning that the data inside it could be UTF-8 encoded or JPEG encoded for example.

### Command characters
All command characters should be returned to the caller, and any non-command characters should error.
To make all characters available to the caller, any command character should be escaped with `ESC`, and `ESC` should be escaped with `ESC` too.
This requirement makes it so that the only way to encode a singular `ESC` is with `ESC` `ESC` `ESC` `ESC`. Using 4 byte to encode one byte of data. 
Command characters should be saved together, if they are followed by another command character.

`EM` is the end of the `.xff` file and serves as a check that the entire `.xff` file has been read.

> [!important]
> Because of the way I decided to design the `.xff` specification, the control characters, as they are called in the ASCII standard, are not all valid command characters.
> Some non control characters are also valid command characters.

![Chart of the composition of a command character in token form.](pictures/xff-cmd-char-chart.jpeg)

### Windows-1252 reference table
The reference table below is a copy of the Windows-1252 reference table.
Please note that the decimal representation of the code-points is used as the value of the `ASCII Code` in this document. (e.g. `ASCII code 32` is `Space`)

| DEC | BIN | Symbol | Description |
| :---: | :--------: | :---: | :-- |
| 0 | 00000000 | NUL | Null character |
| 1 | 00000001 | SOH | Start of Heading |
| 2 | 00000010 | STX | Start of Text |
| 3 | 00000011 | ETX | End of Text |
| 4 | 00000100 | EOT | End of Transmission |
| 5 | 00000101 | ENQ | Enquiry |
| 6 | 00000110 | ACK | Acknowledge |
| 7 | 00000111 | BEL | Bell, Alert |
| 8 | 00001000 | BS | Backspace
| 9 | 00001001 | HT | Horizontal Tab |
| 10 | 00001010 | LF | Line Feed
| 11 | 00001011 | VT | Vertical Tabulation |
| 12 | 00001100 | FF | Form Feed |
| 13 | 00001101 | CR | Carriage Return |
| 14 | 00001110 | SO | Shift Out |
| 15 | 00001111 | SI | Shift In |
| 16 | 00010000 | DLE | Data Link Escape |
| 17 | 00010001 | DC1 | Device Control One (XON) |
| 18 | 00010010 | DC2 | Device Control Two |
| 19 | 00010011 | DC3 | Device Control Three (XOFF)| |
| 20 | 00010100 | DC4 | Device Control Four |
| 21 | 00010101 | NAK | Negative Acknowledge |
| 22 | 00010110 | SYN | Synchronous Idle |
| 23 | 00010111 | ETB | End of Transmission Block |
| 24 | 00011000 | CAN | Cancel |
| 25 | 00011001 | EM | End of medium |
| 26 | 00011010 | SUB | Substitute |
| 27 | 00011011 | ESC | Escape |
| 28 | 00011100 | FS | File Separator
| 29 | 00011101 | GS | Group Separator
| 30 | 00011110 | RS | Record Separator
| 31 | 00011111 | US | Unit Separator
| 32 | 00100000 | SP | Space
| 33 | 00100001 | ! | Exclamation mark
| 34 | 00100010 | " | Double quotes (or speech marks)
| 35 | 00100011 | # | Number sign
| 36 | 00100100 | $ | Dollar
| 37 | 00100101 | % | Per cent sign
| 38 | 00100110 | & | Ampersand
| 39 | 00100111 | ' | Single quote
| 40 | 00101000 | ( | Open parenthesis (or open bracket)
| 41 | 00101001 | ) | Close parenthesis (or close bracket)
| 42 | 00101010 | * | Asterisk
| 43 | 00101011 | + | Plus
| 44 | 00101100 | , | Comma
| 45 | 00101101 | - | Hyphen-minus
| 46 | 00101110 | . | Period, dot or full stop
| 47 | 00101111 | / | Slash or divide
| 48 | 00110000 | 0 | Zero
| 49 | 00110001 | 1 | One
| 50 | 00110010 | 2 | Two
| 51 | 00110011 | 3 | Three
| 52 | 00110100 | 4 | Four
| 53 | 00110101 | 5 | Five
| 54 | 00110110 | 6 | Six
| 55 | 00110111 | 7 | Seven
| 56 | 00111000 | 8 | Eight
| 57 | 00111001 | 9 | Nine
| 58 | 00111010 | : | Colon
| 59 | 00111011 | ; | Semicolon
| 60 | 00111100 | < | Less than (or open angled bracket)
| 61 | 00111101 | = | Equals
| 62 | 00111110 | > | Greater than (or close angled bracket)
| 63 | 00111111 | ? | Question mark
| 64 | 01000000 | @ | At sign
| 65 | 01000001 | A | Uppercase A
| 66 | 01000010 | B | Uppercase B
| 67 | 01000011 | C | Uppercase C
| 68 | 01000100 | D | Uppercase D
| 69 | 01000101 | E | Uppercase E
| 70 | 01000110 | F | Uppercase F
| 71 | 01000111 | G | Uppercase G
| 72 | 01001000 | H | Uppercase H
| 73 | 01001001 | I | Uppercase I
| 74 | 01001010 | J | Uppercase J
| 75 | 01001011 | K | Uppercase K
| 76 | 01001100 | L | Uppercase L
| 77 | 01001101 | M | Uppercase M
| 78 | 01001110 | N | Uppercase N
| 79 | 01001111 | O | Uppercase O
| 80 | 01010000 | P | Uppercase P
| 81 | 01010001 | Q | Uppercase Q
| 82 | 01010010 | R | Uppercase R
| 83 | 01010011 | S | Uppercase S
| 84 | 01010100 | T | Uppercase T
| 85 | 01010101 | U | Uppercase U
| 86 | 01010110 | V | Uppercase V
| 87 | 01010111 | W | Uppercase W
| 88 | 01011000 | X | Uppercase X
| 89 | 01011001 | Y | Uppercase Y
| 90 | 01011010 | Z | Uppercase Z
| 91 | 01011011 | [ | Opening bracket
| 92 | 01011100 | \ | Backslash
| 93 | 01011101 | ] | Closing bracket
| 94 | 01011110 | ^ | Caret - circumflex
| 95 | 01011111 | _ | Underscore
| 96 | 01100000 | ` | Grave | accent
| 97 | 01100001 | a | Lowercase a
| 98 | 01100010 | b | Lowercase b
| 99 | 01100011 | c | Lowercase c
| 100 | 01100100 | d | Lowercase d
| 101 | 01100101 | e | Lowercase e
| 102 | 01100110 | f | Lowercase f
| 103 | 01100111 | g | Lowercase g
| 104 | 01101000 | h | Lowercase h
| 105 | 01101001 | i | Lowercase i
| 106 | 01101010 | j | Lowercase j
| 107 | 01101011 | k | Lowercase k
| 108 | 01101100 | l | Lowercase l
| 109 | 01101101 | m | Lowercase m
| 110 | 01101110 | n | Lowercase n
| 111 | 01101111 | o | Lowercase o
| 112 | 01110000 | p | Lowercase p
| 113 | 01110001 | q | Lowercase q
| 114 | 01110010 | r | Lowercase r
| 115 | 01110011 | s | Lowercase s
| 116 | 01110100 | t | Lowercase t
| 117 | 01110101 | u | Lowercase u
| 118 | 01110110 | v | Lowercase v
| 119 | 01110111 | w | Lowercase w
| 120 | 01111000 | x | Lowercase x
| 121 | 01111001 | y | Lowercase y
| 122 | 01111010 | z | Lowercase z
| 123 | 01111011 | { | Opening brace
| 124 | 01111100 | | | Vertical bar
| 125 | 01111101 | } | Closing brace
| 126 | 01111110 | ~ | Equivalency sign - tilde
| 127 | 01111111 | DEL | Delete
| 128 | 10000000 | € | Euro sign
| 129 | 10000001 | | Unused
| 130 | 10000010 | ‚ | Single low-9 quotation mark
| 131 | 10000011 | ƒ | Latin small letter f with hook
| 132 | 10000100 | „ | Double low-9 quotation mark
| 133 | 10000101 | … | Horizontal ellipsis
| 134 | 10000110 | † | Dagger
| 135 | 10000111 | ‡ | Double dagger
| 136 | 10001000 | ˆ | Modifier letter circumflex accent
| 137 | 10001001 | ‰ | Per mille sign
| 138 | 10001010 | Š | Latin capital letter S with caron
| 139 | 10001011 | ‹ | Single left-pointing angle quotation
| 140 | 10001100 | Œ | Latin capital ligature OE
| 141 | 10001101 | | Unused
| 142 | 10001110 | Ž | Latin capital letter Z with caron
| 143 | 10001111 | | Unused
| 144 | 10010000 | | Unused
| 145 | 10010001 | ‘ | Left single quotation mark
| 146 | 10010010 | ’ | Right single quotation mark
| 147 | 10010011 | “ | Left double quotation mark
| 148 | 10010100 | ” | Right double quotation mark
| 149 | 10010101 | • | Bullet
| 150 | 10010110 | – | En dash
| 151 | 10010111 | — | Em dash
| 152 | 10011000 | ˜ | Small tilde
| 153 | 10011001 | ™ | Trade mark sign
| 154 | 10011010 | š | Latin small letter S with caron
| 155 | 10011011 | › | Single right-pointing angle quotation mark
| 156 | 10011100 | œ | Latin small ligature oe
| 157 | 10011101 | | Unused
| 158 | 10011110 | ž | Latin small letter z with caron
| 159 | 10011111 | Ÿ | Latin capital letter Y with diaeresis
| 160 | 10100000 | NBSP | Non-breaking space
| 161 | 10100001 | ¡ | Inverted exclamation mark
| 162 | 10100010 | ¢ | Cent sign
| 163 | 10100011 | £ | Pound sign
| 164 | 10100100 | ¤ | Currency sign
| 165 | 10100101 | ¥ | Yen sign
| 166 | 10100110 | ¦ | Pipe, broken vertical bar
| 167 | 10100111 | § | Section sign
| 168 | 10101000 | ¨ | Spacing diaeresis - umlaut
| 169 | 10101001 | © | Copyright sign
| 170 | 10101010 | ª | Feminine ordinal indicator
| 171 | 10101011 | « | Left double angle quotes
| 172 | 10101100 | ¬ | Negation
| 173 | 10101101 | SHY | Soft hyphen
| 174 | 10101110 | ® | Registered trade mark sign
| 175 | 10101111 | ¯ | Spacing macron - over-line
| 176 | 10110000 | ° | Degree sign
| 177 | 10110001 | ± | Plus-or-minus sign
| 178 | 10110010 | ² | Superscript two - squared
| 179 | 10110011 | ³ | Superscript three - cubed
| 180 | 10110100 | ´ | Acute accent - spacing acute
| 181 | 10110101 | µ | Micro sign
| 182 | 10110110 | ¶ | Pilcrow sign - paragraph sign
| 183 | 10110111 | · | Middle dot - Georgian comma
| 184 | 10111000 | ¸ | Spacing cedilla
| 185 | 10111001 | ¹ | Superscript one
| 186 | 10111010 | º | Masculine ordinal indicator
| 187 | 10111011 | » | Right double angle quotes
| 188 | 10111100 | ¼ | Fraction one quarter
| 189 | 10111101 | ½ | Fraction one half
| 190 | 10111110 | ¾ | Fraction three quarters
| 191 | 10111111 | ¿ | Inverted question mark
| 192 | 11000000 | À | Latin capital letter A with grave
| 193 | 11000001 | Á | Latin capital letter A with acute
| 194 | 11000010 | Â | Latin capital letter A with circumflex
| 195 | 11000011 | Ã | Latin capital letter A with tilde
| 196 | 11000100 | Ä | Latin capital letter A with diaeresis
| 197 | 11000101 | Å | Latin capital letter A with ring above
| 198 | 11000110 | Æ | Latin capital letter AE
| 199 | 11000111 | Ç | Latin capital letter C with cedilla
| 200 | 11001000 | È | Latin capital letter E with grave
| 201 | 11001001 | É | Latin capital letter E with acute
| 202 | 11001010 | Ê | Latin capital letter E with circumflex
| 203 | 11001011 | Ë | Latin capital letter E with diaeresis
| 204 | 11001100 | Ì | Latin capital letter I with grave
| 205 | 11001101 | Í | Latin capital letter I with acute
| 206 | 11001110 | Î | Latin capital letter I with circumflex
| 207 | 11001111 | Ï | Latin capital letter I with diaeresis
| 208 | 11010000 | Ð | Latin capital letter ETH
| 209 | 11010001 | Ñ | Latin capital letter N with tilde
| 210 | 11010010 | Ò | Latin capital letter O with grave
| 211 | 11010011 | Ó | Latin capital letter O with acute
| 212 | 11010100 | Ô | Latin capital letter O with circumflex
| 213 | 11010101 | Õ | Latin capital letter O with tilde
| 214 | 11010110 | Ö | Latin capital letter O with diaeresis
| 215 | 11010111 | × | Multiplication sign
| 216 | 11011000 | Ø | Latin capital letter O with slash
| 217 | 11011001 | Ù | Latin capital letter U with grave
| 218 | 11011010 | Ú | Latin capital letter U with acute
| 219 | 11011011 | Û | Latin capital letter U with circumflex
| 220 | 11011100 | Ü | Latin capital letter U with diaeresis
| 221 | 11011101 | Ý | Latin capital letter Y with acute
| 222 | 11011110 | Þ | Latin capital letter THORN
| 223 | 11011111 | ß | Latin small letter sharp s - ess-zed
| 224 | 11100000 | à | Latin small letter a with grave
| 225 | 11100001 | á | Latin small letter a with acute
| 226 | 11100010 | â | Latin small letter a with circumflex
| 227 | 11100011 | ã | Latin small letter a with tilde
| 228 | 11100100 | ä | Latin small letter a with diaeresis
| 229 | 11100101 | å | Latin small letter a with ring above
| 230 | 11100110 | æ | Latin small letter ae
| 231 | 11100111 | ç | Latin small letter c with cedilla
| 232 | 11101000 | è | Latin small letter e with grave
| 233 | 11101001 | é | Latin small letter e with acute
| 234 | 11101010 | ê | Latin small letter e with circumflex
| 235 | 11101011 | ë | Latin small letter e with diaeresis
| 236 | 11101100 | ì | Latin small letter i with grave
| 237 | 11101101 | í | Latin small letter i with acute
| 238 | 11101110 | î | Latin small letter i with circumflex
| 239 | 11101111 | ï | Latin small letter i with diaeresis
| 240 | 11110000 | ð | Latin small letter eth
| 241 | 11110001 | ñ | Latin small letter n with tilde
| 242 | 11110010 | ò | Latin small letter o with grave
| 243 | 11110011 | ó | Latin small letter o with acute
| 244 | 11110100 | ô | Latin small letter o with circumflex
| 245 | 11110101 | õ | Latin small letter o with tilde
| 246 | 11110110 | ö | Latin small letter o with diaeresis
| 247 | 11110111 | ÷ | Division sign
| 248 | 11111000 | ø | Latin small letter o with slash
| 249 | 11111001 | ù | Latin small letter u with grave
| 250 | 11111010 | ú | Latin small letter u with acute
| 251 | 11111011 | û | Latin small letter u with circumflex
| 252 | 11111100 | ü | Latin small letter u with diaeresis
| 253 | 11111101 | ý | Latin small letter y with acute
| 254 | 11111110 | þ | Latin small letter thorn
| 255 | 11111111 | ÿ | Latin small letter y with diaeresis
