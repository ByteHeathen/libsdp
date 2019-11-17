use nom::character::is_digit;

use crate::parse::parse_u32;

use std::fmt;

// https://www.iana.org/assignments/rtp-parameters/rtp-parameters.xhtml#rtp-parameters-1

#[derive(Debug, PartialEq, Clone)]
pub enum Codec {
    Pcmu,
    Gsm,
    G723,
    Pcma,
    Jpeg,
    Unknown(u32)
}

impl fmt::Display for Codec {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Codec::Pcmu => write!(f, "0"),
            Codec::Gsm => write!(f, "3"),
            Codec::G723 => write!(f, "4"),
            Codec::Pcma => write!(f, "8"),
            Codec::Jpeg => write!(f, "26"),
            Codec::Unknown(num) => write!(f, "{}", num)
        }
    }
}

named!(pub _parse_codec<Codec>, alt!(
    map!(tag!("0"), |_| Codec::Pcmu) |
    map!(tag!("8"), |_| Codec::Pcma) |
    map!(tag!("3"), |_| Codec::Gsm) |
    map!(tag!("4"), |_| Codec::G723) |
    map!(tag!("26"), |_| Codec::Jpeg) |
    parse_unknown_codec
));
named!(pub parse_codec<Codec>, do_parse!(
    out: _parse_codec >>
    (out)
));

named!(parse_unknown_codec<Codec>, do_parse!(
    num: map_res!(take_while!(is_digit), parse_u32) >>
    (Codec::Unknown(num))
));
