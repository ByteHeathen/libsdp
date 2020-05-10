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

use nom::{
    IResult,
    branch::alt,
};

use std::fmt;

#[derive(Debug, PartialEq, Clone)]
pub enum SdpOptionalAttribute {
    Connection(SdpConnection),
    Email(String),
    Phone(String),
    Information(String),
    Bandwidth(SdpBandwidth),
    Timing(SdpTiming),
    Uri(String)
}

impl fmt::Display for SdpOptionalAttribute {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            SdpOptionalAttribute::Connection(conn) => write!(f, "c={}", conn),
            SdpOptionalAttribute::Email(email) => write!(f, "e={}", email),
            SdpOptionalAttribute::Phone(phone) => write!(f, "p={}", phone),
            SdpOptionalAttribute::Information(information) => write!(f, "i={}", information),
            SdpOptionalAttribute::Bandwidth(bandwidth) => write!(f, "b={}", bandwidth),
            SdpOptionalAttribute::Timing(timing) => write!(f, "t={}", timing),
            SdpOptionalAttribute::Uri(uri) => write!(f, "u={}", uri),
        }
    }
}

pub fn parse_optional_sdp_attribute(input: &[u8]) -> IResult<&[u8], SdpOptionalAttribute> {
    alt((
      parse_uri_line,
      parse_time_line,
      parse_bandwidth_line,
      parse_connection_name,
      parse_email_line,
      parse_phone_line,
      parse_information_line
    ))(input)
}

pub fn parse_optional_attributes(input: &[u8]) -> ParserResult<Vec<SdpOptionalAttribute>> {
    let mut output = vec![];
    let mut data = input;
    while let Ok((remains, attribute)) = parse_optional_sdp_attribute(data) {
        output.push(attribute);
        data = remains;
    }
    Ok((data, output))
}
