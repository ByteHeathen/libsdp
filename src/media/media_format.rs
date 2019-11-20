use crate::SdpAttribute;
use crate::SdpConnection;
use crate::SdpCodec;

#[derive(Debug, PartialEq, Clone)]
pub struct SdpMediaFormat {
    pub codec: SdpCodec,
    pub connection: Option<SdpConnection>,
    pub attributes: Vec<SdpAttribute>
}

impl SdpMediaFormat {

    pub fn new(codec: SdpCodec) -> SdpMediaFormat {
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
