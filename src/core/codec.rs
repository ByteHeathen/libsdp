use nom::character::is_digit;

use crate::parse::parse_u32;

use std::fmt;

// https://www.iana.org/assignments/rtp-parameters/rtp-parameters.xhtml#rtp-parameters-1

#[derive(Debug, PartialEq, Clone)]
pub enum SdpCodec {
    Pcmu,
    Gsm,
    G723,
    Pcma,
    Jpeg,
    Unknown(u32)
}

impl fmt::Display for SdpCodec {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            SdpCodec::Pcmu => write!(f, "0"),
            SdpCodec::Gsm => write!(f, "3"),
            SdpCodec::G723 => write!(f, "4"),
            SdpCodec::Pcma => write!(f, "8"),
            SdpCodec::Jpeg => write!(f, "26"),
            SdpCodec::Unknown(num) => write!(f, "{}", num)
        }
    }
}

named!(pub _parse_codec<SdpCodec>, alt!(
    map!(tag!("0"), |_| SdpCodec::Pcmu) |
    map!(tag!("8"), |_| SdpCodec::Pcma) |
    map!(tag!("3"), |_| SdpCodec::Gsm) |
    map!(tag!("4"), |_| SdpCodec::G723) |
    map!(tag!("26"), |_| SdpCodec::Jpeg) |
    parse_unknown_codec
));
named!(pub parse_codec<SdpCodec>, do_parse!(
    out: _parse_codec >>
    (out)
));

named!(parse_unknown_codec<SdpCodec>, do_parse!(
    num: map_res!(take_while!(is_digit), parse_u32) >>
    (SdpCodec::Unknown(num))
));
