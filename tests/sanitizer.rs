use libsdp::*;

#[test]
fn sanitizer_simple_single() {
    let in_data = "v=0\r
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

let out_data = "v=0\r
o=jdoe 2890844526 2890842807 IN IP4 10.47.16.5\r
s=SDP Seminar\r
i=A Seminar on the session description protocol\r
u=http://www.example.com/seminars/sdp.pdf\r
e=j.doe@example.com (Jane Doe)\r
c=IN IP4 224.2.17.12/127\r
t=2873397496 2873404696\r
a=recvonly\r
m=audio 49170 RTP/AVP 0\r\n";
   let cfg = SdpSanitizerConfig {
       allowed_codecs: vec![SdpCodecIdentifier(0)],
       allowed_transports: vec![SdpTransport::RtpAvp],
       allowed_media_types: vec![SdpMediaType::Audio]
   };
   let (_, sdp_offer) = parse_sdp_offer(in_data.as_ref()).expect("Failed to parse in sdp");
   let (_, expected_sdp) = parse_sdp_offer(out_data.as_ref()).expect("Failed to parse out sdp");
   let sanitizer = SdpSanitizer::new(cfg);
   let response = sanitizer.sanitize(sdp_offer).expect("Failed to create sip response");
   assert_eq!(expected_sdp, response);
}

#[test]
fn sanitizer_simple_multiple() {
    let in_data = "v=0\r
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
let out_data = "v=0\r
o=bytebuddha 1303 2598 IN IP4 10.1.10.120\r
s=Talk\r
c=IN IP4 10.1.10.120\r
t=0 0\r
m=audio 7078 RTP/AVP 0 8\r
";
let cfg = SdpSanitizerConfig {
    allowed_codecs: vec![SdpCodecIdentifier(0), SdpCodecIdentifier(8)],
    allowed_transports: vec![SdpTransport::RtpAvp],
    allowed_media_types: vec![SdpMediaType::Audio]
};
let (_, sdp_offer) = parse_sdp_offer(in_data.as_ref()).expect("Failed to parse in sdp");
let (_, expected_sdp) = parse_sdp_offer(out_data.as_ref()).expect("Failed to parse out sdp");
let sanitizer = SdpSanitizer::new(cfg);
let response = sanitizer.sanitize(sdp_offer).expect("Failed to create sip response");
assert_eq!(expected_sdp, response);
}
