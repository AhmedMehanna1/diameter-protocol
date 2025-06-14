use crate::modeling::avp::data::{AvpData, AvpDataFormater};
use std::net::{Ipv4Addr, Ipv6Addr};
use std::rc::Rc;

pub trait EncodeAsBytes {
    fn encode_bytes(&self) -> Vec<u8>;
}

macro_rules! impl_encode_as_bytes_for_ip {
    ($($t:ty),*) => {
        $(
            impl EncodeAsBytes for $t {
                fn encode_bytes(&self) -> Vec<u8> {
                    <$t>::octets(self).to_vec()
                }
            }
        )*
    };
}

macro_rules! impl_encode_as_bytes_for_num {
    ($($t:ty),*) => {
        $(
            impl EncodeAsBytes for $t {
                fn encode_bytes(&self) -> Vec<u8> {
                    self.to_be_bytes().to_vec()
                }
            }
        )*
    };
}

impl_encode_as_bytes_for_ip!(Ipv4Addr, Ipv6Addr);
impl_encode_as_bytes_for_num!(u8, i32, i64, u32, u64, f32, f64);

impl<T: EncodeAsBytes> AvpDataFormater for AvpData<T> {
    fn encode(&mut self) -> Rc<Vec<u8>> {
        match self.encoded_value {
            Some(ref encoded_value) => Rc::clone(encoded_value),
            None => {
                let encoded_data = Rc::new(self.raw_value.encode_bytes());
                self.encoded_value = Some(Rc::clone(&encoded_data));
                encoded_data
            }
        }
    }
}
