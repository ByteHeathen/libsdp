use crate::parse::slice_to_string;

use std::fmt;

use nom::{
    IResult,
    combinator::map_res,
    bytes::complete::{take_until, tag}
};

#[derive(Debug, PartialEq, Clone)]
pub struct SdpSessionName(pub String);

impl SdpSessionName {

    pub fn new<S: Into<String>>(s: S) -> SdpSessionName {
        SdpSessionName(s.into())
    }
}

impl fmt::Display for SdpSessionName {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl Into<String> for SdpSessionName {
    fn into(self) -> String {
        self.0
    }
}

pub fn parse_session_name_line(input: &[u8]) -> IResult<&[u8], SdpSessionName> {
    let (input, _) = tag("s=")(input)?;
    let (input, name) = parse_session_name(input)?;
    let (input, _) = tag("\r\n")(input)?;
    Ok((input, name))
}

pub fn parse_session_name(input: &[u8]) -> IResult<&[u8], SdpSessionName> {
    let (input, data) = map_res(take_until("\r"), slice_to_string)(input)?;
    Ok((input, SdpSessionName(data)))
}
