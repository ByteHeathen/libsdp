use crate::SdpAttribute;
use crate::SdpConnection;
use crate::Codec;

#[derive(Debug, PartialEq, Clone)]
pub struct SdpMediaFormat {
    pub codec: Codec,
    pub connection: Option<SdpConnection>,
    pub attributes: Vec<SdpAttribute>
}

impl SdpMediaFormat {

    pub fn new(codec: Codec) -> SdpMediaFormat {
        SdpMediaFormat {
            codec,
            connection: None,
            attributes: vec![]
        }
    }

    pub fn add_connection(mut self, conn: SdpConnection) -> SdpMediaFormat {
        self.connection = Some(conn);
        self
    }

    pub fn add_attribute(mut self, attr: SdpAttribute) -> SdpMediaFormat {
        self.attributes.push(attr);
        self
    }
}
