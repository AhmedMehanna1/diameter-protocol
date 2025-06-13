use crate::modeling::avp::data::{AvpData, AvpDataFormater};
use std::rc::Rc;

pub type UTF8String<'a> = AvpData<&'a str>;

impl<'a> AvpDataFormater for UTF8String<'a> {
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
