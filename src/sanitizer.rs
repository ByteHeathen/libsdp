use crate::SdpOffer;
use crate::core::SdpTransport;
use crate::media::SdpMediaType;
use crate::SdpCodecIdentifier;

#[derive(Debug, PartialEq, Clone)]
pub enum SanitizerError {
    SanitizeError
}

pub struct SdpSanitizerConfig {
    pub allowed_transports: Vec<SdpTransport>,
    pub allowed_media_types: Vec<SdpMediaType>,
    pub allowed_codecs: Vec<SdpCodecIdentifier>
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
        sdp.media.retain(|media| {
            if !self.cfg.allowed_media_types.contains(&media.media) {
                return false;
            }
            if !self.cfg.allowed_transports.contains(&media.transport) {
                return false;
            }
            true
        });
        for media in sdp.media.iter_mut() {
            media.formats.retain(|format| {
                if !self.cfg.allowed_codecs.contains(&format.codec) {
                    false
                } else {
                    true
                }
            });
        }
        Ok(sdp)
    }
}
