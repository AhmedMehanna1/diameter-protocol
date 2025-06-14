use crate::modeling::avp::data::{AvpData, AvpDataFormater};
use chrono::{DateTime, Utc};
use std::rc::Rc;

pub type Time = AvpData<DateTime<Utc>>;

const RFC868_OFFSET: u32 = 2208988800; // Diff. between 1970 and 1900 in seconds.

impl AvpDataFormater for Time {
    fn encode(&mut self) -> Rc<Vec<u8>> {
        match &self.encoded_value {
            Some(encoded_value) => Rc::clone(&encoded_value),
            None => {
                let unix_timestamp = self.raw_value.timestamp();
                let diameter_timestamp = unix_timestamp + RFC868_OFFSET as i64;
                let diameter_timestamp = diameter_timestamp as u32;
                let encoded_data = Rc::new(Vec::from(diameter_timestamp.to_be_bytes()));
                self.encoded_value = Some(Rc::clone(&encoded_data));
                encoded_data
            }
        }
    }
}
