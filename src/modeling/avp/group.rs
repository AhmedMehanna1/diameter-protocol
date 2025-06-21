use crate::errors::DiameterResult;
use crate::modeling::avp::avp::{Avp, AvpFlags, AvpValue};
use crate::modeling::avp::data::{AvpData, AvpDataFormater};
use std::io::{Read, Write};

pub type Grouped = AvpData<Vec<Avp>>;

impl AvpDataFormater for Grouped {
    type Output = Vec<Avp>;

    fn encode_to<W: Write>(&mut self, writer: &mut W) -> DiameterResult<()> {
        for avp in &mut self.0 {
            avp.encode_to(writer)?;
        }
        Ok(())
    }

    fn decode_from<R: Read>(
        reader: &mut R,
        length: Option<usize>,
    ) -> DiameterResult<AvpData<Self::Output>> {
        todo!()
    }

    fn len(&self) -> u32 {
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

    pub fn add_avp(&mut self, code: u32, vendor_id: Option<u32>, flags: AvpFlags, value: AvpValue) {
        let avp = Avp::new(code, flags, vendor_id, value);
        self.add(avp);
    }
}

impl Into<AvpValue> for Grouped {
    fn into(self) -> AvpValue {
        AvpValue::Grouped(self)
    }
}
