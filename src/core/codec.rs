use nom::character::is_digit;

use crate::parse::parse_u32;

use std::fmt;

// https://www.iana.org/assignments/rtp-parameters/rtp-parameters.xhtml#rtp-parameters-1

#[derive(Debug, PartialEq, Clone)]
pub struct SdpCodecIdentifier(pub u32);

impl fmt::Display for SdpCodecIdentifier {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

named!(pub parse_codec_identifier<SdpCodecIdentifier>, do_parse!(
    num: map_res!(take_while!(is_digit), parse_u32) >>
    (SdpCodecIdentifier(num))
));
