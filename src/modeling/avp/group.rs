use crate::errors::DiameterResult;
use crate::modeling::avp::avp::{Avp, AvpFlags, AvpValue};
use crate::modeling::avp::data::AvpData;
use crate::modeling::message::command_code::CommandCode;
use crate::modeling::message::dictionary::Dictionary;
use std::io::{Read, Write};
use std::sync::Arc;

pub type Grouped = AvpData<Vec<Avp>>;

impl Grouped {
    pub(super) fn encode_to<W: Write>(&self, writer: &mut W) -> DiameterResult<()> {
        for avp in &self.0 {
            avp.encode_to(writer)?;
        }
        Ok(())
    }

    pub(super) fn decode_from<R: Read>(
        reader: &mut R,
        length: usize,
        dict: Arc<Dictionary>,
    ) -> DiameterResult<AvpData<Vec<Avp>>> {
        let mut avps_length = length;
        let mut avps: Vec<Avp> = Vec::new();
        while avps_length > 0 {
            let avp = Avp::decode_from(reader, Arc::clone(&dict))?;
            avps_length -= avp.get_length() as usize;
            avps.push(avp);
        }
        Ok(AvpData(avps))
    }

    pub(super) fn len(&self) -> u32 {
        self.0.iter().map(|avp| avp.get_length()).sum()
    }
}

impl Grouped {
    pub fn avps(&self) -> &Vec<Avp> {
        &self.0
    }

    pub fn add(&mut self, avp: Avp) {
        self.0.push(avp);
    }

    pub fn add_avp(
        &mut self,
        code: CommandCode,
        vendor_id: Option<u32>,
        flags: AvpFlags,
        value: AvpValue,
    ) {
        let avp = Avp::new(code, flags, vendor_id, value);
        self.add(avp);
    }
}

impl Into<AvpValue> for Grouped {
    fn into(self) -> AvpValue {
        AvpValue::Grouped(self)
    }
}
