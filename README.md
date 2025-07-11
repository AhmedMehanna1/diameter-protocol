# Diameter

Rust Implementation of the Diameter Protocol.

## Overview

This library provides a Rust implementation of the Diameter protocol, as defined by [RFC 6733](https://tools.ietf.org/html/rfc6733).
Currently, the library supports only Diameter Client without TLS.

## Getting Started

### Installation
Add this crate to your Rust project by adding the following to your `Cargo.toml`:

```toml
[dependencies]
diameter-protocol = "^0.1"
```

Or use **cargo** command
```shell
cargo add diameter-protocol
```

## Usage

### Diameter Client Example
Below is an example of creating a Diameter client that sends a Credit-Control-Request (CCR) message to a server and waits for a response.


```rust
use diameter_protocol::errors::DiameterResult;
use diameter_protocol::modeling::avp::avp::Avp;
use diameter_protocol::modeling::avp::avp::AvpFlags::M;
use diameter_protocol::modeling::avp::enumerated::Enumerated;
use diameter_protocol::modeling::avp::unsigned32::Unsigned32;
use diameter_protocol::modeling::avp::utf8_string::{Identity, UTF8String};
use diameter_protocol::modeling::diameter::DiameterMessage;
use diameter_protocol::modeling::message::application_id::ApplicationId;
use diameter_protocol::modeling::message::command_code::CommandCode;
use diameter_protocol::modeling::message::command_flags::CommandFlag;
use diameter_protocol::modeling::message::dictionary;
use diameter_protocol::modeling::message::dictionary::Dictionary;
use diameter_protocol::transport::client::DiameterClient;
use std::sync::Arc;

fn main() -> DiameterResult<()> {
    let dict = Arc::new(Dictionary::new(&[&dictionary::DEFAULT_DICT_XML]));

    let mut ccr: DiameterMessage = DiameterMessage::new(
        CommandFlag::Request,
        CommandCode::CreditControl,
        ApplicationId::Gx,
        1123158611,
        3102381851,
    );

    ccr.add_avp(Avp::new(263, M, None, UTF8String::from_str("a")));
    ccr.add_avp(Avp::new(
        264,
        M,
        None,
        Identity::from_str("host.example.com"),
    ));
    ccr.add_avp(Avp::new(
        296,
        M,
        None,
        Identity::from_str("realm.example.com"),
    ));
    ccr.add_avp(Avp::new(263, M, None, UTF8String::from_str("ses;12345888")));
    ccr.add_avp(Avp::new(416, M, None, Enumerated::new(1)));
    ccr.add_avp(Avp::new(415, M, None, Unsigned32::new(1000)));
    ccr.add_avp(Avp::new(
        264,
        M,
        None,
        Identity::from_str("host.example.com"),
    ));
    ccr.add_avp(Avp::new(
        296,
        M,
        None,
        Identity::from_str("realm.example.com"),
    ));
    ccr.add_avp(Avp::new(416, M, None, Enumerated::new(1)));
    ccr.add_avp(Avp::new(415, M, None, Unsigned32::new(1000)));

    let mut client = DiameterClient::new("127.0.0.1:3868");
    client.connect()?;
    client.send_message(&mut ccr, dict)?;
    client.close()?;
    Ok(())
}
```