lazy_static! {
    static ref WHITESPACE_RE: regex::Regex = regex::Regex::new("^ +| +$|( )+").unwrap();
}

pub fn sanitize_string(msg: &str) -> String{
    let msg: String = msg.chars().into_iter().map(|c| match c {
        // Accents
        'Å' => 'A',
        'å' => 'a',
        'Ä' => 'A',
        'ä' => 'a',
        'à' => 'a',
        'Ç' => 'C',
        'É' => 'E',
        'è' => 'e',
        'é' => 'e',
        'ì' => 'i',
        'Ñ' => 'N',
        'ñ' => 'n',
        'Ö' => 'O',
        'ö' => 'o',
        'ò' => 'o',
        'Ø' => 'O',
        'ø' => 'o',
        'Ü' => 'U',
        'ü' => 'u',
        'ù' => 'u',
        // Non printable chars
        '\u{0000}' => ' ',
        '\u{0001}' => ' ',
        '\u{0002}' => ' ',
        '\u{0003}' => ' ',
        '\u{0004}' => ' ',
        '\u{0005}' => ' ',
        '\u{0006}' => ' ',
        '\u{0007}' => ' ',
        '\u{0008}' => ' ',
        '\u{0009}' => ' ',
        '\n' => ' ',
        '\u{000B}' => ' ',
        '\u{000C}' => ' ',
        '\r' => ' ',
        '\u{000E}' => ' ',
        '\u{000F}' => ' ',
        '\u{0011}' => ' ',
        '\u{0012}' => ' ',
        '\u{0013}' => ' ',
        '\u{0014}' => ' ',
        '\u{0015}' => ' ',
        '\u{0016}' => ' ',
        '\u{0017}' => ' ',
        '\u{0018}' => ' ',
        '\u{0019}' => ' ',
        '\u{001A}' => ' ',
        '\u{001B}' => ' ',
        '\u{001C}' => ' ',
        '\u{001D}' => ' ',
        '\u{001E}' => ' ',
        '\u{001F}' => ' ',
        // Unicode space equivalent
        '\u{0080}' => ' ', 
        '\u{0081}' => ' ', 
        '\u{0082}' => ' ', 
        '\u{0083}' => ' ', 
        '\u{0084}' => ' ', 
        '\u{0085}' => ' ', 
        '\u{0086}' => ' ', 
        '\u{0087}' => ' ', 
        '\u{0088}' => ' ', 
        '\u{0089}' => ' ', 
        '\u{008A}' => ' ', 
        '\u{008B}' => ' ', 
        '\u{008C}' => ' ', 
        '\u{008D}' => ' ', 
        '\u{008E}' => ' ', 
        '\u{008F}' => ' ', 
        '\u{0090}' => ' ', 
        '\u{0091}' => ' ', 
        '\u{0092}' => ' ', 
        '\u{0093}' => ' ', 
        '\u{0094}' => ' ', 
        '\u{0095}' => ' ', 
        '\u{0096}' => ' ', 
        '\u{0097}' => ' ', 
        '\u{0098}' => ' ', 
        '\u{0099}' => ' ', 
        '\u{009A}' => ' ', 
        '\u{009B}' => ' ', 
        '\u{009C}' => ' ', 
        '\u{009D}' => ' ', 
        '\u{009E}' => ' ', 
        '\u{009F}' => ' ', 
        '\u{00A0}' => ' ', 
        '\u{1680}' => ' ', 
        '\u{180E}' => ' ',
        '\u{2000}' => ' ', 
        '\u{2001}' => ' ', 
        '\u{2002}' => ' ', 
        '\u{2003}' => ' ', 
        '\u{2004}' => ' ', 
        '\u{2005}' => ' ', 
        '\u{2006}' => ' ', 
        '\u{2007}' => ' ', 
        '\u{2008}' => ' ', 
        '\u{2009}' => ' ', 
        '\u{200A}' => ' ', 
        '\u{200B}' => ' ', 
        '\u{200C}' => ' ', 
        '\u{200D}' => ' ', 
        '\u{2028}' => ' ', 
        '\u{2029}' => ' ', 
        '\u{202F}' => ' ', 
        '\u{205F}' => ' ', 
        '\u{2060}' => ' ', 
        '\u{3000}' => ' ', 
        '\u{FEFF}' => ' ', 
        // 7bit ASCII coercion
        '\u{0060}' => '\u{0027}',
        '\u{00A2}' => '\u{0063}',
        '\u{00A6}' => '\u{007C}',
        '\u{00A8}' => '\u{0022}',
        '\u{00A9}' => '\u{0063}',
        '\u{00AB}' => '\u{003C}',
        '\u{00AC}' => '\u{002D}',
        '\u{00AE}' => '\u{0052}',
        '\u{00AF}' => '\u{002D}',
        '\u{00B0}' => '\u{006F}',
        '\u{00B1}' => '\u{003F}',
        '\u{00B4}' => '\u{0027}',
        '\u{00B6}' => '\u{003F}',
        '\u{00B7}' => '\u{002E}',
        '\u{00B8}' => '\u{002C}',
        '\u{00BB}' => '\u{003E}',
        '\u{00C0}' => '\u{0041}',
        '\u{00C1}' => '\u{0041}',
        '\u{00C2}' => '\u{0041}',
        '\u{00C3}' => '\u{0041}',
        '\u{00C8}' => '\u{0045}',
        '\u{00CA}' => '\u{0045}',
        '\u{00CB}' => '\u{0045}',
        '\u{00CC}' => '\u{0049}',
        '\u{00CD}' => '\u{0049}',
        '\u{00CE}' => '\u{0049}',
        '\u{00CF}' => '\u{0049}',
        '\u{00D0}' => '\u{0044}',
        '\u{00D2}' => '\u{004F}',
        '\u{00D3}' => '\u{004F}',
        '\u{00D4}' => '\u{004F}',
        '\u{00D5}' => '\u{004F}',
        '\u{00D7}' => '\u{0078}',
        '\u{00D9}' => '\u{0055}',
        '\u{00DA}' => '\u{0055}',
        '\u{00DB}' => '\u{0055}',
        '\u{00DD}' => '\u{0059}',
        '\u{00DE}' => '\u{0054}',
        '\u{00E1}' => '\u{0061}',
        '\u{00E2}' => '\u{0061}',
        '\u{00E3}' => '\u{0061}',
        '\u{00E7}' => '\u{00C7}',
        '\u{00EA}' => '\u{0065}',
        '\u{00EB}' => '\u{0065}',
        '\u{00ED}' => '\u{0069}',
        '\u{00EE}' => '\u{0069}',
        '\u{00EF}' => '\u{0069}',
        '\u{00F0}' => '\u{0064}',
        '\u{00F3}' => '\u{006F}',
        '\u{00F4}' => '\u{006F}',
        '\u{00F5}' => '\u{006F}',
        '\u{00F7}' => '\u{002F}',
        '\u{00FA}' => '\u{0075}',
        '\u{00FB}' => '\u{0075}',
        '\u{00FD}' => '\u{0079}',
        '\u{00FE}' => '\u{0074}',
        '\u{00FF}' => '\u{0079}',
        '\u{0100}' => '\u{0041}',
        '\u{0101}' => '\u{0061}',
        '\u{0102}' => '\u{0041}',
        '\u{0103}' => '\u{0061}',
        '\u{0104}' => '\u{0041}',
        '\u{0105}' => '\u{0061}',
        '\u{0106}' => '\u{0043}',
        '\u{0107}' => '\u{0063}',
        '\u{0109}' => '\u{0063}',
        '\u{010A}' => '\u{0043}',
        '\u{010B}' => '\u{0063}',
        '\u{010C}' => '\u{0043}',
        '\u{010D}' => '\u{0063}',
        '\u{010E}' => '\u{0044}',
        '\u{010F}' => '\u{0064}',
        '\u{0110}' => '\u{0044}',
        '\u{0111}' => '\u{0064}',
        '\u{0112}' => '\u{0045}',
        '\u{0113}' => '\u{0065}',
        '\u{0114}' => '\u{0045}',
        '\u{0115}' => '\u{0065}',
        '\u{0116}' => '\u{0045}',
        '\u{0117}' => '\u{0065}',
        '\u{0118}' => '\u{0045}',
        '\u{0119}' => '\u{0065}',
        '\u{011A}' => '\u{0045}',
        '\u{011B}' => '\u{0065}',
        '\u{011C}' => '\u{0047}',
        '\u{011D}' => '\u{0067}',
        '\u{011E}' => '\u{0047}',
        '\u{011F}' => '\u{0067}',
        '\u{0120}' => '\u{0047}',
        '\u{0121}' => '\u{0067}',
        '\u{0122}' => '\u{0047}',
        '\u{0123}' => '\u{0067}',
        '\u{0124}' => '\u{0048}',
        '\u{0125}' => '\u{0068}',
        '\u{0126}' => '\u{0048}',
        '\u{0127}' => '\u{0068}',
        '\u{0128}' => '\u{0049}',
        '\u{0129}' => '\u{0069}',
        '\u{012A}' => '\u{0049}',
        '\u{012B}' => '\u{0069}',
        '\u{012C}' => '\u{0049}',
        '\u{012D}' => '\u{0069}',
        '\u{012E}' => '\u{0049}',
        '\u{012F}' => '\u{0069}',
        '\u{0130}' => '\u{0049}',
        '\u{0131}' => '\u{0069}',
        '\u{0132}' => '\u{0049}',
        '\u{0133}' => '\u{006A}',
        '\u{0134}' => '\u{004A}',
        '\u{0135}' => '\u{006A}',
        '\u{0136}' => '\u{004B}',
        '\u{0137}' => '\u{006B}',
        '\u{0138}' => '\u{006B}',
        '\u{0139}' => '\u{004C}',
        '\u{013A}' => '\u{006C}',
        '\u{013B}' => '\u{004C}',
        '\u{013C}' => '\u{006C}',
        '\u{013D}' => '\u{004C}',
        '\u{013E}' => '\u{006C}',
        '\u{013F}' => '\u{004C}',
        '\u{0140}' => '\u{006C}',
        '\u{0141}' => '\u{004C}',
        '\u{0142}' => '\u{006C}',
        '\u{0143}' => '\u{004E}',
        '\u{0144}' => '\u{006E}',
        '\u{0145}' => '\u{004E}',
        '\u{0146}' => '\u{006E}',
        '\u{0147}' => '\u{004E}',
        '\u{0148}' => '\u{006E}',
        '\u{0149}' => '\u{006E}',
        '\u{014A}' => '\u{004E}',
        '\u{014B}' => '\u{006E}',
        '\u{014C}' => '\u{004F}',
        '\u{014D}' => '\u{006F}',
        '\u{014E}' => '\u{004F}',
        '\u{014F}' => '\u{006F}',
        '\u{0150}' => '\u{004F}',
        '\u{0151}' => '\u{006F}',
        '\u{0152}' => '\u{004F}',
        '\u{0153}' => '\u{006F}',
        '\u{0154}' => '\u{0052}',
        '\u{0155}' => '\u{0072}',
        '\u{0156}' => '\u{0052}',
        '\u{0157}' => '\u{0072}',
        '\u{0158}' => '\u{0052}',
        '\u{0159}' => '\u{0072}',
        '\u{015A}' => '\u{0053}',
        '\u{015B}' => '\u{0073}',
        '\u{015C}' => '\u{0053}',
        '\u{015D}' => '\u{0073}',
        '\u{015E}' => '\u{0053}',
        '\u{015F}' => '\u{0073}',
        '\u{0160}' => '\u{0053}',
        '\u{0161}' => '\u{0073}',
        '\u{0162}' => '\u{0054}',
        '\u{0163}' => '\u{0074}',
        '\u{0164}' => '\u{0054}',
        '\u{0165}' => '\u{0074}',
        '\u{0166}' => '\u{0054}',
        '\u{0167}' => '\u{0074}',
        '\u{0168}' => '\u{0055}',
        '\u{0169}' => '\u{0075}',
        '\u{016A}' => '\u{0055}',
        '\u{016B}' => '\u{0075}',
        '\u{016E}' => '\u{0055}',
        '\u{016F}' => '\u{0075}',
        '\u{0170}' => '\u{0055}',
        '\u{0171}' => '\u{0075}',
        '\u{0172}' => '\u{0055}',
        '\u{0173}' => '\u{0075}',
        '\u{0174}' => '\u{0057}',
        '\u{0175}' => '\u{0077}',
        '\u{0176}' => '\u{0059}',
        '\u{0177}' => '\u{0079}',
        '\u{0178}' => '\u{0059}',
        '\u{0179}' => '\u{005A}',
        '\u{017A}' => '\u{007A}',
        '\u{017B}' => '\u{005A}',
        '\u{017C}' => '\u{007A}',
        '\u{017D}' => '\u{005A}',
        '\u{017E}' => '\u{007A}',
        '\u{017F}' => '\u{0066}',
        '\u{0181}' => '\u{0042}',
        '\u{018A}' => '\u{0044}',
        '\u{018F}' => '\u{0045}',
        '\u{0192}' => '\u{003F}',
        '\u{0198}' => '\u{004B}',
        '\u{0199}' => '\u{006B}',
        '\u{01A0}' => '\u{004F}',
        '\u{01A1}' => '\u{006F}',
        '\u{01AF}' => '\u{0055}',
        '\u{01B0}' => '\u{0075}',
        '\u{01B3}' => '\u{0059}',
        '\u{01B4}' => '\u{0079}',
        '\u{0253}' => '\u{0062}',
        '\u{0257}' => '\u{0064}',
        '\u{0259}' => '\u{0065}',
        '\u{02BB}' => '\u{0027}',
        '\u{02BC}' => '\u{0027}',
        '\u{02BD}' => '\u{0027}',
        '\u{02D9}' => '\u{0027}',
        '\u{02DD}' => '\u{0022}',
        '\u{037E}' => '\u{003B}',
        '\u{0386}' => '\u{0041}',
        '\u{0387}' => '\u{002E}',
        '\u{0388}' => '\u{0045}',
        '\u{0389}' => '\u{0048}',
        '\u{038A}' => '\u{0049}',
        '\u{038C}' => '\u{004F}',
        '\u{038E}' => '\u{0059}',
        '\u{038F}' => '\u{03A9}',
        '\u{0390}' => '\u{0049}',
        '\u{0391}' => '\u{0041}',
        '\u{0392}' => '\u{0042}',
        '\u{0395}' => '\u{0045}',
        '\u{0396}' => '\u{005A}',
        '\u{0397}' => '\u{0048}',
        '\u{0399}' => '\u{0049}',
        '\u{039A}' => '\u{004B}',
        '\u{039C}' => '\u{004D}',
        '\u{039D}' => '\u{004E}',
        '\u{039F}' => '\u{004F}',
        '\u{03A1}' => '\u{0050}',
        '\u{03A4}' => '\u{0054}',
        '\u{03A5}' => '\u{0059}',
        '\u{03A7}' => '\u{0058}',
        '\u{03AA}' => '\u{0049}',
        '\u{03AB}' => '\u{0059}',
        '\u{03AC}' => '\u{0041}',
        '\u{03AD}' => '\u{0045}',
        '\u{03AE}' => '\u{0048}',
        '\u{03AF}' => '\u{0049}',
        '\u{03B0}' => '\u{0059}',
        '\u{03B1}' => '\u{0041}',
        '\u{03B2}' => '\u{0042}',
        '\u{03B3}' => '\u{0393}',
        '\u{03B4}' => '\u{0394}',
        '\u{03B5}' => '\u{0045}',
        '\u{03B6}' => '\u{005A}',
        '\u{03B7}' => '\u{0048}',
        '\u{03B8}' => '\u{0398}',
        '\u{03B9}' => '\u{0049}',
        '\u{03BA}' => '\u{004B}',
        '\u{03BB}' => '\u{039B}',
        '\u{03BC}' => '\u{004D}',
        '\u{03BD}' => '\u{004E}',
        '\u{03BE}' => '\u{039E}',
        '\u{03BF}' => '\u{004F}',
        '\u{03C0}' => '\u{03A0}',
        '\u{03C1}' => '\u{0050}',
        '\u{03C2}' => '\u{03A3}',
        '\u{03C3}' => '\u{03A3}',
        '\u{03C4}' => '\u{0054}',
        '\u{03C5}' => '\u{0059}',
        '\u{03C6}' => '\u{03A6}',
        '\u{03C7}' => '\u{0058}',
        '\u{03C8}' => '\u{03A8}',
        '\u{03C9}' => '\u{03A9}',
        '\u{03CA}' => '\u{0049}',
        '\u{03CB}' => '\u{0059}',
        '\u{03CC}' => '\u{004F}',
        '\u{03CD}' => '\u{0059}',
        '\u{03CE}' => '\u{03A9}',
        '\u{1E62}' => '\u{0053}',
        '\u{1E63}' => '\u{0073}',
        '\u{1EB8}' => '\u{0045}',
        '\u{1EB9}' => '\u{0065}',
        '\u{1ECA}' => '\u{0049}',
        '\u{1ECB}' => '\u{0069}',
        '\u{1ECC}' => '\u{004F}',
        '\u{1ECD}' => '\u{006F}',
        '\u{1EE4}' => '\u{0055}',
        '\u{2010}' => '\u{002D}',
        '\u{2013}' => '\u{002D}',
        '\u{2014}' => '\u{002D}',
        '\u{201A}' => '\u{0027}',
        '\u{201C}' => '\u{0022}',
        '\u{201D}' => '\u{0022}',
        '\u{201E}' => '\u{0022}',
        '\u{2020}' => '\u{002B}',
        '\u{2021}' => '\u{002B}',
        '\u{2022}' => '\u{002E}',
        '\u{2026}' => '\u{002E}',
        '\u{2030}' => '\u{0025}',
        '\u{2039}' => '\u{003C}',
        '\u{203A}' => '\u{003E}',
        '\u{20A3}' => '\u{0023}',
        '\u{20A4}' => '\u{0023}',
        '\u{20B1}' => '\u{0023}',
        '\u{2122}' => '\u{003F}',
        '\u{221A}' => '\u{003F}',
        '\u{221E}' => '\u{003F}',
        '\u{2248}' => '\u{003F}',
        '\u{2260}' => '\u{003F}',
        '\u{2264}' => '\u{003C}',
        '\u{2265}' => '\u{003E}',
        c => c
    }).collect();
    let msg = WHITESPACE_RE.replace_all(&msg, " ");
    msg.trim().to_string()
}