use std::fmt::Debug;

#[derive(Debug)]
pub struct AVP {
    code: u32,
    flags: u8,
    length: u32, // 24 bits | how many octets in the AVP
    vendor_id: u32,
    pub data: Vec<u8>,
}

#[derive(Debug)]
pub enum AvpFlags {
    MANDATORY, // = 0b01000000,
    OPTIONAL,  // = 0b00000000,
}

impl AvpFlags {
    fn value(&self) -> u8 {
        match self {
            Self::MANDATORY => 0b01000000,
            Self::OPTIONAL => 0b01000000,
        }
    }
}

#[derive(Debug)]
pub struct AvpData<T> {
    data: T,
    encoded_data: Option<Vec<u8>>,
}

impl AVP {
    const VENDOR_FLAG_BIT: u8 = 0b10000000;

    pub fn new(code: u32, flags: AvpFlags) -> Self {
        Self {
            code,
            flags: flags.value(),
            length: 8,
            vendor_id: 0,
            data: vec![],
        }
    }

    pub fn new_with_vendor(code: u32, flags: AvpFlags, vendor_id: u32) -> Self {
        let flag_byte = flags.value() | Self::VENDOR_FLAG_BIT;
        Self {
            code,
            flags: flag_byte,
            length: 12,
            vendor_id,
            data: vec![],
        }
    }

    pub fn add_data<T: AppDataFormater>(&mut self, avp: &T) {
        for el in avp.encode() {
            self.data.push(el);
            self.length += 1;
        }
    }
}

impl<T> AvpData<T> {
    pub fn new(data: T) -> Self {
        Self {
            data,
            encoded_data: None,
        }
    }
}

pub type OctetString = AvpData<u8>;
pub type Integer32 = AvpData<i32>;
pub type Integer64 = AvpData<i64>;
pub type Unsigned32 = AvpData<u32>;
pub type Unsigned64 = AvpData<u64>;
pub type Float32 = AvpData<f32>;
pub type Float64 = AvpData<f64>;
pub type Grouped<'a> = AvpData<Vec<&'a AVP>>;

pub trait AppDataFormater {
    fn encode(&self) -> Vec<u8>;
}

impl AppDataFormater for OctetString {
    fn encode(&self) -> Vec<u8> {
        Vec::from(self.data.to_be_bytes())
    }
}

impl AppDataFormater for Integer32 {
    fn encode(&self) -> Vec<u8> {
        Vec::from(self.data.to_be_bytes())
    }
}

impl AppDataFormater for Integer64 {
    fn encode(&self) -> Vec<u8> {
        Vec::from(self.data.to_be_bytes())
    }
}

impl AppDataFormater for Unsigned32 {
    fn encode(&self) -> Vec<u8> {
        Vec::from(self.data.to_be_bytes())
    }
}

impl AppDataFormater for Unsigned64 {
    fn encode(&self) -> Vec<u8> {
        Vec::from(self.data.to_be_bytes())
    }
}

impl AppDataFormater for Float32 {
    fn encode(&self) -> Vec<u8> {
        Vec::from(self.data.to_be_bytes())
    }
}

impl AppDataFormater for Float64 {
    fn encode(&self) -> Vec<u8> {
        Vec::from(self.data.to_be_bytes())
    }
}

impl<'a> AppDataFormater for Grouped<'a> {
    fn encode(&self) -> Vec<u8> {
        let mut encoded_date: Vec<Vec<u8>> = vec![];
        for avp in self.data.iter() {
            encoded_date.push(Vec::from(avp.code.to_be_bytes()));
            encoded_date.push(vec![avp.flags]);
            encoded_date.push(Vec::from(avp.length.to_be_bytes()));
            if avp.flags & AVP::VENDOR_FLAG_BIT == AVP::VENDOR_FLAG_BIT {
                encoded_date.push(Vec::from(avp.vendor_id.to_be_bytes()));
            }
            encoded_date.push(avp.data.clone());
        }
        encoded_date
            .iter()
            .flat_map(|el| el.iter())
            .cloned()
            .collect()
    }
}
