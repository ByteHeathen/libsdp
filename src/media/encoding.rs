use nom::character::is_alphanumeric;

use crate::parse::slice_to_string;

use std::fmt;

use nom::{
    IResult,
    branch::alt,
    combinator::{map, map_res},
    bytes::complete::{tag_no_case, take_while},
};

#[derive(Debug, PartialEq, Clone)]
pub enum SdpEncoding {
    Pcmu,
    Pcma,
    Unknown(String)
}

pub fn parse_encoding(input: &[u8]) -> IResult<&[u8], SdpEncoding> {
    alt((
      map(tag_no_case("pcmu"), |_| SdpEncoding::Pcmu),
      map(tag_no_case("pcma"), |_| SdpEncoding::Pcma),
      parse_unknown_encoding
    ))(input)
}

fn parse_unknown_encoding(input: &[u8]) -> IResult<&[u8], SdpEncoding> {
    let (input, data) = map_res(take_while(|item| is_alphanumeric(item) || b'-' == item), slice_to_string)(input)?;
    Ok((input, SdpEncoding::Unknown(data)))
}

impl fmt::Display for SdpEncoding {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            SdpEncoding::Pcmu => write!(f, "PCMU"),
            SdpEncoding::Pcma => write!(f, "PCMA"),
            SdpEncoding::Unknown(data) => write!(f, "{}", data)
        }
    }
}
