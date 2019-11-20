use nom::character::is_digit;
use nom::character::is_alphanumeric;
use crate::parse::ParserResult;
use crate::parse::parse_u32;
use crate::parse::slice_to_string;

use crate::Codec;
use crate::SdpMedia;
use crate::SdpAttribute;
use crate::SdpMediaFormat;
use crate::SdpAttributeType;
use crate::parse_codec;
use crate::parse_protocol;
use crate::parse_attribute_type;
use super::parse_media_type;

pub fn parse_media_lines(input: &[u8]) -> ParserResult<Vec<SdpMedia>> {
    let mut output = vec![];
    let mut data = input;
    while let Ok((remains, media)) = parse_media(data.as_ref()) {
        output.push(media);
        data = remains;
    }
    Ok((data, output))
}

named!(pub parse_media<SdpMedia>, do_parse!(
    opt!(tag!("\r\n")) >>
    tag!("m=") >>
    media: map_res!(take_while!(is_alphanumeric), parse_media_type) >>
    char!(' ') >>
    port: map_res!(take_while!(is_digit), parse_u32) >>
    port_count: opt!(parse_optional_port) >>
    char!(' ') >>
    protocol: parse_protocol >>
    char!(' ') >>
    formats: parse_attribute_list >>
    (SdpMedia { media: media.1, port, port_count, protocol, attributes: formats.0, formats: formats.1 })
));

pub fn parse_attribute_list(input: &[u8]) -> ParserResult<(Vec<SdpAttribute>, Vec<SdpMediaFormat>)> {
     let mut initial_data = input;
     let mut formats = vec![];
     let mut global = vec![];
     while let Ok((remains, format)) = parse_initial_media_format(initial_data) {
         formats.push(format);
         initial_data = remains;
     }
     if &initial_data[..2] == b"\r\n" {
         initial_data = &initial_data[2..];
     }

     while let Ok((remains, (ty, codec, value))) = parse_attribute(initial_data) {
        initial_data = remains;
        if let Some(codec) = &codec {
            for media_format in formats.iter_mut() {
                if &media_format.codec == codec {
                    match ty {
                        SdpAttributeType::Rtpmap => {
                            if let Some(value) = value {
                                media_format.attributes.push(SdpAttribute::RtpMap(value));
                            }
                        },
                        SdpAttributeType::Fmtp => {
                            if let Some(value) = value {
                                media_format.attributes.push(SdpAttribute::Fmtp(value));
                            }
                        },
                        SdpAttributeType::SendOnly => {
                            media_format.attributes.push(SdpAttribute::SendOnly);
                        },
                        SdpAttributeType::RecvOnly => {
                            media_format.attributes.push(SdpAttribute::RecvOnly);
                        },
                        SdpAttributeType::SendRecv => {
                            media_format.attributes.push(SdpAttribute::SendRecv);
                        }
                    }
                    break;
                }
            }
        } else {
            match ty {
                SdpAttributeType::Rtpmap => {
                    if let Some(value) = value {
                        global.push(SdpAttribute::RtpMap(value))
                    }
                },
                SdpAttributeType::Fmtp => {
                    if let Some(value) = value {
                        global.push(SdpAttribute::Fmtp(value))
                    }
                },
                SdpAttributeType::SendOnly => global.push(SdpAttribute::SendOnly),
                SdpAttributeType::RecvOnly => global.push(SdpAttribute::RecvOnly),
                SdpAttributeType::SendRecv => global.push(SdpAttribute::SendRecv),
            }
        }
     }
     Ok((initial_data, (global, formats)))
}

named!(pub parse_attribute<(SdpAttributeType, Option<Codec>, Option<String>)>, do_parse!(
    tag!("a=") >>
    ty: parse_attribute_type >>
    opt!(char!(':')) >>
    //port: map_res!(take_while!(is_digit), parse_u32) >>
    //port_count: opt!(parse_optional_port) >>
    codec: opt!(parse_codec) >>
    opt!(char!(' ')) >>
    value: opt!(map_res!(take_until!("\r"), slice_to_string)) >>
    tag!("\r\n") >>
    ((ty, codec, value))
));

named!(pub parse_optional_port<u32>, do_parse!(
    tag!("/") >>
    count: map_res!(take_while!(is_digit), parse_u32) >>
    (count)
));

named!(pub parse_initial_media_format<SdpMediaFormat>, do_parse!(
    opt!(char!(' ')) >>
    codec: parse_codec >>
    (SdpMediaFormat { codec: codec , connection: None, attributes: vec![] })
));
