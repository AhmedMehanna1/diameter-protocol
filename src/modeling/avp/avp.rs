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
use crate::modeling::message::command_code::CommandCode;
use crate::modeling::message::dictionary::Dictionary;
use std::fmt::Debug;
use std::io::{Read, Write};
use std::sync::Arc;

#[derive(Debug)]
pub struct Avp {
    header: AvpHeader,
    pub(super) value: AvpValue,
}

#[derive(Debug)]
pub struct AvpHeader {
    code: u32,
    flags: u8,
    pub(crate) length: u32, // 24 bits | how many octets in the AVP
    vendor_id: Option<u32>,
}

#[derive(Debug)]
pub enum AvpFlags {
    M, // Mandatory
    O, // Optional
}

#[derive(Debug)]
pub enum AvpType {
    Unknown,
    AddressIPv4,
    AddressIPv6,
    Identity,
    DiameterURI,
    Enumerated,
    Float32,
    Float64,
    Grouped,
    Integer32,
    Integer64,
    OctetString,
    Time,
    Unsigned32,
    Unsigned64,
    UTF8String,
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

    fn has_vendor_bit(flag: u8) -> bool {
        if Self::VENDOR_FLAG_BIT & flag == Self::VENDOR_FLAG_BIT {
            true
        } else {
            false
        }
    }
}

impl AvpHeader {
    fn encode_to<W: Write>(&self, avp_length: u32, writer: &mut W) -> DiameterResult<()> {
        writer.write_all(&self.code.to_be_bytes())?;
        writer.write(&[self.flags])?;
        println!("avp_length: {}", avp_length);
        writer.write_all(&avp_length.to_be_bytes()[1..])?;
        match self.vendor_id {
            Some(vendor_id) => {
                writer.write_all(&vendor_id.to_be_bytes())?;
                Ok(())
            }
            None => Ok(()),
        }
    }

    pub fn decode_from<R: Read>(reader: &mut R) -> DiameterResult<Self> {
        let mut b = [0u8; 8];
        reader.read_exact(&mut b)?;
        for i in 1..b.len() + 1 {
            print!("{:08b} ", b[i - 1]);
            if i % 4 == 0 {
                println!()
            }
        }
        let command_code = u32::from_be_bytes([b[0], b[1], b[2], b[3]]);
        let flag = b[4];
        let length = u32::from_be_bytes([0, b[5], b[6], b[7]]);
        let header = AvpHeader {
            code: command_code,
            flags: flag,
            length,
            vendor_id: match AvpFlags::has_vendor_bit(flag) {
                false => None,
                true => {
                    let mut b = [0u8; 4];
                    reader.read_exact(&mut b)?;
                    Some(u32::from_be_bytes([b[0], b[1], b[2], b[3]]))
                }
            },
        };
        Ok(header)
    }
}

impl Avp {
    pub fn new<T: Into<AvpValue>>(
        code: CommandCode,
        flags: AvpFlags,
        vendor_id: Option<u32>,
        value: T,
    ) -> Self {
        let avp_value: AvpValue = value.into();
        let (length, avp_flags) = match vendor_id {
            Some(_) => (12 + avp_value.len(), flags.with_vendor_bit()),
            None => (8 + avp_value.len(), flags.value()),
        };
        Self {
            header: AvpHeader {
                code: code.get_code(),
                flags: avp_flags,
                length,
                vendor_id,
            },
            value: avp_value,
        }
    }

    pub fn encode_to<W: Write>(&self, writer: &mut W) -> DiameterResult<()> {
        self.header.encode_to(self.get_length(), writer)?;
        self.value.encode(writer)?;
        self.add_padding(writer)?;
        Ok(())
    }

    pub fn decode_from<R: Read>(reader: &mut R, dict: Arc<Dictionary>) -> DiameterResult<Self> {
        let header = AvpHeader::decode_from(reader)?;

        let avp_type = dict
            .get_avp_type(header.code, header.vendor_id)
            .unwrap_or(&AvpType::Unknown);
        dbg!(avp_type);

        let value: AvpValue = match avp_type {
            AvpType::Unsigned32 => Unsigned32::decode_from(reader)?.into(),
            AvpType::Identity => Identity::decode_from(
                reader,
                match header.vendor_id {
                    Some(_) => Some((header.length - 12) as usize),
                    None => Some((header.length - 8) as usize),
                },
            )?
            .into(),
            AvpType::UTF8String => UTF8String::decode_from(
                reader,
                match header.vendor_id {
                    Some(_) => Some((header.length - 12) as usize),
                    None => Some((header.length - 8) as usize),
                },
            )?
            .into(),
            AvpType::Enumerated => Enumerated::decode_from(reader)?.into(),
            AvpType::Grouped => Grouped::decode_from(
                reader,
                match header.vendor_id {
                    Some(_) => (header.length - 12) as usize,
                    None => (header.length - 8) as usize,
                },
                Arc::clone(&dict),
            )?
            .into(),
            _ => todo!(),
        };
        let avp = Self { header, value };
        let mut vec = vec![0u8; avp.get_padding() as usize];
        reader.read_exact(&mut vec)?;
        Ok(avp)
    }

    pub fn get_length(&self) -> u32 {
        self.header.length
    }

    pub fn get_padding(&self) -> u32 {
        let remainder = (self.header.length + self.value.len()) % 4;
        if remainder != 0 { 4 - remainder + 1 } else { 0 }
    }

    fn add_padding<W: Write>(&self, writer: &mut W) -> DiameterResult<()> {
        for _ in 0..self.get_padding() {
            writer.write(&[0])?;
        }
        Ok(())
    }
}

macro_rules! impl_encode_avp_value_for_enum_variants {
    ($enum_name:ident { $($variant:ident($inner_ty:ty)),* }) => {
        impl $enum_name {
            fn encode<W: Write>(
                &self,
                writer: &mut W
            ) -> DiameterResult<()> {
                match self {
                    $(
                        $enum_name::$variant(value) => {
                            value.encode_to(writer)?;
                        }
                    )*
                }
                Ok(())
            }

            fn len(&self) -> u32 {
                match self {
                    $(
                        $enum_name::$variant(value) => {
                            value.len()
                        }
                    )*
                }
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
