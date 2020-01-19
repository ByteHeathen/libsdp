# libsdp

**WIP**: This library is still a work in progress

[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![Crates.io](https://img.shields.io/crates/v/libsdp.svg)](https://crates.io/crates/libsdp)
[![Docs.rs](https://docs.rs/libsdp/badge.svg)](https://docs.rs/libsdp)
[![Build Status](https://travis-ci.org/byteheathen/libsdp.svg?branch=master)](https://travis-ci.org/byteheathen/libsdp)
[![Build status](https://ci.appveyor.com/api/projects/status/806nir2h407jkndr?svg=true)](https://ci.appveyor.com/project/byteheathen/libsdp)

libsdp is a parser for the Session Description Protocol, Mostly intended for
SDP's use in SIP messages.

### Examples

```rust
  let sdp_data = "v=0\r
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
  if let Ok((_, sdp_msg)) = parse_sdp_offer(sdp_data.as_ref()) {
    println!("{:?}", &sdp_msg);
  }
```

### dependencies

- **[nom](https://crates.io/crates/nom) 5.0.1**
