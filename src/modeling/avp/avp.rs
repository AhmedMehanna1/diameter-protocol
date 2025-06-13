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
    pub(super) encoded_data: Option<Rc<Vec<u8>>>,
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
        mut value: Box<dyn AvpDataFormater>,
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
            raw_data: value.encode(),
            encoded_data: None,
        }
    }

    pub fn encode(&mut self) -> Rc<Vec<u8>> {
        match self.encoded_data {
            Some(ref encoded_data) => Rc::clone(encoded_data),
            None => {
                let mut encoded_data: Vec<u8> = vec![];
                encoded_data.extend_from_slice(&self.code.to_be_bytes());
                let masked_length = self.length & 0x00ffffffu32;
                let flags_and_length = (self.flags as u32) << 24 | masked_length;
                encoded_data.extend(flags_and_length.to_be_bytes());
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
