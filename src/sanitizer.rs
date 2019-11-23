use crate::SdpOffer;
use crate::core::SdpTransport;
use crate::media::SdpMediaType;
use crate::SdpCodecIdentifier;

#[derive(Debug, PartialEq, Clone)]
pub enum SanitizerError {
    SanitizeError
}

/// Configuration for the sanitizer.
pub struct SdpSanitizerConfig {
    /// If a SdpMediaFormat contains a transport that is not in this list
    /// it will be removed from the `sanitize` response.
    pub allowed_transports: Vec<SdpTransport>,
    /// If a SdpMediaFormat contains a media type that is not in this list
    /// it will be removed from the `sanitize` response.
    pub allowed_media_types: Vec<SdpMediaType>,
    /// If a SdpMediaFormat contains a codec that is not in this list
    /// it will be removed from the `sanitize` response.
    pub allowed_codecs: Vec<SdpCodecIdentifier>
}

/// This struct helps you filter out transports/media types/codecs that you are not prepared to handle.
/// The returned sdp offer will be missing anything not directly allowed by the `SdpSanitizerConfig`
/// struct.
pub struct SdpSanitizer {
    cfg: SdpSanitizerConfig
}

impl SdpSanitizer {

    pub fn new(cfg: SdpSanitizerConfig) -> SdpSanitizer {
        SdpSanitizer { cfg }
    }

    pub fn sanitize(&self, sdp: SdpOffer) -> Result<SdpOffer, SanitizerError> {
        Ok(self._sanitize(sdp)?)
    }

    fn _sanitize(&self, mut sdp: SdpOffer) -> Result<SdpOffer, SanitizerError> {
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
