use nom::character::{ is_alphanumeric, is_digit };
use crate::parse::parse_u64;
use crate::parse::slice_to_string;

use std::fmt;

use crate::SdpAddressType;
use crate::parse_address_type;
use crate::SdpNetworkType;
use crate::parse_network_type;

#[derive(Debug, PartialEq, Clone)]
pub struct SdpOrigin {
    pub username: String,
    pub session_id: u64,
    pub session_version: u64,
    pub network_type: SdpNetworkType,
    pub address_type: SdpAddressType,
    pub address: String
}

impl SdpOrigin {

    pub fn new<S: Into<String>, S2: Into<String>>(s: S, id: u64, version: u64, addr: S2) -> SdpOrigin {
        SdpOrigin {
            username: s.into(),
            session_id: id,
            session_version: version,
            network_type: SdpNetworkType::Internet,
            address_type: SdpAddressType::Ipv4,
            address: addr.into()
        }
    }

    pub fn address_type(mut self, address: SdpAddressType) -> SdpOrigin {
        self.address_type = address;
        self
    }
}

impl fmt::Display for SdpOrigin {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f,
            "{} {} {} {} {} {}",
            self.username,
            self.session_id,
            self.session_version,
            self.network_type,
            self.address_type,
            self.address
         )
    }
}

use nom::{
    IResult,
    combinator::{map_res},
    character::complete::char,
    bytes::complete::{take_while, take_until, tag}
};

pub fn parse_origin(input: &[u8]) -> IResult<&[u8], SdpOrigin> {
    let (input, username) = map_res(take_while(is_alphanumeric), slice_to_string)(input)?;
    let (input, _) = char(' ')(input)?;
    let (input, session_id) = map_res(take_while(is_digit), parse_u64)(input)?;
    let (input, _) = char(' ')(input)?;
    let (input, session_version) = map_res(take_while(is_digit), parse_u64)(input)?;
    let (input, _) = char(' ')(input)?;
    let (input, network_type) = parse_network_type(input)?;
    let (input, _) = char(' ')(input)?;
    let (input, address_type) = parse_address_type(input)?;
    let (input, _) = char(' ')(input)?;
    let (input, address) = map_res(take_until("\r"), slice_to_string)(input)?;
    Ok((input, SdpOrigin { username, session_id, session_version, network_type, address_type, address }))
}

pub fn parse_origin_line(input: &[u8]) -> IResult<&[u8], SdpOrigin> {
    let (input, _) = tag("o=")(input)?;
    let (input, origin) = parse_origin(input)?;
    let (input, _) = tag("\r\n")(input)?;
    Ok((input, origin))
}
