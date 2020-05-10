use nom::character::is_digit;

use crate::parse::parse_u32;

use std::fmt;
use nom::{
    IResult,
    combinator::map_res,
    bytes::complete::take_while
};

// https://www.iana.org/assignments/rtp-parameters/rtp-parameters.xhtml#rtp-parameters-1

#[derive(Debug, PartialEq, Clone)]
pub struct SdpCodecIdentifier(pub u32);

impl fmt::Display for SdpCodecIdentifier {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

pub fn parse_codec_identifier(input: &[u8]) -> IResult<&[u8], SdpCodecIdentifier> {
    let (input, num) = map_res(take_while(is_digit), parse_u32)(input)?;
    Ok((input, SdpCodecIdentifier(num)))
}
