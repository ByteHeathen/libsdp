use std::fmt;

#[derive(Debug, PartialEq, Clone)]
pub enum SdpProtocol {
    RtpAvp
}

named!(pub parse_protocol<SdpProtocol>, alt!(
    map!(tag!("RTP/AVP"), |_| SdpProtocol::RtpAvp)
));

impl fmt::Display for SdpProtocol {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            SdpProtocol::RtpAvp => write!(f, "RTP/AVP")
        }
    }
}
