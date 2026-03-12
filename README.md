# udp-wiremock

Provides an UDP receiver to perform black-box testing on Rust applications that output UDP packets. 

Inspired by [wiremock-rs](https://github.com/LukeMathWalker/wiremock-rs)

## Features

- Automatic test results verification
- Multi-message matching 

## Limitations

`udp-wiremock` will not be able to tell appart messages that have identical field types and sizes.

Workarounds require altering messages sent by users, but we are trying to avoid that.

See `tests/api/limitations.rs` for an example.