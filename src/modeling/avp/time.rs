use crate::errors::Error::EncodeError;
use crate::errors::{DiameterResult, Error};
use crate::modeling::avp::avp::AvpValue;
use crate::modeling::avp::data::AvpData;
use chrono::{DateTime, TimeZone, Utc};
use std::io::{Read, Write};

pub type Time = AvpData<DateTime<Utc>>;

const RFC868_OFFSET: u32 = 2208988800; // Diff. between 1970 and 1900 in seconds.

impl Time {
    pub(super) fn encode_to<W: Write>(&self, writer: &mut W) -> DiameterResult<()> {
        let unix_timestamp = self.0.timestamp();
        let diameter_timestamp = unix_timestamp + RFC868_OFFSET as i64;
        if diameter_timestamp > u32::MAX as i64 {
            Err(EncodeError(
                "Time is too far in the future to fit into 32 bits",
            ))?
        }
        let diameter_timestamp = diameter_timestamp as u32;
        let encoded_data = Vec::from(diameter_timestamp.to_be_bytes());
        writer.write(&encoded_data)?;
        Ok(())
    }

    pub(super) fn decode_from<R: Read>(reader: &mut R) -> DiameterResult<AvpData<DateTime<Utc>>> {
        let mut b = [0; 4];
        reader.read_exact(&mut b)?;

        let diameter_timestamp = u32::from_be_bytes(b); // seconds since 1900
        let unix_timestamp = diameter_timestamp as i64 - RFC868_OFFSET as i64;
        let timestamp = Utc
            .timestamp_opt(unix_timestamp, 0)
            .single()
            .ok_or_else(|| Error::DecodeError("Invalid time"))?;
        Ok(Time::new(timestamp))
    }

    pub(super) fn len(&self) -> u32 {
        4
    }
}

impl Into<AvpValue> for Time {
    fn into(self) -> AvpValue {
        AvpValue::Time(self)
    }
}
