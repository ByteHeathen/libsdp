use crate::parse::slice_to_string;

use std::fmt;

use crate::SdpNetworkType;
use crate::parse_network_type;
use crate::SdpAddressType;
use crate::parse_address_type;
use crate::attributes::SdpOptionalAttribute;

#[derive(Debug, PartialEq, Clone)]
pub struct SdpConnection {
    pub network_type: SdpNetworkType,
    pub address_type: SdpAddressType,
    pub address: String
}

impl SdpConnection {

    pub fn new<S: Into<String>>(address: S) -> SdpConnection {
        SdpConnection {
            network_type: SdpNetworkType::Internet,
            address_type: SdpAddressType::Ipv4,
            address: address.into()
        }
    }
}

use nom::{
    IResult,
    character::complete::char,
    combinator::map_res,
    bytes::complete::{take_until, tag}
};

pub fn parse_connection(input: &[u8]) -> IResult<&[u8], SdpConnection> {
    let (input, network_type) = parse_network_type(input)?;
    let (input, _) = char(' ')(input)?;
    let (input, address_type) = parse_address_type(input)?;
    let (input, _) = char(' ')(input)?;
    let (input, address) = map_res(take_until("\r"), slice_to_string)(input)?;
    Ok((input, SdpConnection { network_type, address_type, address }))
}

impl fmt::Display for SdpConnection {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} {} {}", self.network_type, self.address_type, self.address)
    }
}

pub fn parse_connection_name(input: &[u8]) -> IResult<&[u8], SdpOptionalAttribute> {
    let (input, _) = tag("c=")(input)?;
    let (input, conn) = parse_connection(input)?;
    let (input, _) = tag("\r\n")(input)?;
    Ok((input, SdpOptionalAttribute::Connection(conn)))
}
