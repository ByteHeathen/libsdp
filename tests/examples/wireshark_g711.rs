use libsdp::*;

#[test]
fn parse() {
    let data = "v=0\r
o=Clarent 120386 120387 IN IP4 200.57.7.196\r
s=Clarent C5CM\r
c=IN IP4 200.57.7.196\r
t=0 0\r
m=audio 40376 RTP/AVP 8 18 4 0\r
a=rtpmap:8 PCMA/8000\r
a=rtpmap:18 G729/8000\r
a=rtpmap:4 G723/8000\r
a=rtpmap:0 PCMU/8000\r
a=SendRecv\r
";
    let origin = SdpOrigin {
        username: "Clarent".into(),
        session_id: 120386,
        session_version: 120387,
        network_type: SdpNetworkType::Internet,
        address_type: SdpAddressType::Ipv4,
        address: "200.57.7.196".into()
    };

    let connection = SdpConnection {
        network_type: SdpNetworkType::Internet,
        address_type: SdpAddressType::Ipv4,
        address: "200.57.7.196".into()
    };

    let mut sdp_offer = SdpOffer::new(origin, "Clarent C5CM");
    let optional = vec![
         SdpSessionAttributes::Connection(connection),
         SdpSessionAttributes::Timing(SdpTiming::new(0, 0)),
    ];

    let attributes = vec![];
    let medias = vec![
        SdpMedia::new(SdpMediaType::Audio, 40376, SdpProtocol::RtpAvp)
            .add_attribute(SdpAttribute::new(SdpAttributeType::SendRecv))
            .add_format(
                SdpMediaFormat::new(Codec::Pcma)
                    .add_attribute(SdpAttribute {
                        ty: SdpAttributeType::Rtpmap,
                        value: Some("PCMA/8000".into())
            }))
            .add_format(SdpMediaFormat::new(Codec::Unknown(18))
            .add_attribute(SdpAttribute {
                ty: SdpAttributeType::Rtpmap,
                value: Some("G729/8000".into())
            }))
            .add_format(SdpMediaFormat::new(Codec::G723)
                .add_attribute(SdpAttribute {
                    ty: SdpAttributeType::Rtpmap,
                    value: Some("G723/8000".into())
            }))
            .add_format(SdpMediaFormat::new(Codec::Pcmu)
                    .add_attribute(SdpAttribute {
                        ty: SdpAttributeType::Rtpmap,
                        value: Some("PCMU/8000".into())
            }))
    ];

    for attr in optional {
        sdp_offer = sdp_offer.add_optional_attribute(attr);
    }
    for attr in attributes {
        sdp_offer = sdp_offer.add_attribute(attr);
    }
    for media in medias {
        sdp_offer = sdp_offer.add_media(media);
    }
    let remains = vec![];
    assert_eq!(Ok((remains.as_ref(), sdp_offer)), parse_sdp_offer(data.as_ref()));
}


#[test]
fn write() {
    let data = "v=0\r
o=Clarent 120386 120387 IN IP4 200.57.7.196\r
s=Clarent C5CM\r
c=IN IP4 200.57.7.196\r
t=0 0\r
m=audio 40376 RTP/AVP 8 18 4 0\r
a=sendrecv\r
a=rtpmap:8 PCMA/8000\r
a=rtpmap:18 G729/8000\r
a=rtpmap:4 G723/8000\r
a=rtpmap:0 PCMU/8000\r
".to_string();
    let origin = SdpOrigin {
        username: "Clarent".into(),
        session_id: 120386,
        session_version: 120387,
        network_type: SdpNetworkType::Internet,
        address_type: SdpAddressType::Ipv4,
        address: "200.57.7.196".into()
    };

    let connection = SdpConnection {
        network_type: SdpNetworkType::Internet,
        address_type: SdpAddressType::Ipv4,
        address: "200.57.7.196".into()
    };

    let mut sdp_offer = SdpOffer::new(origin, "Clarent C5CM");
    let optional = vec![
         SdpSessionAttributes::Connection(connection),
         SdpSessionAttributes::Timing(SdpTiming::new(0, 0)),
    ];

    let attributes = vec![];
    let medias = vec![
        SdpMedia::new(SdpMediaType::Audio, 40376, SdpProtocol::RtpAvp)
            .add_attribute(SdpAttribute::new(SdpAttributeType::SendRecv))
            .add_format(
                SdpMediaFormat::new(Codec::Pcma)
                    .add_attribute(SdpAttribute {
                        ty: SdpAttributeType::Rtpmap,
                        value: Some("PCMA/8000".into())
            }))
            .add_format(SdpMediaFormat::new(Codec::Unknown(18))
            .add_attribute(SdpAttribute {
                ty: SdpAttributeType::Rtpmap,
                value: Some("G729/8000".into())
            }))
            .add_format(SdpMediaFormat::new(Codec::G723)
                .add_attribute(SdpAttribute {
                    ty: SdpAttributeType::Rtpmap,
                    value: Some("G723/8000".into())
            }))
            .add_format(SdpMediaFormat::new(Codec::Pcmu)
                    .add_attribute(SdpAttribute {
                        ty: SdpAttributeType::Rtpmap,
                        value: Some("PCMU/8000".into())
            }))
    ];

    for attr in optional {
        sdp_offer = sdp_offer.add_optional_attribute(attr);
    }
    for attr in attributes {
        sdp_offer = sdp_offer.add_attribute(attr);
    }
    for media in medias {
        sdp_offer = sdp_offer.add_media(media);
    }
    assert_eq!(data, format!("{}", sdp_offer));
}
