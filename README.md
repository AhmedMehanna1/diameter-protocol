# Diameter

Rust Implementation of the Diameter Protocol.

[![Crates.io][crates-badge]][crates-url]
[![MIT licensed][mit-badge]][mit-url]
[![MIT licensed][apache-badge]][apache-url]

[crates-badge]: https://img.shields.io/crates/v/diameter-interface.svg
[crates-url]: https://crates.io/crates/diameter-interface
[mit-badge]: https://img.shields.io/badge/license-MIT-blue.svg
[mit-url]: LICENSE-MIT
[apache-badge]: https://img.shields.io/badge/License-Apache_2.0-blue.svg
[apache-url]: LICENSE-APACHE

## Overview

This library provides a Rust implementation of the Diameter protocol, as defined by [RFC 6733](https://tools.ietf.org/html/rfc6733).
Currently, the library supports only Diameter Client without TLS.

## Getting Started

### Installation
Add this crate to your Rust project by adding the following to your `Cargo.toml`:

```toml
[dependencies]
diameter-interface = "^0.1"
```

Or use **cargo** command
```shell
cargo add diameter-interface
```

## Usage

### Diameter Client Example
Below is an example of creating a Diameter client that sends a Credit-Control-Request (CCR) message to a server and waits for a response.


```rust
use diameter_interface::errors::DiameterResult;
use diameter_interface::modeling::avp::avp::Avp;
use diameter_interface::modeling::avp::avp::AvpFlags::M;
use diameter_interface::modeling::avp::enumerated::Enumerated;
use diameter_interface::modeling::avp::unsigned32::Unsigned32;
use diameter_interface::modeling::avp::utf8_string::{Identity, UTF8String};
use diameter_interface::modeling::diameter::DiameterMessage;
use diameter_interface::modeling::message::application_id::ApplicationId;
use diameter_interface::modeling::message::command_code::CommandCode;
use diameter_interface::modeling::message::command_flags::CommandFlag;
use diameter_interface::modeling::message::dictionary;
use diameter_interface::modeling::message::dictionary::Dictionary;
use diameter_interface::transport::client::DiameterClient;
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

    ccr.add(Avp::new(263, M, None, UTF8String::from_str("a")));
    ccr.add_avp(264, M, None, Identity::from_str("host.example.com"));
    ccr.add(Avp::new(
        296,
        M,
        None,
        Identity::from_str("realm.example.com"),
    ));
    ccr.add(Avp::new(263, M, None, UTF8String::from_str("ses;12345888")));
    ccr.add(Avp::new(416, M, None, Enumerated::new(1)));
    ccr.add(Avp::new(415, M, None, Unsigned32::new(1000)));
    ccr.add(Avp::new(
        264,
        M,
        None,
        Identity::from_str("host.example.com"),
    ));
    ccr.add(Avp::new(
        296,
        M,
        None,
        Identity::from_str("realm.example.com"),
    ));
    ccr.add(Avp::new(416, M, None, Enumerated::new(1)));
    ccr.add(Avp::new(415, M, None, Unsigned32::new(1000)));

    let mut client = DiameterClient::new("127.0.0.1:3868");
    client.connect()?;
    let cca: DiameterMessage = client.send_message(&mut ccr, dict)?;
    client.close()?;
    println!("{:?}", cca);
    Ok(())
}
```
