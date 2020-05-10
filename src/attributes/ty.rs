use std::fmt;

use nom::{
    IResult,
    branch::alt,
    combinator::map,
    bytes::complete::tag_no_case
};

#[derive(Debug, PartialEq, Clone)]
pub enum SdpAttributeType {
    Rtpmap,
    RecvOnly,
    SendOnly,
    SendRecv,
    Fmtp
}

impl fmt::Display for SdpAttributeType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            SdpAttributeType::Rtpmap => write!(f, "rtpmap"),
            SdpAttributeType::RecvOnly => write!(f, "recvonly"),
            SdpAttributeType::SendRecv => write!(f, "sendrecv"),
            SdpAttributeType::SendOnly => write!(f, "sendonly"),
            SdpAttributeType::Fmtp => write!(f, "fmtp")
        }
    }
}

pub fn parse_attribute_type(input: &[u8]) -> IResult<&[u8], SdpAttributeType> {
    alt((
      map(tag_no_case("rtpmap"), |_| SdpAttributeType::Rtpmap),
      map(tag_no_case("fmtp"), |_| SdpAttributeType::Fmtp),
      map(tag_no_case("recvonly"), |_| SdpAttributeType::RecvOnly),
      map(tag_no_case("sendrecv"), |_| SdpAttributeType::SendRecv),
      map(tag_no_case("sendonly"), |_| SdpAttributeType::SendOnly)
    ))(input)
}
