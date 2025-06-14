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

use crate::modeling::avp::data::AvpDataFormater;
use std::fmt::Debug;
use std::ops::Deref;
use std::rc::Rc;

#[derive(Debug)]
pub struct Avp {
    code: u32,
    flags: u8,
    length: u32, // 24 bits | how many octets in the AVP
    vendor_id: Option<u32>,
    raw_data: Rc<Vec<u8>>,
    pub(crate) encoded_data: Option<Rc<Vec<u8>>>,
}

#[derive(Debug)]
pub enum AvpFlags {
    M, // Mandatory
    O, // Optional
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

impl Avp {
    pub fn new(
        code: u32,
        flags: AvpFlags,
        vendor_id: Option<u32>,
        mut value: impl AvpDataFormater,
    ) -> Self {
        let encoded_data = value.encode();
        let (length, avp_flags) = match vendor_id {
            Some(_) => (12, flags.with_vendor_bit()),
            None => (8, flags.value()),
        };
        Self {
            code,
            flags: avp_flags,
            length: length + encoded_data.len() as u32,
            vendor_id,
            raw_data: encoded_data,
            encoded_data: None,
        }
    }

    pub fn encode(&mut self) -> Rc<Vec<u8>> {
        match self.encoded_data {
            Some(ref encoded_data) => Rc::clone(encoded_data),
            None => {
                let mut encoded_data: Vec<u8> = vec![];
                encoded_data.extend_from_slice(&self.code.to_be_bytes());
                encoded_data.push(self.flags);
                encoded_data.extend_from_slice(&self.length.to_be_bytes()[1..]);
                match self.vendor_id {
                    None => {}
                    Some(vendor_id) => {
                        encoded_data.extend(vendor_id.to_be_bytes());
                    }
                }
                encoded_data.extend(self.raw_data.deref());
                let remainder = self.raw_data.len() % 4;
                if remainder != 0 {
                    println!("{}", remainder);
                    for _ in 0..4 - remainder {
                        encoded_data.push(0);
                    }
                }
                let rc_encoded_data = Rc::new(encoded_data);
                self.encoded_data = Some(Rc::clone(&rc_encoded_data));
                rc_encoded_data
            }
        }
    }
}
