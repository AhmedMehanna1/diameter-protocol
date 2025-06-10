use std::fmt::Debug;
use std::rc::Rc;

#[derive(Debug)]
pub struct Avp {
    code: u32,
    flags: u8,
    length: u32, // 24 bits | how many octets in the AVP
    vendor_id: Option<u32>,
    data: Rc<Vec<u8>>,
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

#[derive(Debug)]
pub struct AvpData<T> {
    raw_value: T,
    encoded_value: Option<Rc<Vec<u8>>>,
}

impl Avp {
    pub fn new(
        code: u32,
        flags: AvpFlags,
        vendor_id: Option<u32>,
        mut value: Box<dyn ApvDataFormater>,
    ) -> Self {
        let encoded_data = value.encode();
        match vendor_id {
            Some(_) => Self {
                code,
                flags: flags.value(),
                length: 8 + encoded_data.len() as u32,
                vendor_id,
                data: value.encode(),
            },
            None => Self {
                code,
                flags: flags.with_vendor_bit(),
                length: 12 + encoded_data.len() as u32,
                vendor_id,
                data: value.encode(),
            },
        }
    }

    pub fn encode(&self) -> Vec<u32> {
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
        for (i, el) in self.data.iter().enumerate() {
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
        let remainder = self.data.len() % 4;
        if remainder != 0 {
            let mut encoded_u32 = 0u32;
            for i in 0..remainder {
                encoded_u32 = encoded_u32 & collated_data[i] as u32;
                encoded_u32 = encoded_u32 << 8;
            }
            encoded_data.push(encoded_u32);
        }
        encoded_data
    }
}

impl<T> AvpData<T> {
    pub fn new(data: T) -> Self {
        Self {
            raw_value: data,
            encoded_value: None,
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
pub type Grouped<'a> = AvpData<Vec<&'a Avp>>;

pub trait ApvDataFormater {
    fn encode(&mut self) -> Rc<Vec<u8>>;
}

pub trait ToBeBytes {
    fn to_be_bytes(&self) -> Vec<u8>;
}

macro_rules! impl_to_be_bytes {
    ($($t:ty),*) => {
        $(
            impl ToBeBytes for $t {
                fn to_be_bytes(&self) -> Vec<u8> {
                    <$t>::to_be_bytes(*self).to_vec()
                }
            }
        )*
    };
}

impl_to_be_bytes!(u8, i32, i64, u32, u64, f32, f64);

impl<T: ToBeBytes> ApvDataFormater for AvpData<T> {
    fn encode(&mut self) -> Rc<Vec<u8>> {
        let encoded_data = Rc::new(Vec::from(self.raw_value.to_be_bytes()));
        self.encoded_value = Some(Rc::clone(&encoded_data));
        encoded_data
    }
}

impl<'a> ApvDataFormater for Grouped<'a> {
    fn encode(&mut self) -> Rc<Vec<u8>> {
        let mut collated_data: Vec<Vec<u8>> = vec![];
        for avp in self.raw_value.iter() {
            collated_data.push(Vec::from(avp.code.to_be_bytes()));
            collated_data.push(vec![avp.flags]);
            collated_data.push(Vec::from(avp.length.to_be_bytes()));
            match avp.vendor_id {
                Some(v_id) => {
                    collated_data.push(Vec::from(v_id.to_be_bytes()));
                }
                _ => {}
            }
            // collated_data.push(avp.data.clone());
        }
        let encoded_data = Rc::new(
            collated_data
                .iter()
                .flat_map(|el| el.iter())
                .cloned()
                .collect(),
        );
        self.encoded_value = Some(Rc::clone(&encoded_data));
        encoded_data
    }
}
