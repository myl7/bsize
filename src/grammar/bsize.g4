grammar bsize;

size: Number Space* unit;
unit: word | abbr;
word: prefix suffix;
abbr: pre suf?;
prefix: KiloWord | MegaWord | GigaWord | TeraWord | PetaWord | ExaWord;
suffix: (BitBody | ByteBody) S?;
pre: Kilo | Mega | Giga | Tera | Peta | Exa;
suf: Bit | Byte;

ByteBody: 'byte';
BitBody: 'bit';
Number: Digit+;
Space: ' ';
KiloWord: Kilo 'ilo';
MegaWord: Mega 'ega';
GigaWord: Giga 'iga';
TeraWord: Tera 'era';
PetaWord: Peta 'eta';
ExaWord: Exa 'xa';
BiSign: 'i';
Kilo: 'k' | 'K';
Mega: 'm' | 'M';
Giga: 'g' | 'G';
Tera: 't' | 'T';
Peta: 'p' | 'P';
Exa: 'e' | 'E';
Bit: 'b';
Byte: 'B';
S: 's';

fragment Digit: [0-9];
