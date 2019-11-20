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

    let connection = SdpConnection::new("200.57.7.196");

    let mut sdp_offer = SdpOffer::new(origin, "Clarent C5CM");
    let optional = vec![
         SdpOptionalAttributes::Connection(connection),
         SdpOptionalAttributes::Timing(SdpTiming::new(0, 0)),
    ];

    let attributes = vec![];
    let medias = vec![
        SdpMedia::new(SdpMediaType::Audio, 40376, SdpProtocol::RtpAvp)
            .attribute(SdpAttribute::SendRecv)
            .format(SdpMediaFormat::new(SdpCodec::Pcma)
                    .attribute(SdpAttribute::RtpMap("PCMA/8000".into()))
            )
            .format(SdpMediaFormat::new(SdpCodec::Unknown(18))
                .attribute(SdpAttribute::RtpMap("G729/8000".into())))
            .format(SdpMediaFormat::new(SdpCodec::G723)
                .attribute(SdpAttribute::RtpMap("G723/8000".into())))
            .format(SdpMediaFormat::new(SdpCodec::Pcmu)
                    .attribute(SdpAttribute::RtpMap("PCMU/8000".into())))
    ];

    for attr in optional {
        sdp_offer = sdp_offer.optional_attribute(attr);
    }
    for attr in attributes {
        sdp_offer = sdp_offer.attribute(attr);
    }
    for media in medias {
        sdp_offer = sdp_offer.media(media);
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

    let connection = SdpConnection::new("200.57.7.196");

    let mut sdp_offer = SdpOffer::new(origin, "Clarent C5CM");
    let optional = vec![
         SdpOptionalAttributes::Connection(connection),
         SdpOptionalAttributes::Timing(SdpTiming::new(0, 0)),
    ];

    let attributes = vec![];
    let medias = vec![
        SdpMedia::new(SdpMediaType::Audio, 40376, SdpProtocol::RtpAvp)
              .attribute(SdpAttribute::SendRecv)

              .format(SdpMediaFormat::new(SdpCodec::Pcma)
                .attribute(SdpAttribute::RtpMap("PCMA/8000".into())))

              .format(SdpMediaFormat::new(SdpCodec::Unknown(18))
                .attribute(SdpAttribute::RtpMap("G729/8000".into())))

            .format(SdpMediaFormat::new(SdpCodec::G723)
              .attribute(SdpAttribute::RtpMap("G723/8000".into())))

            .format(SdpMediaFormat::new(SdpCodec::Pcmu)
              .attribute(SdpAttribute::RtpMap("PCMU/8000".into())))
    ];

    for attr in optional {
        sdp_offer = sdp_offer.optional_attribute(attr);
    }
    for attr in attributes {
        sdp_offer = sdp_offer.attribute(attr);
    }
    for media in medias {
        sdp_offer = sdp_offer.media(media);
    }
    assert_eq!(data, format!("{}", sdp_offer));
}
