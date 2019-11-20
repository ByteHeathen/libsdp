use libsdp::*;

#[test]
fn parse() {
    let data = "v=0\r
o=bytebuddha 1303 2598 IN IP4 10.1.10.120\r
s=Talk\r
c=IN IP4 10.1.10.120\r
t=0 0\r
m=audio 7078 RTP/AVP 124 111 110 0 8 101\r
a=rtpmap:124 opus/48000\r
a=fmtp:124 useinbandfec=1; usedtx=1\r
a=rtpmap:111 speex/16000\r
a=fmtp:111 vbr=on\r
a=rtpmap:110 speex/8000\r
a=fmtp:110 vbr=on\r
a=rtpmap:101 telephone-event/8000\r
a=fmtp:101 0-11\r
m=video 9078 RTP/AVP 103 99\r
a=rtpmap:103 VP8/90000\r
a=rtpmap:99 MP4V-ES/90000\r
a=fmtp:99 profile-level-id=3\r
";
    let origin = SdpOrigin {
        username: "bytebuddha".into(),
        session_id: 1303,
        session_version: 2598,
        network_type: SdpNetworkType::Internet,
        address_type: SdpAddressType::Ipv4,
        address: "10.1.10.120".into()
    };

    let offer = SdpOffer {
        version: SdpVersion,
        origin,
        name: SdpSessionName::new("Talk"),
        optional: vec![
            SdpOptionalAttributes::Connection(SdpConnection {
                network_type: SdpNetworkType::Internet,
                address_type: SdpAddressType::Ipv4,
                address: "10.1.10.120".into()
            }),
            SdpOptionalAttributes::Timing(SdpTiming::new(0, 0))
        ],
        attributes: vec![],
        media: vec![
            SdpMedia {
                media: SdpMediaType::Audio,
                port: 7078,
                port_count: None,
                protocol: SdpProtocol::RtpAvp,
                attributes: vec![],
                formats: vec![
                    SdpMediaFormat {
                        codec: Codec::Unknown(124),
                        connection: None,
                        attributes: vec![
                            SdpAttribute::RtpMap("opus/48000".into()),
                            SdpAttribute::Fmtp("useinbandfec=1; usedtx=1".into())
                        ]
                    },
                    SdpMediaFormat {
                        codec: Codec::Unknown(111),
                        connection: None,
                        attributes: vec![
                            SdpAttribute::RtpMap("speex/16000".into()),
                            SdpAttribute::Fmtp("vbr=on".into())
                        ]
                    },
                    SdpMediaFormat {
                        codec: Codec::Unknown(110),
                        connection: None,
                        attributes: vec![
                            SdpAttribute::RtpMap("speex/8000".into()),
                            SdpAttribute::Fmtp("vbr=on".into())
                        ]
                    },
                    SdpMediaFormat {
                        codec: Codec::Pcmu,
                        connection: None,
                        attributes: vec![],
                    },
                    SdpMediaFormat {
                        codec: Codec::Pcma,
                        connection: None,
                        attributes: vec![],
                    },
                    SdpMediaFormat {
                        codec: Codec::Unknown(101),
                        connection: None,
                        attributes: vec![
                            SdpAttribute::RtpMap("telephone-event/8000".into()),
                            SdpAttribute::Fmtp("0-11".into())
                        ]
                    }
                ]
            },
            SdpMedia {
                media: SdpMediaType::Video,
                port: 9078,
                port_count: None,
                protocol: SdpProtocol::RtpAvp,
                attributes: vec![],
                formats: vec![
                    SdpMediaFormat {
                        codec: Codec::Unknown(103),
                        connection: None,
                        attributes: vec![
                            SdpAttribute::RtpMap("VP8/90000".into())
                        ]
                    },
                    SdpMediaFormat {
                        codec: Codec::Unknown(99),
                        connection: None,
                        attributes: vec![
                            SdpAttribute::RtpMap("MP4V-ES/90000".into()),
                            SdpAttribute::Fmtp("profile-level-id=3".into())
                        ]
                    }
                ]
            }
        ]
    };
    let remains = vec![];
    assert_eq!(Ok((remains.as_ref(), offer)), parse_sdp_offer(data.as_ref()));
}

#[test]
fn write() {
    let data = "v=0\r
o=bytebuddha 1303 2598 IN IP4 10.1.10.120\r
s=Talk\r
c=IN IP4 10.1.10.120\r
t=0 0\r
m=audio 7078 RTP/AVP 124 111 110 0 8 101\r
a=rtpmap:124 opus/48000\r
a=fmtp:124 useinbandfec=1; usedtx=1\r
a=rtpmap:111 speex/16000\r
a=fmtp:111 vbr=on\r
a=rtpmap:110 speex/8000\r
a=fmtp:110 vbr=on\r
a=rtpmap:101 telephone-event/8000\r
a=fmtp:101 0-11\r
m=video 9078 RTP/AVP 103 99\r
a=rtpmap:103 VP8/90000\r
a=rtpmap:99 MP4V-ES/90000\r
a=fmtp:99 profile-level-id=3\r
".to_string();
    let origin = SdpOrigin {
        username: "bytebuddha".into(),
        session_id: 1303,
        session_version: 2598,
        network_type: SdpNetworkType::Internet,
        address_type: SdpAddressType::Ipv4,
        address: "10.1.10.120".into()
    };

    let offer = SdpOffer {
        version: SdpVersion,
        origin,
        name: SdpSessionName::new("Talk"),
        optional: vec![
            SdpOptionalAttributes::Connection(SdpConnection {
                network_type: SdpNetworkType::Internet,
                address_type: SdpAddressType::Ipv4,
                address: "10.1.10.120".into()
            }),
            SdpOptionalAttributes::Timing(SdpTiming::new(0, 0))
        ],
        attributes: vec![],
        media: vec![
            SdpMedia {
                media: SdpMediaType::Audio,
                port: 7078,
                port_count: None,
                protocol: SdpProtocol::RtpAvp,
                attributes: vec![],
                formats: vec![
                    SdpMediaFormat {
                        codec: Codec::Unknown(124),
                        connection: None,
                        attributes: vec![
                            SdpAttribute::RtpMap("opus/48000".into()),
                            SdpAttribute::Fmtp("useinbandfec=1; usedtx=1".into())
                        ]
                    },
                    SdpMediaFormat {
                        codec: Codec::Unknown(111),
                        connection: None,
                        attributes: vec![
                            SdpAttribute::RtpMap("speex/16000".into()),
                            SdpAttribute::Fmtp("vbr=on".into()),
                        ]
                    },
                    SdpMediaFormat {
                        codec: Codec::Unknown(110),
                        connection: None,
                        attributes: vec![
                            SdpAttribute::RtpMap("speex/8000".into()),
                            SdpAttribute::Fmtp("vbr=on".into())
                        ]
                    },
                    SdpMediaFormat {
                        codec: Codec::Pcmu,
                        connection: None,
                        attributes: vec![],
                    },
                    SdpMediaFormat {
                        codec: Codec::Pcma,
                        connection: None,
                        attributes: vec![],
                    },
                    SdpMediaFormat {
                        codec: Codec::Unknown(101),
                        connection: None,
                        attributes: vec![
                            SdpAttribute::RtpMap("telephone-event/8000".into()),
                            SdpAttribute::Fmtp("0-11".into()),
                        ]
                    }
                ]
            },
            SdpMedia {
                media: SdpMediaType::Video,
                port: 9078,
                port_count: None,
                protocol: SdpProtocol::RtpAvp,
                attributes: vec![],
                formats: vec![
                    SdpMediaFormat {
                        codec: Codec::Unknown(103),
                        connection: None,
                        attributes: vec![
                            SdpAttribute::RtpMap("VP8/90000".into())
                        ]
                    },
                    SdpMediaFormat {
                        codec: Codec::Unknown(99),
                        connection: None,
                        attributes: vec![
                            SdpAttribute::RtpMap("MP4V-ES/90000".into()),
                            SdpAttribute::Fmtp("profile-level-id=3".into()),
                        ]
                    }
                ]
            }
        ]
    };
    assert_eq!(data, format!("{}", offer));
}
