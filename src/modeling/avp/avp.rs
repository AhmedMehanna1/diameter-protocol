//! # AVP Module
//!
//! This module defines the structure and functionalities related to AVPs in Diameter messages.
//!
//! ## AVP Format
//! The diagram below illustrates the format for an AVP:
//! ```text
//!   0                   1                   2                   3
//!   0 1 2 3 4 5 6 7 8 9 0 1 2 3 4 5 6 7 8 9 0 1 2 3 4 5 6 7 8 9 0 1
//!  +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
//!  |                         Command-Code                          |
//!  +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
//!  |  Flags       |                 AVP Length                     |
//!  +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
//!  |                         Vendor ID (optional)                  |
//!  +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
//!  |                             Data                              |
//!  +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
//!  |                             Data             |    Padding     |
//!  +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
//!
//!  AVP Flags:
//!    0 1 2 3 4 5 6 7
//!   +-+-+-+-+-+-+-+-+  V(endor), M(andatory), P(rivate)
//!   |V M P r r r r r|  r(eserved)
//!   +-+-+-+-+-+-+-+-+
//! ```
//!

use crate::errors::DiameterResult;
use crate::modeling::avp::data::AvpDataFormater;
use crate::modeling::avp::enumerated::Enumerated;
use crate::modeling::avp::float32::Float32;
use crate::modeling::avp::float64::Float64;
use crate::modeling::avp::group::Grouped;
use crate::modeling::avp::integer32::Integer32;
use crate::modeling::avp::integer64::Integer64;
use crate::modeling::avp::ipv4::IPv4;
use crate::modeling::avp::ipv6::IPv6;
use crate::modeling::avp::octet_string::{DiameterURI, OctetString};
use crate::modeling::avp::time::Time;
use crate::modeling::avp::unsigned32::Unsigned32;
use crate::modeling::avp::unsigned64::Unsigned64;
use crate::modeling::avp::utf8_string::{Identity, UTF8String};
use std::fmt::Debug;
use std::io::Write;

#[derive(Debug)]
pub struct Avp {
    header: AvpHeader,
    pub(super) value: AvpValue,
}

#[derive(Debug)]
pub struct AvpHeader {
    code: u32,
    flags: u8,
    length: u32, // 24 bits | how many octets in the AVP
    vendor_id: Option<u32>,
}

#[derive(Debug)]
pub enum AvpFlags {
    M, // Mandatory
    O, // Optional
}

#[derive(Debug)]
pub enum AvpValue {
    AddressIPv4(IPv4),
    AddressIPv6(IPv6),
    Identity(Identity),
    DiameterURI(DiameterURI),
    Enumerated(Enumerated),
    Float32(Float32),
    Float64(Float64),
    Grouped(Grouped),
    Integer32(Integer32),
    Integer64(Integer64),
    OctetString(OctetString),
    Time(Time),
    Unsigned32(Unsigned32),
    Unsigned64(Unsigned64),
    UTF8String(UTF8String),
}

impl AvpFlags {
    const VENDOR_FLAG_BIT: u8 = 0b10000000;

    fn value(&self) -> u8 {
        match self {
            Self::M => 0b01000000,
            Self::O => 0b01000000,
        }
    }

    fn with_vendor_bit(&self) -> u8 {
        self.value() | Self::VENDOR_FLAG_BIT
    }
}

impl AvpHeader {
    fn encode_to<W: Write>(&self, writer: &mut W) -> DiameterResult<()> {
        writer.write_all(&self.code.to_be_bytes())?;
        writer.write(&[self.flags])?;
        writer.write_all(&self.length.to_be_bytes()[1..])?;
        match self.vendor_id {
            None => Ok(()),
            Some(vendor_id) => {
                writer.write_all(&vendor_id.to_be_bytes())?;
                Ok(())
            }
        }
    }
}

impl Avp {
    pub fn new<T: Into<AvpValue>>(
        code: u32,
        flags: AvpFlags,
        vendor_id: Option<u32>,
        value: T,
    ) -> Self {
        let (length, avp_flags) = match vendor_id {
            Some(_) => (12, flags.with_vendor_bit()),
            None => (8, flags.value()),
        };
        Self {
            header: AvpHeader {
                code,
                flags: avp_flags,
                length,
                vendor_id,
            },
            value: value.into(),
        }
    }

    pub fn encode_to<W: Write>(&mut self, writer: &mut W) -> DiameterResult<()> {
        self.header.encode_to(writer)?;
        self.value.encode(&mut self.header, writer)?;
        self.add_padding(writer)?;
        Ok(())
    }

    pub fn get_length(&self) -> u32 {
        self.header.length
    }

    fn add_padding<W: Write>(&mut self, writer: &mut W) -> DiameterResult<()> {
        let remainder = self.header.length % 4;
        if remainder != 0 {
            println!("{}", remainder);
            let padding = 4 - remainder;
            self.header.length += padding;
            for _ in 0..padding {
                writer.write(&[0])?;
            }
        }
        Ok(())
    }
}

macro_rules! impl_encode_avp_value_for_enum_variants {
    ($enum_name:ident { $($variant:ident($inner_ty:ty)),* }) => {
        impl $enum_name {
            fn encode<W: Write>(&mut self, avp_header: &mut AvpHeader, writer: &mut W) -> DiameterResult<()> {
                match self {
                    $(
                        $enum_name::$variant(value) => {
                            value.encode_to(writer)?;
                            avp_header.length += value.len();
                        }
                    )*
                    _ => todo!("Unhandled AvpValue variants to encode data"),
                }
                Ok(())
            }
        }
    };
}

impl_encode_avp_value_for_enum_variants!(AvpValue {
    AddressIPv4(IPv4),
    AddressIPv6(IPv6),
    Identity(Identity),
    DiameterURI(DiameterURI),
    Enumerated(Enumerated),
    Float32(Float32),
    Float64(Float64),
    Grouped(Grouped),
    Integer32(Integer32),
    Integer64(Integer64),
    OctetString(OctetString),
    Time(Time),
    Unsigned32(Unsigned32),
    Unsigned64(Unsigned64),
    UTF8String(UTF8String)
});
