use nom::character::is_digit;

use crate::parse::slice_to_string;
use crate::parse::parse_u32;

use std::fmt;

use nom::{
    IResult,
    branch::alt,
    character::complete::char,
    bytes::complete::{tag, take_while, take_until},
    combinator::map_res
};

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

pub fn parse_as_bandwidth(input: &[u8]) -> IResult<&[u8], SdpBandwidth> {
    let (input, _) = tag("AS")(input)?;
    let (input, _) = char(' ')(input)?;
    let (input, count) = map_res(take_while(is_digit), parse_u32)(input)?;
    Ok((input, SdpBandwidth::As(count)))
}

pub fn parse_ct_bandwidth(input: &[u8]) -> IResult<&[u8], SdpBandwidth> {
    let (input, _) = tag("CT")(input)?;
    let (input, _) = char(' ')(input)?;
    let (input, count) = map_res(take_while(is_digit), parse_u32)(input)?;
    Ok((input, SdpBandwidth::Ct(count)))
}

pub fn parse_tias_bandwidth(input: &[u8]) -> IResult<&[u8], SdpBandwidth> {
    let (input, _) = tag("TIAS")(input)?;
    let (input, _) = char(' ')(input)?;
    let (input, count) = map_res(take_while(is_digit), parse_u32)(input)?;
    Ok((input, SdpBandwidth::Tias(count)))
}

pub fn parse_known_bandwidth(input: &[u8]) -> IResult<&[u8], SdpBandwidth> {
    alt((
      parse_as_bandwidth,
      parse_ct_bandwidth,
      parse_tias_bandwidth
    ))(input)
}

pub fn parse_unknown_bandwidth(input: &[u8]) -> IResult<&[u8], SdpBandwidth> {
    let (input, key) = map_res(take_until(" "), slice_to_string)(input)?;
    let (input, _) = char(' ')(input)?;
    let (input, value) = map_res(take_while(is_digit), parse_u32)(input)?;
    Ok((input, SdpBandwidth::Unknown(key, value)))
}

pub fn parse_bandwidth(input: &[u8]) -> IResult<&[u8], SdpBandwidth> {
    alt((
      parse_known_bandwidth,
      parse_unknown_bandwidth
    ))(input)
}

pub fn parse_bandwidth_line(input: &[u8]) -> IResult<&[u8], SdpOptionalAttribute> {
    let (input, _) = tag("b=")(input)?;
    let (input, width) = parse_bandwidth(input)?;
    Ok((input, SdpOptionalAttribute::Bandwidth(width)))
}
