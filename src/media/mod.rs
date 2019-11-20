use crate::SdpProtocol;
use crate::SdpAttribute;

mod media_format;
pub use self::media_format::SdpMediaFormat;

mod media_type;
pub use self::media_type::SdpMediaType;
pub use self::media_type::parse_media_type;

mod parse;
pub use self::parse::parse_media;
pub use self::parse::parse_attribute;
pub use self::parse::parse_media_lines;
pub use self::parse::parse_optional_port;
pub use self::parse::parse_attribute_list;
pub use self::parse::parse_initial_media_format;

use std::fmt;

#[derive(Debug, PartialEq, Clone)]
pub struct SdpMedia {
    pub media: SdpMediaType,
    pub port: u32,
    pub port_count: Option<u32>,
    pub protocol: SdpProtocol,
    pub attributes: Vec<SdpAttribute>,
    pub formats: Vec<SdpMediaFormat>
}

impl SdpMedia {

    pub fn new(media: SdpMediaType, port: u32, protocol: SdpProtocol) -> SdpMedia {
        SdpMedia {
            media,
            port,
            port_count: None,
            protocol,
            attributes: vec![],
            formats: vec![]
        }
    }

    pub fn attribute(mut self, attr: SdpAttribute) -> SdpMedia {
        self.attributes.push(attr);
        self
    }

    pub fn attributes(mut self, attrs: Vec<SdpAttribute>) -> SdpMedia {
        self.attributes = attrs;
        self
    }

    pub fn format(mut self, attr: SdpMediaFormat) -> SdpMedia {
        self.formats.push(attr);
        self
    }

    pub fn protocol(mut self, proto: SdpProtocol) -> SdpMedia {
        self.protocol = proto;
        self
    }

    pub fn port_count(mut self, port_count: u32) -> SdpMedia {
        self.port_count = Some(port_count);
        self
    }
}

impl fmt::Display for SdpMedia {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if let Some(port_count) = self.port_count {
            write!(f, "{} {}/{} {}", self.media, self.port, port_count, self.protocol)?;
        } else {
            write!(f, "{} {} {}", self.media, self.port, self.protocol)?;
        }
        for format in &self.formats {
            write!(f, " {}", format.codec)?;
        }
        for attribute in &self.attributes {
            write!(f, "\r\na={}", attribute)?;
        }
        for format in &self.formats {
            for attribute in &format.attributes {
                match attribute {
                    SdpAttribute::SendOnly => write!(f, "\r\na=sendonly:{}", format.codec)?,
                    SdpAttribute::RecvOnly => write!(f, "\r\na=recvonly:{}", format.codec)?,
                    SdpAttribute::SendRecv => write!(f, "\r\na=sendrecv:{}", format.codec)?,
                    SdpAttribute::RtpMap(data) => write!(f, "\r\na=rtpmap:{} {}", format.codec, data)?,
                    SdpAttribute::Fmtp(data) => write!(f, "\r\na=fmtp:{} {}", format.codec, data)?
                }
            }
        }
        Ok(())
    }
}
