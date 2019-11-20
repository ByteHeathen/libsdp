use crate::parse::ParserResult;
use crate::lines::parse_email_line;
use crate::lines::parse_uri_line;
use crate::core::parse_bandwidth_line;
use crate::lines::parse_connection_name;
use crate::lines::parse_phone_line;
use crate::lines::parse_information_line;
use crate::lines::parse_time_line;
use crate::SdpTiming;
use crate::SdpConnection;
use crate::SdpBandwidth;

use std::fmt;

#[derive(Debug, PartialEq, Clone)]
pub enum SdpOptionalAttributes {
    Connection(SdpConnection),
    Email(String),
    Phone(String),
    Information(String),
    Bandwidth(SdpBandwidth),
    Timing(SdpTiming),
    Uri(String)
}

impl fmt::Display for SdpOptionalAttributes {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            SdpOptionalAttributes::Connection(conn) => write!(f, "c={}", conn),
            SdpOptionalAttributes::Email(email) => write!(f, "e={}", email),
            SdpOptionalAttributes::Phone(phone) => write!(f, "p={}", phone),
            SdpOptionalAttributes::Information(information) => write!(f, "i={}", information),
            SdpOptionalAttributes::Bandwidth(bandwidth) => write!(f, "b={}", bandwidth),
            SdpOptionalAttributes::Timing(timing) => write!(f, "t={}", timing),
            SdpOptionalAttributes::Uri(uri) => write!(f, "u={}", uri),
        }
    }
}


named!(pub parse_optional_sdp_attribute<SdpOptionalAttributes>, alt!(
    parse_uri_line |
    parse_time_line |
    parse_bandwidth_line |
    parse_connection_name |
    parse_email_line |
    parse_phone_line |
    parse_information_line
));

pub fn parse_optional_attributes(input: &[u8]) -> ParserResult<Vec<SdpOptionalAttributes>> {
    let mut output = vec![];
    let mut data = input;
    while let Ok((remains, attribute)) = parse_optional_sdp_attribute(data) {
        output.push(attribute);
        data = remains;
    }
    Ok((data, output))
}
