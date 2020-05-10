use nom::character::is_digit;
use nom::character::is_alphanumeric;
use crate::parse::ParserResult;
use crate::parse::parse_u32;
use crate::parse::slice_to_string;

use crate::SdpCodecIdentifier;
use crate::SdpMedia;
use crate::SdpAttribute;
use crate::SdpMediaFormat;
use crate::SdpAttributeType;
use crate::parse_codec_identifier;
use crate::parse_transport;
use crate::parse_attribute_type;
use super::parse_media_type;
use crate::attributes::parse_rtpmap;

use nom::{
    IResult,
    character::complete::char,
    bytes::complete::{take_until, take_while,tag},
    combinator::{opt, map_res}
};

pub type RawAttribute = (SdpAttributeType, Option<SdpCodecIdentifier>, Option<String>);

pub fn parse_media_lines(input: &[u8]) -> ParserResult<Vec<SdpMedia>> {
    let mut output = vec![];
    let mut data = input;
    while let Ok((remains, media)) = parse_media(data) {
        output.push(media);
        data = remains;
    }
    Ok((data, output))
}

pub fn parse_media(input: &[u8]) -> IResult<&[u8], SdpMedia> {
    let (input, _) = opt(tag("\r\n"))(input)?;
    let (input, _) = tag("m=")(input)?;
    let (input, media) = map_res(take_while(is_alphanumeric), parse_media_type)(input)?;
    let (input, _) = char(' ')(input)?;
    let (input, port) = map_res(take_while(is_digit), parse_u32)(input)?;
    let (input, port_count) = opt(parse_optional_port)(input)?;
    let (input, _) = char(' ')(input)?;
    let (input, transport) = parse_transport(input)?;
    let (input, _) = char(' ')(input)?;
    let (input, formats) = parse_attribute_list(input)?;
    Ok((input, SdpMedia {
        media: media.1,
        port,
        port_count,
        transport,
        attributes: formats.0,
        formats: formats.1 
    }))
}

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
                                let value = format!("{} ", value);
                                if let Ok((_, rtpmap)) = parse_rtpmap(value.as_ref()) {
                                    media_format.attributes.push(SdpAttribute::RtpMap(rtpmap));
                                }
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
                        if let Ok((_, rtpmap)) = parse_rtpmap(value.as_ref()) {
                            global.push(SdpAttribute::RtpMap(rtpmap))
                        }
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

pub fn parse_attribute(input: &[u8]) -> IResult<&[u8], RawAttribute> {
    let (input, _) = tag("a=")(input)?;
    let (input, ty) = parse_attribute_type(input)?;
    let (input, _) = opt(char(':'))(input)?;
    //let (input, port) = map_res(take_while(is_digit), parse_u32)(input)?;
    //let (input, port_count) = opt(parse_optional_port)(input)?;
    let (input, codec) = opt(parse_codec_identifier)(input)?;
    let (input, _) = opt(char(' '))(input)?;
    let (input, value) = opt(map_res(take_until("\r"), slice_to_string))(input)?;
    let (input, _) = tag("\r\n")(input)?;
    Ok((input, (ty, codec, value)))
}

pub fn parse_optional_port(input: &[u8]) -> IResult<&[u8], u32> {
    let (input, _) = tag("/")(input)?;
    let (input, count) = map_res(take_while(is_digit), parse_u32)(input)?;
    Ok((input, count))
}

pub fn parse_initial_media_format(input: &[u8]) -> IResult<&[u8], SdpMediaFormat> {
    let (input, _) = opt(char(' '))(input)?;
    let (input, codec) = parse_codec_identifier(input)?;
    Ok((input, SdpMediaFormat {
        codec,
        connection: None,
        attributes: vec![]
    }))
}
