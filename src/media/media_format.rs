use crate::SdpAttribute;
use crate::SdpConnection;
use crate::SdpCodecIdentifier;

#[derive(Debug, PartialEq, Clone)]
pub struct SdpMediaFormat {
    pub codec: SdpCodecIdentifier,
    pub connection: Option<SdpConnection>,
    pub attributes: Vec<SdpAttribute>
}

impl SdpMediaFormat {

    pub fn new(codec: SdpCodecIdentifier) -> SdpMediaFormat {
        SdpMediaFormat {
            codec,
            connection: None,
            attributes: vec![]
        }
    }

    pub fn connection(mut self, conn: SdpConnection) -> SdpMediaFormat {
        self.connection = Some(conn);
        self
    }

    pub fn attribute(mut self, attr: SdpAttribute) -> SdpMediaFormat {
        self.attributes.push(attr);
        self
    }
}
