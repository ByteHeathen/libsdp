use nom::{
    IResult,
    branch::alt,
    bytes::complete::{tag,take_until},
    combinator::{map, map_res},
};

use crate::parse::slice_to_string;
use crate::parse::ParserResult;

mod ty;
pub use self::ty::SdpAttributeType;
pub use self::ty::parse_attribute_type;

mod optional;
pub use self::optional::SdpOptionalAttribute;
pub use self::optional::parse_optional_attributes;

use std::fmt;

mod rtpmap;
pub use self::rtpmap::RtpMap;
pub use self::rtpmap::parse_rtpmap;


#[derive(Debug, PartialEq, Clone)]
pub enum SdpAttribute {
    SendOnly,
    RecvOnly,
    SendRecv,
    RtpMap(RtpMap),
    Fmtp(String)
}

pub fn parse_global_attribute(input: &[u8]) -> IResult<&[u8], SdpAttribute> {
     alt((
       map(tag("a=sendrecv"), |_| SdpAttribute::SendOnly),
       map(tag("a=recvonly"), |_| SdpAttribute::RecvOnly),
       map(tag("a=sendrecv"), |_| SdpAttribute::SendRecv),
       parse_rtpmap_attribute,
       parse_fmtp_attribute
     ))(input)
}

pub fn parse_rtpmap_attribute(input: &[u8]) -> IResult<&[u8], SdpAttribute> {
    let (input, _) = tag("a=rtpmap ")(input)?;
    let (input, data) = parse_rtpmap(input)?;
    Ok((input, SdpAttribute::RtpMap(data)))
}

pub fn parse_fmtp_attribute(input: &[u8]) -> IResult<&[u8], SdpAttribute> {
    let (input, _) = tag("a=fmtp ")(input)?;
    let (input, data) = map_res(take_until("\r"), slice_to_string)(input)?;
    Ok((input, SdpAttribute::Fmtp(data)))
}

pub fn parse_global_attributes(input: &[u8]) -> ParserResult<Vec<SdpAttribute>> {
    let mut output = vec![];
    let mut data = input;
    while let Ok((remains, attribute)) = parse_global_attribute(data) {
        output.push(attribute);
        data = remains;
    }
    Ok((data, output))
}

impl fmt::Display for SdpAttribute {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            SdpAttribute::RecvOnly => write!(f, "recvonly"),
            SdpAttribute::SendOnly => write!(f, "sendonly"),
            SdpAttribute::SendRecv => write!(f, "sendrecv"),
            SdpAttribute::RtpMap(data) => write!(f, "rtpmap {}", data),
            SdpAttribute::Fmtp(data) => write!(f, "fmtp {}", data)
        }
    }
}
