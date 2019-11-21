use crate::SdpOffer;
use crate::core::SdpTransport;
use crate::media::SdpMediaType;

#[derive(Debug, PartialEq, Clone)]
pub enum SanitizerError {
    SanitizeError
}

pub struct SdpSanitizerConfig {
    pub allowed_transports: Vec<SdpTransport>,
    pub allowed_media_types: Vec<SdpMediaType>
}

pub struct SdpSanitizer {
    cfg: SdpSanitizerConfig
}

impl SdpSanitizer {

    pub fn new(cfg: SdpSanitizerConfig) -> SdpSanitizer {
        SdpSanitizer { cfg }
    }

    pub fn respond(&self, sdp: SdpOffer) -> Result<SdpOffer, SanitizerError> {
        Ok(self.sanitize(sdp)?)
    }

    fn sanitize(&self, mut sdp: SdpOffer) -> Result<SdpOffer, SanitizerError> {
        let mut dex_list = vec![];
        for (dex, media) in sdp.media.iter().enumerate() {
            if !self.cfg.allowed_media_types.contains(&media.media) {
                dex_list.push(dex);
                continue;
            }
            if !self.cfg.allowed_transports.contains(&media.transport) {
                dex_list.push(dex);
                continue;
            }
        }
        for dex in dex_list {
            sdp.media.remove(dex);
        }
        Ok(sdp)
    }
}
