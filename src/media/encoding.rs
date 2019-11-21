use nom::character::is_alphanumeric;

use crate::parse::slice_to_string;

use std::fmt;

#[derive(Debug, PartialEq, Clone)]
pub enum SdpEncoding {
    Pcmu,
    Pcma,
    Unknown(String)
}

named!(pub parse_encoding<SdpEncoding>, alt!(
    map!(tag_no_case!("pcmu"), |_| SdpEncoding::Pcmu) |
    map!(tag_no_case!("pcma"), |_| SdpEncoding::Pcma) |
    parse_unknown_encoding
));

named!(parse_unknown_encoding<SdpEncoding>, do_parse!(
    data: map_res!(take_while!(|item| is_alphanumeric(item) || b'-' == item), slice_to_string) >>
    (SdpEncoding::Unknown(data))
));

impl fmt::Display for SdpEncoding {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            SdpEncoding::Pcmu => write!(f, "PCMU"),
            SdpEncoding::Pcma => write!(f, "PCMA"),
            SdpEncoding::Unknown(data) => write!(f, "{}", data)
        }
    }
}
