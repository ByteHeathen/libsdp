use libsdp::*;

#[test]
fn write() {
    let origin = SdpOrigin::new("someguy", 2345, 2345, "10.1.10.1");
    let output = "someguy 2345 2345 IN IP4 10.1.10.1".to_string();
    assert_eq!(output, format!("{}", origin));

    let origin = SdpOrigin::new("something", 23452345, 23452345, "2001:0db8:0000:0000:0000:8a2e:0370:7334")
                                .address_type(SdpAddressType::Ipv6);
    let output = "something 23452345 23452345 IN IP6 2001:0db8:0000:0000:0000:8a2e:0370:7334".to_string();
    assert_eq!(output, format!("{}", origin));
}

#[test]
fn parse() {
    let remains = vec![b'\r'];
    let output = SdpOrigin::new("someguy", 2345, 2345, "10.1.10.1");
    let origin = "someguy 2345 2345 IN IP4 10.1.10.1\r";
    assert_eq!(Ok((remains.as_ref(), output)), parse_origin(origin.as_ref()));

    let remains = vec![b'\r'];
    let output = SdpOrigin::new("something", 23452345, 23452345, "2001:0db8:0000:0000:0000:8a2e:0370:7334")
        .address_type(SdpAddressType::Ipv6);
    let origin = "something 23452345 23452345 IN IP6 2001:0db8:0000:0000:0000:8a2e:0370:7334\r";
    assert_eq!(Ok((remains.as_ref(), output)), parse_origin(origin.as_ref()));
}
