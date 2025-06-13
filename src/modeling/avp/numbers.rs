use crate::modeling::avp::data::{AvpData, AvpDataFormater};
use std::rc::Rc;

pub type Integer32 = AvpData<i32>;
pub type Integer64 = AvpData<i64>;
pub type Unsigned32 = AvpData<u32>;
pub type Unsigned64 = AvpData<u64>;
pub type Float32 = AvpData<f32>;
pub type Float64 = AvpData<f64>;

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

impl<T: ToBeBytes> AvpDataFormater for AvpData<T> {
    fn encode(&mut self) -> Rc<Vec<u8>> {
        match &self.encoded_value {
            Some(encoded_value) => Rc::clone(&encoded_value),
            None => {
                let encoded_data = Rc::new(Vec::from(self.raw_value.to_be_bytes()));
                self.encoded_value = Some(Rc::clone(&encoded_data));
                encoded_data
            }
        }
    }
}