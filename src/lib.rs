//! libsdp is a small utility library for parsing the sdp protocol.
//! Mostly Intended for SIP user agents.

extern crate nom;

mod lines;
pub use self::lines::{
    SdpVersion, parse_version, parse_version_line,
    SdpSessionName, parse_session_name, parse_session_name_line,
    SdpOrigin, parse_origin, parse_origin_line, SdpTiming,
    parse_timing, parse_time_line, SdpConnection, parse_connection,
    parse_connection_name, parse_phone_line, parse_email_line,
    parse_uri_line, parse_information_line 
};

mod attributes;
pub use self::attributes::{
    SdpAttribute, SdpAttributeType, SdpOptionalAttribute,
    parse_attribute_type, parse_global_attribute,
    parse_global_attributes, RtpMap
};

mod media;
pub use self::media::{
    SdpMedia, SdpMediaType, SdpMediaFormat,
    parse_media, parse_media_lines, SdpEncoding
};

mod core;
pub use self::core::{
    SdpNetworkType, parse_network_type,
    SdpAddressType, parse_address_type,
    SdpCodecIdentifier, parse_codec_identifier,
    SdpBandwidth, parse_bandwidth_line,
    parse_bandwidth, SdpTransport, parse_transport
};

mod offer;
pub use self::offer::SdpOffer;
pub use self::offer::parse_sdp_offer;

mod sanitizer;
pub use self::sanitizer::SdpSanitizer;
pub use self::sanitizer::SanitizerError;
pub use self::sanitizer::SdpSanitizerConfig;

pub(crate) mod parse;
