use libsdp::*;
use libsdp::attributes::RtpMap;
use libsdp::SdpEncoding;

#[test]
fn parse() {
    let data = "v=0\r
o=jdoe 2890844526 2890842807 IN IP4 10.47.16.5\r
s=SDP Seminar\r
i=A Seminar on the session description protocol\r
u=http://www.example.com/seminars/sdp.pdf\r
e=j.doe@example.com (Jane Doe)\r
c=IN IP4 224.2.17.12/127\r
t=2873397496 2873404696\r
a=recvonly\r
m=audio 49170 RTP/AVP 0\r
m=video 51372 RTP/AVP 99\r
a=rtpmap:99 h263-1998/90000\r\n";
    let origin = SdpOrigin {
        username: "jdoe".into(),
        session_id: 2890844526,
        session_version: 2890842807,
        network_type: SdpNetworkType::Internet,
        address_type: SdpAddressType::Ipv4,
        address: "10.47.16.5".into()
    };
    let remains: Vec<u8> = vec![];
    let mut sdp_offer = SdpOffer::new(origin, "SDP Seminar");
    let connection = SdpConnection::new("224.2.17.12/127");
    let optional = vec![
         SdpOptionalAttributes::Information("A Seminar on the session description protocol".into()),
         SdpOptionalAttributes::Uri("http://www.example.com/seminars/sdp.pdf".into()),
         SdpOptionalAttributes::Email("j.doe@example.com (Jane Doe)".into()),
         SdpOptionalAttributes::Connection(connection),
         SdpOptionalAttributes::Timing(SdpTiming::new(2873397496, 2873404696))
    ];
    let attributes = vec![
        SdpAttribute::RecvOnly
    ];
    let medias = vec![
        SdpMedia::new(SdpMediaType::Audio, 49170, SdpTransport::RtpAvp)
            .format(SdpMediaFormat::new(SdpCodecIdentifier(0))),
        SdpMedia::new(SdpMediaType::Video, 51372, SdpTransport::RtpAvp)
            .format(SdpMediaFormat::new(SdpCodecIdentifier(99))
                .attribute(SdpAttribute::RtpMap(RtpMap::new(SdpEncoding::Unknown("h263-1998".into()), 90000)))
             )
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
    assert_eq!(Ok((remains.as_ref(), sdp_offer)), parse_sdp_offer(data.as_ref()));
}

#[test]
fn write() {
    let data = "v=0\r
o=jdoe 2890844526 2890842807 IN IP4 10.47.16.5\r
s=SDP Seminar\r
i=A Seminar on the session description protocol\r
u=http://www.example.com/seminars/sdp.pdf\r
e=j.doe@example.com (Jane Doe)\r
c=IN IP4 224.2.17.12/127\r
t=2873397496 2873404696\r
a=recvonly\r
m=audio 49170 RTP/AVP 0\r
m=video 51372 RTP/AVP 99\r
a=rtpmap:99 h263-1998/90000\r\n".to_string();
    let origin = SdpOrigin {
        username: "jdoe".into(),
        session_id: 2890844526,
        session_version: 2890842807,
        network_type: SdpNetworkType::Internet,
        address_type: SdpAddressType::Ipv4,
        address: "10.47.16.5".into()
    };
    let mut sdp_offer = SdpOffer::new(origin, "SDP Seminar");
    let connection = SdpConnection::new("224.2.17.12/127");
    let optional = vec![
         SdpOptionalAttributes::Information("A Seminar on the session description protocol".into()),
         SdpOptionalAttributes::Uri("http://www.example.com/seminars/sdp.pdf".into()),
         SdpOptionalAttributes::Email("j.doe@example.com (Jane Doe)".into()),
         SdpOptionalAttributes::Connection(connection),
         SdpOptionalAttributes::Timing(SdpTiming::new(2873397496, 2873404696))
    ];
    let attributes = vec![
        SdpAttribute::RecvOnly
    ];
    let medias = vec![
        SdpMedia::new(SdpMediaType::Audio, 49170, SdpTransport::RtpAvp)
            .format(SdpMediaFormat::new(SdpCodecIdentifier(0))),
        SdpMedia::new(SdpMediaType::Video, 51372, SdpTransport::RtpAvp)
            .format(SdpMediaFormat::new(SdpCodecIdentifier(99))
                .attribute(SdpAttribute::RtpMap(RtpMap::new(SdpEncoding::Unknown("h263-1998".into()), 90000)))
             )
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
