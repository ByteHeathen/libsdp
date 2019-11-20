use crate::parse::slice_to_string;

use std::fmt;

use crate::SdpNetworkType;
use crate::parse_network_type;
use crate::SdpAddressType;
use crate::parse_address_type;
use crate::attributes::SdpOptionalAttributes;

#[derive(Debug, PartialEq, Clone)]
pub struct SdpConnection {
    pub network_type: SdpNetworkType,
    pub address_type: SdpAddressType,
    pub address: String
}

named!(pub parse_connection<SdpConnection>, do_parse!(
    network_type: parse_network_type >>
    char!(' ') >>
    address_type: parse_address_type >>
    char!(' ') >>
    address: map_res!(take_until!("\r"), slice_to_string) >>
    (SdpConnection { network_type, address_type, address })
));

impl fmt::Display for SdpConnection {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} {} {}", self.network_type, self.address_type, self.address)
    }
}

named!(pub parse_connection_name<SdpOptionalAttributes>, do_parse!(
    tag!("c=") >>
    conn: parse_connection >>
    tag!("\r\n") >>
    (SdpOptionalAttributes::Connection(conn))
));
