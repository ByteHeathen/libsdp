use std::fmt;

use nom::{
    IResult,
    bytes::complete::tag,
    combinator::map
};

#[derive(Debug, PartialEq, Clone, Default)]
pub struct SdpVersion;

pub fn parse_version(input: &[u8]) -> IResult<&[u8], SdpVersion> {
    map(tag("0"), |_| SdpVersion)(input)
}

impl fmt::Display for SdpVersion {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "0")
    }
}

pub fn parse_version_line(input: &[u8]) -> IResult<&[u8], SdpVersion> {
    let (input, _) = tag("v=")(input)?;
    let (input, _) = parse_version(input)?;
    let (input, _) = tag("\r\n")(input)?;
    Ok((input, SdpVersion))
}
