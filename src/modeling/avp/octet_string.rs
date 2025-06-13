use crate::modeling::avp::data::{AvpData, AvpDataFormater};
use std::rc::Rc;

pub type OctetString = AvpData<Vec<u8>>;

impl AvpDataFormater for OctetString {
    fn encode(&mut self) -> Rc<Vec<u8>> {
        match &self.encoded_value {
            Some(encoded_value) => Rc::clone(&encoded_value),
            None => {
                let encoded_data = Rc::new(self.raw_value.clone());
                self.encoded_value = Some(Rc::clone(&encoded_data));
                encoded_data
            }
        }
    }
}