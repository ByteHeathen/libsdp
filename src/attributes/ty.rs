use std::fmt;

#[derive(Debug, PartialEq, Clone)]
pub enum SdpAttributeType {
    Rtpmap,
    RecvOnly,
    SendRecv,
    Fmtp
}

impl fmt::Display for SdpAttributeType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            SdpAttributeType::Rtpmap => write!(f, "rtpmap"),
            SdpAttributeType::RecvOnly => write!(f, "recvonly"),
            SdpAttributeType::SendRecv => write!(f, "sendrecv"),
            SdpAttributeType::Fmtp => write!(f, "fmtp")
        }
    }
}

named!(pub parse_attribute_type<SdpAttributeType>, alt!(
    map!(tag_no_case!("rtpmap"), |_| SdpAttributeType::Rtpmap) |
    map!(tag_no_case!("fmtp"), |_| SdpAttributeType::Fmtp) |
    map!(tag_no_case!("recvonly"), |_| SdpAttributeType::RecvOnly) |
    map!(tag_no_case!("sendrecv"), |_| SdpAttributeType::SendRecv)
));
