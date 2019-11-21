use nom::character::is_digit;

use crate::media::SdpEncoding;
use crate::parse::parse_u64;
use crate::media::parse_encoding;

use std::fmt;

#[derive(Debug, PartialEq, Clone)]
pub struct RtpMap {
    pub encoding: SdpEncoding,
    pub clock_rate: u64,
    pub params: Option<u64>
}

impl RtpMap {

    pub fn new(encoding: SdpEncoding, clock_rate: u64) -> RtpMap {
        RtpMap { encoding, clock_rate, params: None }
    }
}

impl fmt::Display for RtpMap {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if let Some(params) = self.params {
            write!(f, "{}/{}/{}", self.encoding, self.clock_rate, params)
        } else {
            write!(f, "{}/{}", self.encoding, self.clock_rate)
        }
    }
}

named!(pub parse_rtpmap<RtpMap>, do_parse!(
    encoding: parse_encoding >>
    tag!("/") >>
    clock_rate: map_res!(take_while!(is_digit), parse_u64) >>
    opt!(tag!("/")) >>
    params: opt!(map_res!(take_while!(is_digit), parse_u64)) >>
    (RtpMap { encoding, clock_rate, params })
));
