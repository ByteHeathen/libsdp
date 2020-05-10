use nom::character::is_digit;

use crate::media::SdpEncoding;
use crate::parse::parse_u64;
use crate::media::parse_encoding;

use std::fmt;

use nom::{
    IResult,
    bytes::complete::{tag, take_while},
    combinator::{map_res, opt}
};

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

pub fn parse_rtpmap(input: &[u8]) -> IResult<&[u8], RtpMap> {
    let (input, encoding) = parse_encoding(input)?;
    let (input, _) = tag("/")(input)?;
    let (input, clock_rate) = map_res(take_while(is_digit), parse_u64)(input)?;
    let (input, _) = opt(tag("/"))(input)?;
    let (input, params) = opt(map_res(take_while(is_digit), parse_u64))(input)?;
    Ok((input, RtpMap { encoding, clock_rate, params }))
}
