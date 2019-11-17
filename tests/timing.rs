use libsdp::*;

#[test]
fn write() {
    let timing = SdpTiming::new(2345, 2345);
    let output = "2345 2345".to_string();
    assert_eq!(output, format!("{}", timing));
}

#[test]
fn parse() {
    let remains = vec![b'\r'];
    let output = SdpTiming::new(2345, 2345);
    let timing = "2345 2345\r";
    assert_eq!(Ok((remains.as_ref(), output)), parse_timing(timing.as_ref()));
}
