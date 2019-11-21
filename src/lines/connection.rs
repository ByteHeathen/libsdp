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

named!(pub parse_connection_name<SdpOptionalAttribute>, do_parse!(
    tag!("c=") >>
    conn: parse_connection >>
    tag!("\r\n") >>
    (SdpOptionalAttribute::Connection(conn))
));
