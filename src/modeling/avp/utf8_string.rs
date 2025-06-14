use crate::modeling::avp::data::{AvpData, AvpDataFormater};
use std::rc::Rc;

pub type UTF8String = AvpData<String>;
pub type Identity = UTF8String;

impl AvpDataFormater for UTF8String {
    fn encode(&mut self) -> Rc<Vec<u8>> {
        match &self.encoded_value {
            Some(encoded_value) => Rc::clone(&encoded_value),
            None => {
                let encoded_data = Rc::new(Vec::from(self.raw_value.as_bytes()));
                self.encoded_value = Some(Rc::clone(&encoded_data));
                encoded_data
            }
        }
    }
}
