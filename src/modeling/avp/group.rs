use crate::modeling::avp::avp::Avp;
use crate::modeling::avp::data::{AvpData, AvpDataFormater};
use std::rc::Rc;

pub type Grouped = AvpData<Vec<Avp>>;

impl AvpDataFormater for Grouped {
    fn encode(&mut self) -> Rc<Vec<u8>> {
        match &self.encoded_value {
            Some(encoded_value) => Rc::clone(&encoded_value),
            None => {
                let mut encoded_data: Vec<u8> = vec![];
                for avp in self.raw_value.iter() {
                    let mut encoded_avp: Vec<u8> = (*avp.get_avp_encoded_data()).clone();
                    encoded_data.append(&mut encoded_avp);
                }
                Rc::new(encoded_data)
            }
        }
    }
}
