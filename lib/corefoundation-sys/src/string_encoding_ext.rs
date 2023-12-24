use crate::CFStringEncoding;

pub const kCFStringEncodingMacJapanese: CFStringEncoding = 1;
pub const kCFStringEncodingMacChineseTrad: CFStringEncoding = 2;
pub const kCFStringEncodingMacKorean: CFStringEncoding = 3;
pub const kCFStringEncodingMacArabic: CFStringEncoding = 4;
pub const kCFStringEncodingMacHebrew: CFStringEncoding = 5;
pub const kCFStringEncodingMacGreek: CFStringEncoding = 6;
pub const kCFStringEncodingMacCyrillic: CFStringEncoding = 7;
pub const kCFStringEncodingMacDevanagari: CFStringEncoding = 9;
pub const kCFStringEncodingMacGurmukhi: CFStringEncoding = 10;
pub const kCFStringEncodingMacGujarati: CFStringEncoding = 11;
pub const kCFStringEncodingMacOriya: CFStringEncoding = 12;
pub const kCFStringEncodingMacBengali: CFStringEncoding = 13;
pub const kCFStringEncodingMacTamil: CFStringEncoding = 14;
pub const kCFStringEncodingMacTelugu: CFStringEncoding = 15;
pub const kCFStringEncodingMacKannada: CFStringEncoding = 16;
pub const kCFStringEncodingMacMalayalam: CFStringEncoding = 17;
pub const kCFStringEncodingMacSinhalese: CFStringEncoding = 18;
pub const kCFStringEncodingMacBurmese: CFStringEncoding = 19;
pub const kCFStringEncodingMacKhmer: CFStringEncoding = 20;
pub const kCFStringEncodingMacThai: CFStringEncoding = 21;
pub const kCFStringEncodingMacLaotian: CFStringEncoding = 22;
pub const kCFStringEncodingMacGeorgian: CFStringEncoding = 23;
pub const kCFStringEncodingMacArmenian: CFStringEncoding = 24;
pub const kCFStringEncodingMacChineseSimp: CFStringEncoding = 25;
pub const kCFStringEncodingMacTibetan: CFStringEncoding = 26;
pub const kCFStringEncodingMacMongolian: CFStringEncoding = 27;
pub const kCFStringEncodingMacEthiopic: CFStringEncoding = 28;
pub const kCFStringEncodingMacCentralEurRoman: CFStringEncoding = 29;
pub const kCFStringEncodingMacVietnamese: CFStringEncoding = 30;
pub const kCFStringEncodingMacExtArabic: CFStringEncoding = 31;
/// Uses script code 0, `smRoman`
pub const kCFStringEncodingMacSymbol: CFStringEncoding = 33;
/// Uses script code 0, `smRoman`
pub const kCFStringEncodingMacDingbats: CFStringEncoding = 34;
/// Uses script code 0, `smRoman`
pub const kCFStringEncodingMacTurkish: CFStringEncoding = 35;
/// Uses script code 0, `smRoman`
pub const kCFStringEncodingMacCroatian: CFStringEncoding = 36;
/// Uses script code 0, `smRoman`
pub const kCFStringEncodingMacIcelandic: CFStringEncoding = 37;
/// Uses script code 0, `smRoman`
pub const kCFStringEncodingMacRomanian: CFStringEncoding = 38;
/// Uses script code 0, `smRoman`
pub const kCFStringEncodingMacCeltic: CFStringEncoding = 39;
/// Uses script code 0, `smRoman`
pub const kCFStringEncodingMacGaelic: CFStringEncoding = 40;
/// Uses script code 4, `smArabic`
/// Like [`kCFStringEncodingMacArabic`] but uses Farsi digits
pub const kCFStringEncodingMacFarsi: CFStringEncoding = 0x8c;
/// Uses script code 7, `smCyrillic`
pub const kCFStringEncodingMacUkrainian: CFStringEncoding = 0x98;
/// Uses script code 32, `smUnimplemented`
pub const kCFStringEncodingMacInuit: CFStringEncoding = 0xec;
/// Uses script code 32, `smUnimplemented`
/// VT100/102 font from Comm Toolbox: Latin-1 repertoire + box drawing etc
pub const kCFStringEncodingMacVT100: CFStringEncoding = 0xfc;
/// Special Mac OS encoding
/// Meta-value, should never appear in a table
pub const kCFStringEncodingMacHFS: CFStringEncoding = 0xff;

/// ISO 8859-2
pub const kCFStringEncodingISOLatin2: CFStringEncoding = 0x0202;
/// ISO 8859-3
pub const kCFStringEncodingISOLatin3: CFStringEncoding = 0x0203;
/// ISO 8859-4
pub const kCFStringEncodingISOLatin4: CFStringEncoding = 0x0204;
/// ISO 8859-5
pub const kCFStringEncodingISOLatinCyrillic: CFStringEncoding = 0x0205;
/// ISO 8859-6, =ASMO 708, =DOS CP 708
pub const kCFStringEncodingISOLatinArabic: CFStringEncoding = 0x0206;
/// ISO 8859-7
pub const kCFStringEncodingISOLatinGreek: CFStringEncoding = 0x0207;
/// ISO 8859-8
pub const kCFStringEncodingISOLatinHebrew: CFStringEncoding = 0x0208;
/// ISO 8859-9
pub const kCFStringEncodingISOLatin5: CFStringEncoding = 0x0209;
/// ISO 8859-10
pub const kCFStringEncodingISOLatin6: CFStringEncoding = 0x020a;
/// ISO 8859-11
pub const kCFStringEncodingISOLatinThai: CFStringEncoding = 0x020b;
/// ISO 8859-13
pub const kCFStringEncodingISOLatin7: CFStringEncoding = 0x020d;
/// ISO 8859-14
pub const kCFStringEncodingISOLatin8: CFStringEncoding = 0x020e;
/// ISO 8859-15
pub const kCFStringEncodingISOLatin9: CFStringEncoding = 0x020f;
/// ISO 8859-16
pub const kCFStringEncodingISOLatin10: CFStringEncoding = 0x0210;

/// MS-DOS & Windows code page 437
pub const kCFStringEncodingDOSLatinUS: CFStringEncoding = 0x0400;
/// MS-DOS & Windows code page 737 (formerly code page 437G)
pub const kCFStringEncodingDOSGreek: CFStringEncoding = 0x0405;
/// MS-DOS & Windows code page 775
pub const kCFStringEncodingDOSBalticRim: CFStringEncoding = 0x0406;
/// MS-DOS & Windows code page 850, "Multilingual"
pub const kCFStringEncodingDOSLatin1: CFStringEncoding = 0x0410;
/// MS-DOS & Windows code page 851
pub const kCFStringEncodingDOSGreek1: CFStringEncoding = 0x0411;
/// MS-DOS & Windows code page 852, Slavic
pub const kCFStringEncodingDOSLatin2: CFStringEncoding = 0x0412;
/// MS-DOS & Windows code page 855, IBM Cyrillic
pub const kCFStringEncodingDOSCyrillic: CFStringEncoding = 0x0413;
/// MS-DOS & Windows code page 857, IBM Turkish
pub const kCFStringEncodingDOSTurkish: CFStringEncoding = 0x0414;
/// MS-DOS & Windows code page 860
pub const kCFStringEncodingDOSPortuguese: CFStringEncoding = 0x0415;
/// MS-DOS & Windows code page 861
pub const kCFStringEncodingDOSIcelandic: CFStringEncoding = 0x0416;
/// MS-DOS & Windows code page 862
pub const kCFStringEncodingDOSHebrew: CFStringEncoding = 0x0417;
/// MS-DOS & Windows code page 863
pub const kCFStringEncodingDOSCanadianFrench: CFStringEncoding = 0x0418;
/// MS-DOS & Windows code page 864
pub const kCFStringEncodingDOSArabic: CFStringEncoding = 0x0419;
/// MS-DOS & Windows code page 865
pub const kCFStringEncodingDOSNordic: CFStringEncoding = 0x041a;
/// MS-DOS & Windows code page 866
pub const kCFStringEncodingDOSRussian: CFStringEncoding = 0x041b;
/// MS-DOS & Windows code page 869, IBM Modern Greek
pub const kCFStringEncodingDOSGreek2: CFStringEncoding = 0x041c;
/// MS-DOS & Windows code page 874, also for Windows
pub const kCFStringEncodingDOSThai: CFStringEncoding = 0x041d;
/// MS-DOS & Windows code page 932, also for Windows
pub const kCFStringEncodingDOSJapanese: CFStringEncoding = 0x0420;
/// MS-DOS & Windows code page 936, also for Windows
pub const kCFStringEncodingDOSChineseSimplif: CFStringEncoding = 0x0421;
/// MS-DOS & Windows code page 949, also for Windows; Unified Hangul Code
pub const kCFStringEncodingDOSKorean: CFStringEncoding = 0x0422;
/// MS-DOS & Windows code page 950, also for Windows
pub const kCFStringEncodingDOSChineseTrad: CFStringEncoding = 0x0423;
/// MS-DOS & Windows code page 1250, Central Europe
pub const kCFStringEncodingWindowsLatin2: CFStringEncoding = 0x0501;
/// MS-DOS & Windows code page 1251, Slavic Cyrillic
pub const kCFStringEncodingWindowsCyrillic: CFStringEncoding = 0x0502;
/// MS-DOS & Windows code page 1253
pub const kCFStringEncodingWindowsGreek: CFStringEncoding = 0x0503;
/// MS-DOS & Windows code page 1254, Turkish
pub const kCFStringEncodingWindowsLatin5: CFStringEncoding = 0x0504;
/// MS-DOS & Windows code page 1255
pub const kCFStringEncodingWindowsHebrew: CFStringEncoding = 0x0505;
/// MS-DOS & Windows code page 1256
pub const kCFStringEncodingWindowsArabic: CFStringEncoding = 0x0506;
/// MS-DOS & Windows code page 1257
pub const kCFStringEncodingWindowsBalticRim: CFStringEncoding = 0x0507;
/// MS-DOS & Windows code page 1258
pub const kCFStringEncodingWindowsVietnamese: CFStringEncoding = 0x0508;
/// MS-DOS & Windows code page 1361, for Windows NT
pub const kCFStringEncodingWindowsKoreanJohab: CFStringEncoding = 0x0510;

/// ANSEL (ANSI Z39.47)
pub const kCFStringEncodingANSEL: CFStringEncoding = 0x0601;
pub const kCFStringEncodingJIS_X0201_76: CFStringEncoding = 0x0620;
pub const kCFStringEncodingJIS_X0208_83: CFStringEncoding = 0x0621;
pub const kCFStringEncodingJIS_X0208_90: CFStringEncoding = 0x0622;
pub const kCFStringEncodingJIS_X0212_90: CFStringEncoding = 0x0623;
pub const kCFStringEncodingJIS_C6226_78: CFStringEncoding = 0x0624;
/// Shift-JIS format encoding of JIS X0213 planes 1 and 2
///
/// # Availability
///
/// * iOS: 2.0
/// * macOS: 10.5
/// * tvOS: 9.0
/// * watchOS: 2.0
#[allow(clippy::doc_markdown)] // LINT: Casing is due to branding. It's not referring to an item.
pub const kCFStringEncodingShiftJIS_X0213: CFStringEncoding = 0x0628;
/// JIS X0213 in plane-row-column notation
pub const kCFStringEncodingShiftJIS_X0213_MenKuTen: CFStringEncoding = 0x0629;
pub const kCFStringEncodingGB_2312_80: CFStringEncoding = 0x0630;
/// annex to GB 13000-93; for Windows 95
pub const kCFStringEncodingGBK_95: CFStringEncoding = 0x0631;
pub const kCFStringEncodingGB_18030_2000: CFStringEncoding = 0x0632;
/// same as KSC 5601-92 without Johab annex
pub const kCFStringEncodingKSC_5601_87: CFStringEncoding = 0x0640;
/// KSC 5601-92 Johab annex
pub const kCFStringEncodingKSC_5601_92_Johab: CFStringEncoding = 0x0641;
/// CNS 11643-1992 plane 1
pub const kCFStringEncodingCNS_11643_92_P1: CFStringEncoding = 0x0651;
/// CNS 11643-1992 plane 2
pub const kCFStringEncodingCNS_11643_92_P2: CFStringEncoding = 0x0652;
/// CNS 11643-1992 plane 3 (was plane 14 in 1986 version)
pub const kCFStringEncodingCNS_11643_92_P3: CFStringEncoding = 0x0653;

pub const kCFStringEncodingISO_2022_JP: CFStringEncoding = 0x0820;
pub const kCFStringEncodingISO_2022_JP_2: CFStringEncoding = 0x0821;
/// RFC 2237
pub const kCFStringEncodingISO_2022_JP_1: CFStringEncoding = 0x0822;
/// JIS X0213
pub const kCFStringEncodingISO_2022_JP_3: CFStringEncoding = 0x0823;
pub const kCFStringEncodingISO_2022_CN: CFStringEncoding = 0x0830;
pub const kCFStringEncodingISO_2022_CN_EXT: CFStringEncoding = 0x0831;
pub const kCFStringEncodingISO_2022_KR: CFStringEncoding = 0x0840;

/// ISO 646, 1-byte katakana, JIS 208, JIS 212
pub const kCFStringEncodingEUC_JP: CFStringEncoding = 0x0920;
/// ISO 646, GB 2312-80
pub const kCFStringEncodingEUC_CN: CFStringEncoding = 0x0930;
/// ISO 646, CNS 11643-1992 Planes 1-16
pub const kCFStringEncodingEUC_TW: CFStringEncoding = 0x0931;
/// ISO 646, KS C 5601-1987
pub const kCFStringEncodingEUC_KR: CFStringEncoding = 0x0940;

/// plain Shift-JIS
pub const kCFStringEncodingShiftJIS: CFStringEncoding = 0x0a01;
/// Russian internet standard
pub const kCFStringEncodingKOI8_R: CFStringEncoding = 0x0a02;
/// Big-5 (has variants)
pub const kCFStringEncodingBig5: CFStringEncoding = 0x0a03;
/// Mac OS Roman permuted to align with ISO Latin-1
pub const kCFStringEncodingMacRomanLatin1: CFStringEncoding = 0x0a04;
/// HZ (RFC 1842, for Chinese mail & news)
pub const kCFStringEncodingHZ_GB_2312: CFStringEncoding = 0x0a05;
/// Big-5 with Hong Kong special char set supplement
pub const kCFStringEncodingBig5_HKSCS_1999: CFStringEncoding = 0x0a06;
/// RFC 1456, Vietnamese
pub const kCFStringEncodingVISCII: CFStringEncoding = 0x0a07;
/// RFC 2319, Ukrainian
pub const kCFStringEncodingKOI8_U: CFStringEncoding = 0x0a08;
/// Taiwan Big-5E standard
pub const kCFStringEncodingBig5_E: CFStringEncoding = 0x0a09;

/// NeXTSTEP Japanese encoding
#[allow(clippy::doc_markdown)] // LINT: Casing is due to branding. It's not referring to an item.
pub const kCFStringEncodingNextStepJapanese: CFStringEncoding = 0x0b02;

/// basic EBCDIC-US
pub const kCFStringEncodingEBCDIC_US: CFStringEncoding = 0x0c01;
/// code page 037, extended EBCDIC (Latin-1 set) for US,Canada...
pub const kCFStringEncodingEBCDIC_CP037: CFStringEncoding = 0x0c02;

/// `kTextEncodingUnicodeDefault + kUnicodeUTF7Format` RFC2152
///
/// # Availability
///
/// * iOS: 4.0
/// * macOS: 10.6
/// * tvOS: 9.0
/// * watchOS: 2.0
#[allow(clippy::doc_markdown)] // LINT: Casing is due to branding. It's not referring to an item.
pub const kCFStringEncodingUTF7: CFStringEncoding = 0x0400_0100;
/// UTF-7 (IMAP folder variant) RFC3501
///
/// # Availability
///
/// * iOS: 4.0
/// * macOS: 10.6
/// * tvOS: 9.0
/// * watchOS: 2.0
#[allow(clippy::doc_markdown)] // LINT: Casing is due to branding. It's not referring to an item.
pub const kCFStringEncodingUTF7_IMAP: CFStringEncoding = 0x0a10;
