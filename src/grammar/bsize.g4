grammar bsize;

size: Number Space* unit?;
unit: word | abbr;
word: (prefix | biPrefix)? suffix;
abbr: pre BiSign? suf | pre | suf;
prefix: KiloWord | MegaWord | GigaWord | TeraWord | PetaWord;
biPrefix: KibiWord | MebiWord | GibiWord | TebiWord | PebiWord;
suffix: (BitBody | ByteBody) S?;
pre: Kilo | Mega | Giga | Tera | Peta;
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
KibiWord: Kilo 'ibi';
MebiWord: Mega 'ebi';
GibiWord: Giga 'ibi';
TebiWord: Tera 'ebi';
PebiWord: Peta 'ebi';
BiSign: 'i';
Kilo: 'k' | 'K';
Mega: 'm' | 'M';
Giga: 'g' | 'G';
Tera: 't' | 'T';
Peta: 'p' | 'P';
Bit: 'b';
Byte: 'B';
S: 's';

fragment Digit: [0-9];
