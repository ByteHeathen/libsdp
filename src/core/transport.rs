use std::fmt;

#[derive(Debug, PartialEq, Clone)]
pub enum SdpTransport {
    Udp,
    Tcp,
    RtpAvp,
    RtpSavp
}

named!(pub parse_transport<SdpTransport>, alt!(
    map!(tag!("UDP"), |_| SdpTransport::Udp) |
    map!(tag!("TCP"), |_| SdpTransport::Tcp) |
    map!(tag!("RTP/AVP"), |_| SdpTransport::RtpAvp) |
    map!(tag!("RTP/SAVP"), |_| SdpTransport::RtpSavp)
));

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
