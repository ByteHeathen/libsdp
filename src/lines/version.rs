use std::fmt;

#[derive(Debug, PartialEq, Clone, Default)]
pub struct SdpVersion;

named!(pub parse_version<SdpVersion>, map!(tag!("0"), |_| SdpVersion));

impl fmt::Display for SdpVersion {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "0")
    }
}

named!(pub parse_version_line<SdpVersion>, do_parse!(
    tag!("v=") >>
    parse_version >>
    tag!("\r\n") >>
    (SdpVersion)
));
