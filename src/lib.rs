#[macro_use]
extern crate nom;

mod lines;
pub use self::lines::SdpVersion;
pub use self::lines::parse_version;
pub use self::lines::parse_version_line;
pub use self::lines::SdpSessionName;
pub use self::lines::parse_session_name;
pub use self::lines::parse_session_name_line;
pub use self::lines::SdpOrigin;
pub use self::lines::parse_origin;
pub use self::lines::parse_origin_line;
pub use self::lines::SdpTiming;
pub use self::lines::parse_timing;
pub use self::lines::parse_time_line;
pub use self::lines::SdpConnection;
pub use self::lines::parse_connection;
pub use self::lines::parse_connection_name;
pub use self::lines::parse_phone_line;
pub use self::lines::parse_email_line;
pub use self::lines::parse_uri_line;
pub use self::lines::parse_information_line;

mod attributes;
pub use self::attributes::SdpAttribute;
pub use self::attributes::SdpAttributeType;
pub use self::attributes::SdpOptionalAttributes;
pub use self::attributes::parse_attribute_type;
pub use self::attributes::parse_global_attribute;
pub use self::attributes::parse_global_attributes;


mod media;
pub use self::media::SdpMedia;
pub use self::media::SdpMediaType;
pub use self::media::SdpMediaFormat;
pub use self::media::parse_media;
pub use self::media::parse_media_lines;

mod core;
pub use self::core::SdpNetworkType;
pub use self::core::parse_network_type;
pub use self::core::SdpAddressType;
pub use self::core::parse_address_type;
pub use self::core::SdpCodecIdentifier;
pub use self::core::parse_codec_identifier;
pub use self::core::SdpBandwidth;
pub use self::core::parse_bandwidth_line;
pub use self::core::parse_bandwidth;
pub use self::core::SdpProtocol;
pub use self::core::parse_protocol;

mod offer;
pub use self::offer::SdpOffer;
pub use self::offer::parse_sdp_offer;

pub(crate) mod parse;
