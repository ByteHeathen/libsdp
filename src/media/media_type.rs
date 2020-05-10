use std::fmt;

use nom::{
    IResult,
    branch::alt,
    combinator::map,
    bytes::complete::tag
};

#[derive(Debug, PartialEq, Clone)]
pub enum SdpMediaType {
    Audio,
    Video,
    Application
}

pub fn parse_media_type(input: &[u8]) -> IResult<&[u8], SdpMediaType> {
    alt((
      map(tag("audio"), |_| SdpMediaType::Audio),
      map(tag("video"), |_| SdpMediaType::Video),
      map(tag("application"), |_| SdpMediaType::Application)
    ))(input)
}

impl fmt::Display for SdpMediaType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            SdpMediaType::Audio => write!(f, "audio"),
            SdpMediaType::Video => write!(f, "video"),
            SdpMediaType::Application => write!(f, "application")
        }
    }
}
