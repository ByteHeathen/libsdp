use std::fmt;

use nom::{
    IResult,
    branch::alt,
    combinator::map,
    bytes::complete::tag
};

// https://www.iana.org/assignments/sdp-parameters/sdp-parameters.xhtml#sdp-parameters-5

#[derive(Debug, PartialEq, Clone)]
pub enum SdpAddressType {
    Ipv4,
    Ipv6,
    Nsap,
    Gwid,
    E164
}

pub fn parse_address_type(input: &[u8]) -> IResult<&[u8], SdpAddressType> {
    alt((
        map(tag("IP4"), |_| SdpAddressType::Ipv4),
        map(tag("IP6"), |_| SdpAddressType::Ipv6),
        map(tag("NSAP"), |_| SdpAddressType::Nsap),
        map(tag("GWID"), |_| SdpAddressType::Gwid),
        map(tag("E164"), |_| SdpAddressType::E164)
    ))(input)
} 

impl fmt::Display for SdpAddressType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            SdpAddressType::Ipv4 => write!(f, "IP4"),
            SdpAddressType::Ipv6 => write!(f, "IP6"),
            SdpAddressType::Nsap => write!(f, "NSAP"),
            SdpAddressType::Gwid => write!(f, "GWID"),
            SdpAddressType::E164 => write!(f, "E164")
        }
    }
}
