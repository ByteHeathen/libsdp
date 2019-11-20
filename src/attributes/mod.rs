use crate::parse::slice_to_string;
use crate::parse::ParserResult;

mod ty;
pub use self::ty::SdpAttributeType;
pub use self::ty::parse_attribute_type;

use std::fmt;

#[derive(Debug, PartialEq, Clone)]
pub enum SdpAttribute {
    SendOnly,
    RecvOnly,
    SendRecv,
    RtpMap(String),
    Fmtp(String)
}

//#[derive(Debug, PartialEq, Clone)]
//pub struct SdpAttribute {
//    pub ty: SdpAttributeType,
//    pub value: Option<String>
//}

//impl SdpAttribute {
//
//    pub fn new(ty: SdpAttributeType) -> SdpAttribute {
//        SdpAttribute { ty, value: None }
//    }
//
//    pub fn value<S: Into<String>>(mut self, value: S) -> SdpAttribute {
//        self.value = Some(value.into());
//        self
//    }
//}

named!(pub parse_global_attribute<SdpAttribute>, alt!(
    map!(tag!("a=sendrecv"), |_| SdpAttribute::SendOnly) |
    map!(tag!("a=recvonly"), |_| SdpAttribute::RecvOnly) |
    map!(tag!("a=sendrecv"), |_| SdpAttribute::SendRecv) |
    parse_rtpmap_attribute |
    parse_fmtp_attribute
));

named!(pub parse_rtpmap_attribute<SdpAttribute>, do_parse!(
    tag!("a=rtpmap ") >>
    data: map_res!(take_until!("\r"), slice_to_string) >>
    (SdpAttribute::RtpMap(data))
));

named!(pub parse_fmtp_attribute<SdpAttribute>, do_parse!(
    tag!("a=fmtp ") >>
    data: map_res!(take_until!("\r"), slice_to_string) >>
    (SdpAttribute::Fmtp(data))
));

//named!(pub parse_global_attribute<SdpAttribute>, do_parse!(
//    tag!("a=") >>
//    ty: map_res!(take_while!(is_alphanumeric), parse_attribute_type) >>
//    value: opt!(map_res!(take_until!("\r"), slice_to_string)) >>
//    tag!("\r\n") >>
//    (SdpAttribute { ty: ty.1, value })
//));

pub fn parse_global_attributes(input: &[u8]) -> ParserResult<Vec<SdpAttribute>> {
    let mut output = vec![];
    let mut data = input;
    while let Ok((remains, attribute)) = parse_global_attribute(data) {
        output.push(attribute);
        data = remains;
    }
    Ok((data, output))
}

impl fmt::Display for SdpAttribute {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            SdpAttribute::RecvOnly => write!(f, "recvonly"),
            SdpAttribute::SendOnly => write!(f, "sendonly"),
            SdpAttribute::SendRecv => write!(f, "sendrecv"),
            SdpAttribute::RtpMap(data) => write!(f, "rtpmap {}", data),
            SdpAttribute::Fmtp(data) => write!(f, "fmtp {}", data)
        }
    }
}
