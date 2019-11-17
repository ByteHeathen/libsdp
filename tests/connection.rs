use libsdp::*;

#[test]
fn write() {
    let connection = SdpConnection {
        address_type: SdpAddressType::Ipv4,
        network_type: SdpNetworkType::Internet,
        address: "10.1.10.1".into()
    };
    let output = "IN IP4 10.1.10.1".to_string();
    assert_eq!(output, format!("{}", connection));

    let connection = SdpConnection {
        address_type: SdpAddressType::Ipv6,
        network_type: SdpNetworkType::Internet,
        address: "2001:0db8:0000:0000:0000:8a2e:0370:7334".into()
    };
    let output = "IN IP6 2001:0db8:0000:0000:0000:8a2e:0370:7334".to_string();
    assert_eq!(output, format!("{}", connection));
}

#[test]
fn parse() {
    let remains = vec![b'\r'];
    let output = SdpConnection {
        address_type: SdpAddressType::Ipv4,
        network_type: SdpNetworkType::Internet,
        address: "10.1.10.1".into()
    };
    let connection = "IN IP4 10.1.10.1\r".to_string();
    assert_eq!(Ok((remains.as_ref(), output)), parse_connection(connection.as_ref()));

    let remains = vec![b'\r'];
    let output = SdpConnection {
        address_type: SdpAddressType::Ipv6,
        network_type: SdpNetworkType::Internet,
        address: "2001:0db8:0000:0000:0000:8a2e:0370:7334".into()
    };
    let connection = "IN IP6 2001:0db8:0000:0000:0000:8a2e:0370:7334\r".to_string();
    assert_eq!(Ok((remains.as_ref(), output)), parse_connection(connection.as_ref()));
}
