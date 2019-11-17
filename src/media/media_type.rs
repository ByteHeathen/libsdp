use std::fmt;

#[derive(Debug, PartialEq, Clone)]
pub enum SdpMediaType {
    Audio,
    Video,
    Application
}

named!(pub parse_media_type<SdpMediaType>, alt!(
    map!(tag!("audio"), |_| SdpMediaType::Audio) |
    map!(tag!("video"), |_| SdpMediaType::Video) |
    map!(tag!("application"), |_| SdpMediaType::Application)
));

impl fmt::Display for SdpMediaType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            SdpMediaType::Audio => write!(f, "audio"),
            SdpMediaType::Video => write!(f, "video"),
            SdpMediaType::Application => write!(f, "application")
        }
    }
}
