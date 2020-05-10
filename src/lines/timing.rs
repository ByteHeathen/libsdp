use nom::character::is_digit;

use crate::parse::parse_u64;
use crate::attributes::SdpOptionalAttribute;

use std::fmt;

use nom::{
    IResult,
    character::complete::char,
    combinator::map_res,
    bytes::complete::{take_while, tag}
};

#[derive(Debug, PartialEq, Clone)]
pub struct SdpTiming(pub u64, pub u64);

impl fmt::Display for SdpTiming {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} {}", self.0, self.1)
    }
}

impl SdpTiming {

    pub fn new(start: u64, end: u64) -> SdpTiming {
        SdpTiming(start, end)
    }
}

pub fn parse_time_line(input: &[u8]) -> IResult<&[u8], SdpOptionalAttribute> {
    let (input, _) = tag("t=")(input)?;
    let (input, timing) = parse_timing(input)?;
    let (input, _) = tag("\r\n")(input)?;
    Ok((input, SdpOptionalAttribute::Timing(timing)))
}

pub fn parse_timing(input: &[u8]) -> IResult<&[u8], SdpTiming> {
    let (input, start) = map_res(take_while(is_digit), parse_u64)(input)?;
    let (input, _) = char(' ')(input)?;
    let (input, end) = map_res(take_while(is_digit), parse_u64)(input)?;
    Ok((input, SdpTiming(start, end)))
}
