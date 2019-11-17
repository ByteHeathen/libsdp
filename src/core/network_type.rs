use std::fmt;

// https://www.iana.org/assignments/sdp-parameters/sdp-parameters.xhtml#sdp-parameters-4

#[derive(Debug, PartialEq, Clone)]
pub enum SdpNetworkType {
    Internet,
    Telephone,
    Atm,
    Pstn
}

named!(pub parse_network_type<SdpNetworkType>, alt!(
    map!(tag!("IN"), |_| SdpNetworkType::Internet) |
    map!(tag!("TN"), |_| SdpNetworkType::Telephone) |
    map!(tag!("ATM"), |_| SdpNetworkType::Atm) |
    map!(tag!("PSTN"), |_| SdpNetworkType::Pstn)
));

impl fmt::Display for SdpNetworkType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            SdpNetworkType::Internet => write!(f, "IN"),
            SdpNetworkType::Telephone => write!(f, "TN"),
            SdpNetworkType::Atm => write!(f, "ATM"),
            SdpNetworkType::Pstn => write!(f, "PSTN")
        }
    }
}
