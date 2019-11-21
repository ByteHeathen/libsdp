use crate::parse::slice_to_string;

use std::fmt;

#[derive(Debug, PartialEq, Clone)]
pub struct SdpSessionName(pub String);

named!(pub parse_session_name<SdpSessionName>, do_parse!(
    data: map_res!(take_until!("\r"), slice_to_string) >>
    (SdpSessionName(data))
));

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

named!(pub parse_session_name_line<SdpSessionName>, do_parse!(
    tag!("s=") >>
    name: parse_session_name >>
    tag!("\r\n") >>
    (name)
));

impl SdpSessionName {

    pub fn new<S: Into<String>>(s: S) -> SdpSessionName {
        SdpSessionName(s.into())
    }
}
