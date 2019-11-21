use nom::character::is_digit;

use crate::parse::slice_to_string;
use crate::parse::parse_u32;

use std::fmt;

use crate::SdpOptionalAttribute;

// https://www.iana.org/assignments/sdp-parameters/sdp-parameters.xhtml#sdp-parameters-3

#[derive(Debug, PartialEq, Clone)]
pub enum SdpBandwidth {
    As(u32),
    Ct(u32),
    Tias(u32),
    Rs(u32),
    Rr(u32),
    Unknown(String, u32)
}

impl fmt::Display for SdpBandwidth {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            SdpBandwidth::As(num) => write!(f, "AS {}", num),
            SdpBandwidth::Ct(num) => write!(f, "CT {}", num),
            SdpBandwidth::Tias(num) => write!(f, "TIAS {}", num),
            SdpBandwidth::Rs(num) => write!(f, "RS {}", num),
            SdpBandwidth::Rr(num) => write!(f, "RR {}", num),
            SdpBandwidth::Unknown(key, num) => write!(f, "{} {}", key, num)
        }
    }
}

named!(pub parse_as_bandwidth<SdpBandwidth>, do_parse!(
    tag!("AS") >>
    char!(' ') >>
    count: map_res!(take_while!(is_digit), parse_u32) >>
    (SdpBandwidth::As(count))
));

named!(pub parse_ct_bandwidth<SdpBandwidth>, do_parse!(
    tag!("CT") >>
    char!(' ') >>
    count: map_res!(take_while!(is_digit), parse_u32) >>
    (SdpBandwidth::Ct(count))
));

named!(pub parse_tias_bandwidth<SdpBandwidth>, do_parse!(
    tag!("TIAS") >>
    char!(' ') >>
    count: map_res!(take_while!(is_digit), parse_u32) >>
    (SdpBandwidth::Tias(count))
));

named!(pub parse_known_bandwidth<SdpBandwidth>, alt!(
    parse_as_bandwidth |
    parse_ct_bandwidth |
    parse_tias_bandwidth
));

named!(pub parse_unknown_bandwidth<SdpBandwidth>, do_parse!(
    key: map_res!(take_until!(" "), slice_to_string) >>
    char!(' ') >>
    value: map_res!(take_while!(is_digit), parse_u32) >>
    (SdpBandwidth::Unknown(key, value))
));

named!(pub parse_bandwidth<SdpBandwidth>, alt!(
    parse_known_bandwidth |
    parse_unknown_bandwidth
));

named!(pub parse_bandwidth_line<SdpOptionalAttribute>, do_parse!(
    tag!("b=") >>
    width: parse_bandwidth >>
    (SdpOptionalAttribute::Bandwidth(width))
));
