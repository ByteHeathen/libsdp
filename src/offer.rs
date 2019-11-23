use crate::*;
use crate::attributes::parse_optional_attributes;

use std::fmt;

#[derive(Debug, PartialEq, Clone)]
pub struct SdpOffer {
    pub version: SdpVersion,
    pub origin: SdpOrigin,
    pub name: SdpSessionName,
    pub optional: Vec<SdpOptionalAttribute>,
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

    /// Generate a new offer from the `origin` and the session name `name`.
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

    /// Add an optional attribute.
    pub fn optional_attribute(mut self, attr: SdpOptionalAttribute) -> SdpOffer {
        self.optional.push(attr);
        self
    }

    /// Add all atributes removing all currently present.
    pub fn optional_attributes(mut self, attr: Vec<SdpOptionalAttribute>) -> SdpOffer {
        self.optional = attr;
        self
    }

    /// Add a single `SdpAttribute` to the attribute list.
    pub fn attribute(mut self, attr: SdpAttribute) -> SdpOffer {
        self.attributes.push(attr);
        self
    }

    /// Add all SdpAttributes removing any that might currently by present.
    pub fn attributes(mut self, attr: Vec<SdpAttribute>) -> SdpOffer {
        self.attributes = attr;
        self
    }

    /// Add a single SdpMedia. Represents a media line.
    pub fn media(mut self, media: SdpMedia) -> SdpOffer {
        self.media.push(media);
        self
    }

    /// Get the global SDP connection if this offer has one. returns none if not present.
    pub fn get_connection(&self) -> Option<SdpConnection> {
        for thing in &self.optional {
            if let SdpOptionalAttribute::Connection(conn) = thing {
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
        writeln!(f, "\r")?;
        Ok(())
    }
}
