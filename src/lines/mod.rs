mod version;
pub use self::version::SdpVersion;
pub use self::version::parse_version;
pub use self::version::parse_version_line;

mod session_name;
pub use self::session_name::SdpSessionName;
pub use self::session_name::parse_session_name;
pub use self::session_name::parse_session_name_line;

mod origin;
pub use self::origin::SdpOrigin;
pub use self::origin::parse_origin;
pub use self::origin::parse_origin_line;

mod timing;
pub use self::timing::SdpTiming;
pub use self::timing::parse_timing;
pub use self::timing::parse_time_line;

mod connection;
pub use self::connection::SdpConnection;
pub use self::connection::parse_connection;
pub use self::connection::parse_connection_name;

use crate::SdpOptionalAttribute;

use crate::parse::slice_to_string;

use nom::{
    IResult,
    bytes::complete::{tag,take_until},
    combinator::map_res
};

pub fn parse_email_line(input: &[u8]) -> IResult<&[u8], SdpOptionalAttribute> {
    let (input, _) = tag("e=")(input)?;
    let (input, output) = map_res(take_until("\r"), slice_to_string)(input)?;
    let (input, _) = tag("\r\n")(input)?;
    Ok((input, SdpOptionalAttribute::Email(output)))
}

pub fn parse_phone_line(input: &[u8]) -> IResult<&[u8], SdpOptionalAttribute> {
    let (input, _) = tag("e=")(input)?;
    let (input, output) = map_res(take_until("\r"), slice_to_string)(input)?;
    let (input, _) = tag("\r\n")(input)?;
    Ok((input, SdpOptionalAttribute::Phone(output)))
}

pub fn parse_information_line(input: &[u8]) -> IResult<&[u8], SdpOptionalAttribute> {
    let (input, _) = tag("i=")(input)?;
    let (input, output) = map_res(take_until("\r"), slice_to_string)(input)?;
    let (input, _) = tag("\r\n")(input)?;
    Ok((input, SdpOptionalAttribute::Information(output)))
}

pub fn parse_uri_line(input: &[u8]) -> IResult<&[u8], SdpOptionalAttribute> {
    let (input, _) = tag("u=")(input)?;
    let (input, output) = map_res(take_until("\r"), slice_to_string)(input)?;
    let (input, _) = tag("\r\n")(input)?;
    Ok((input, SdpOptionalAttribute::Uri(output)))
}
