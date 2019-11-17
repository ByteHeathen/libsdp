#[macro_use]
extern crate nom;

mod lines;
pub use self::lines::SdpVersion;
pub use self::lines::parse_version;
pub use self::lines::parse_version_line;
pub use self::lines::SdpSessionName;
pub use self::lines::parse_session_name;
pub use self::lines::parse_session_name_line;
pub use self::lines::SdpOrigin;
pub use self::lines::parse_origin;
pub use self::lines::parse_origin_line;
pub use self::lines::SdpTiming;
pub use self::lines::parse_timing;
pub use self::lines::parse_time_line;
pub use self::lines::SdpConnection;
pub use self::lines::parse_connection;
pub use self::lines::parse_connection_name;
pub use self::lines::parse_phone_line;
pub use self::lines::parse_email_line;
pub use self::lines::parse_uri_line;
pub use self::lines::parse_information_line;

mod attributes;
pub use self::attributes::SdpAttribute;
pub use self::attributes::SdpAttributeType;
pub use self::attributes::parse_attribute_type;
pub use self::attributes::parse_global_attribute;
pub use self::attributes::parse_global_attributes;


mod media;
pub use self::media::SdpMedia;
pub use self::media::SdpMediaType;
pub use self::media::SdpMediaFormat;
pub use self::media::parse_media;
pub use self::media::parse_media_lines;

mod core;
pub use self::core::SdpNetworkType;
pub use self::core::parse_network_type;
pub use self::core::SdpAddressType;
pub use self::core::parse_address_type;
pub use self::core::Codec;
pub use self::core::parse_codec;
pub use self::core::SdpBandwidth;
pub use self::core::parse_bandwidth_line;
pub use self::core::parse_bandwidth;
pub use self::core::SdpProtocol;
pub use self::core::parse_protocol;

mod offer;
pub use self::offer::SdpOffer;
pub use self::offer::parse_sdp_offer;

pub(crate) mod parse;
use parse::ParserResult;

use std::fmt;

#[derive(Debug, PartialEq, Clone)]
pub enum SdpSessionAttributes {
    Connection(SdpConnection),
    Email(String),
    Phone(String),
    Information(String),
    Bandwidth(SdpBandwidth),
    Timing(SdpTiming),
    Uri(String)
}

impl fmt::Display for SdpSessionAttributes {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            SdpSessionAttributes::Connection(conn) => write!(f, "c={}", conn),
            SdpSessionAttributes::Email(email) => write!(f, "e={}", email),
            SdpSessionAttributes::Phone(phone) => write!(f, "p={}", phone),
            SdpSessionAttributes::Information(information) => write!(f, "i={}", information),
            SdpSessionAttributes::Bandwidth(bandwidth) => write!(f, "b={}", bandwidth),
            SdpSessionAttributes::Timing(timing) => write!(f, "t={}", timing),
            SdpSessionAttributes::Uri(uri) => write!(f, "u={}", uri),
        }
    }
}


named!(pub parse_optional_sdp_attribute<SdpSessionAttributes>, alt!(
    parse_uri_line |
    parse_time_line |
    parse_bandwidth_line |
    parse_connection_name |
    parse_email_line |
    parse_phone_line |
    parse_information_line
));

fn parse_optional_attributes(input: &[u8]) -> ParserResult<Vec<SdpSessionAttributes>> {
    let mut output = vec![];
    let mut data = input;
    while let Ok((remains, attribute)) = parse_optional_sdp_attribute(data) {
        output.push(attribute);
        data = remains;
    }
    Ok((data, output))
}
