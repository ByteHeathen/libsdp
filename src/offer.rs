use crate::*;

use std::fmt;

#[derive(Debug, PartialEq, Clone)]
pub struct SdpOffer {
    pub version: SdpVersion,
    pub origin: SdpOrigin,
    pub name: SdpSessionName,
    pub optional: Vec<SdpSessionAttributes>,
    pub attributes: Vec<SdpAttribute>,
    pub media: Vec<SdpMedia>
}

named!(pub parse_sdp_offer<SdpOffer>, do_parse!(
    version: parse_version_line >>
    origin: parse_origin_line >>
    name: parse_session_name_line >>
    optional: parse_optional_attributes >>
    attributes: parse_global_attributes >>
    media: parse_media_lines >>
    (SdpOffer { version, origin, name, optional, attributes, media })

));

impl SdpOffer {

    pub fn new<S: Into<String>>(origin: SdpOrigin, name: S) -> SdpOffer {
        SdpOffer {
            version: SdpVersion,
            origin,
            name: SdpSessionName::new(name),
            optional: vec![],
            attributes: vec![],
            media: vec![]
        }
    }

    pub fn add_optional_attribute(mut self, attr: SdpSessionAttributes) -> SdpOffer {
        self.optional.push(attr);
        self
    }

    pub fn optional_attributes(mut self, attr: Vec<SdpSessionAttributes>) -> SdpOffer {
        self.optional = attr;
        self
    }

    pub fn add_attribute(mut self, attr: SdpAttribute) -> SdpOffer {
        self.attributes.push(attr);
        self
    }

    pub fn attributes(mut self, attr: Vec<SdpAttribute>) -> SdpOffer {
        self.attributes = attr;
        self
    }

    pub fn add_media(mut self, media: SdpMedia) -> SdpOffer {
        self.media.push(media);
        self
    }

    pub fn media(mut self, media: Vec<SdpMedia>) -> SdpOffer {
        self.media = media;
        self
    }

    pub fn get_connection(&self) -> Option<SdpConnection> {
        for thing in &self.optional {
            if let SdpSessionAttributes::Connection(conn) = thing {
                return Some(conn.clone());
            }
        }
        None
    }
}

impl fmt::Display for SdpOffer {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "v={}\r\no={}\r\ns={}", self.version, self.origin, self.name)?;
        for attribute in &self.optional {
            write!(f, "\r\n{}", attribute)?;
        }
        for attribute in &self.attributes {
            write!(f, "\r\na={}", attribute)?;
        }
        for media in &self.media {
            write!(f, "\r\nm={}", media)?;
        }
        write!(f, "\r\n")?;
        Ok(())
    }
}
