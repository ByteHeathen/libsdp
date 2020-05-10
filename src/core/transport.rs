use std::fmt;

use nom::{
    IResult,
    combinator::map,
    branch::alt,
    bytes::complete::tag
};

#[derive(Debug, PartialEq, Clone)]
pub enum SdpTransport {
    Udp,
    Tcp,
    RtpAvp,
    RtpSavp
}

pub fn parse_transport(input: &[u8]) -> IResult<&[u8], SdpTransport> {
    alt((
        map(tag("UDP"), |_| SdpTransport::Udp),
        map(tag("TCP"), |_| SdpTransport::Tcp),
        map(tag("RTP/AVP"), |_| SdpTransport::RtpAvp),
        map(tag("RTP/SAVP"), |_| SdpTransport::RtpSavp)
    ))(input)
}

impl fmt::Display for SdpTransport {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            SdpTransport::Udp => write!(f, "UDP"),
            SdpTransport::Tcp => write!(f, "TCP"),
            SdpTransport::RtpAvp => write!(f, "RTP/AVP"),
            SdpTransport::RtpSavp => write!(f, "RTP/SAVP")
        }
    }
}
