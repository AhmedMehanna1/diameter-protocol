use crate::modeling::avp::data::AvpDataFormater;
use std::fmt::Debug;
use std::rc::Rc;

#[derive(Debug)]
pub struct Avp {
    code: u32,
    flags: u8,
    length: u32, // 24 bits | how many octets in the AVP
    vendor_id: Option<u32>,
    raw_data: Rc<Vec<u8>>,
    encoded_data: Option<Rc<Vec<u32>>>,
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
        match vendor_id {
            Some(_) => Self {
                code,
                flags: flags.value(),
                length: 12 + encoded_data.len() as u32,
                vendor_id,
                raw_data: value.encode(),
                encoded_data: None,
            },
            None => Self {
                code,
                flags: flags.with_vendor_bit(),
                length: 8 + encoded_data.len() as u32,
                vendor_id,
                raw_data: value.encode(),
                encoded_data: None,
            },
        }
    }

    pub(super) fn get_avp_encoded_data(&self) -> Rc<Vec<u8>> {
        Rc::clone(&self.raw_data)
    }

    pub fn encode(&mut self) -> Rc<Vec<u32>> {
        match self.encoded_data {
            Some(ref encoded_data) => encoded_data.clone(),
            None => {
                let mut encoded_data = vec![];
                encoded_data.push(self.code);
                let masked_length = self.length & 0x00ffffffu32;
                let flags_and_length = (self.flags as u32) << 24 | masked_length;
                encoded_data.push(flags_and_length);
                match self.vendor_id {
                    None => {}
                    Some(v_id) => {
                        encoded_data.push(v_id);
                    }
                }
                let mut collated_data = [0u8; 4];
                for (i, el) in self.raw_data.iter().enumerate() {
                    collated_data[i % 4] = *el;
                    if (i + 1) % 4 == 0 {
                        let mut encoded_u32 = 0u32;
                        for j in 0..4 {
                            encoded_u32 = encoded_u32 | collated_data[j] as u32;
                            if j != 3 {
                                encoded_u32 = encoded_u32 << 8;
                            }
                        }
                        encoded_data.push(encoded_u32);
                    }
                }
                let remainder = self.raw_data.len() % 4;
                if remainder != 0 {
                    let mut encoded_u32 = 0u32;
                    for i in 0..remainder {
                        encoded_u32 = encoded_u32 | collated_data[i] as u32;
                        if i < remainder - 1 {
                            encoded_u32 = encoded_u32 << 8;
                        }
                    }
                    encoded_data.push(encoded_u32);
                }
                let rc_encoded_data = Rc::new(encoded_data);
                self.encoded_data = Some(Rc::clone(&rc_encoded_data));
                rc_encoded_data
            }
        }
    }
}