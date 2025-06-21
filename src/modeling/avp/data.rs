use crate::errors::DiameterResult;
use std::fmt::Debug;
use std::io::{Read, Write};

#[derive(Debug)]
pub struct AvpData<T>(pub(super) T);

impl<T> AvpData<T> {
    pub fn new(data: T) -> Self {
        Self(data)
    }

    pub fn value(&self) -> &T {
        &self.0
    }
}

pub trait AvpDataFormater {
    type Output;

    fn encode_to<W: Write>(&self, writer: &mut W) -> DiameterResult<()>;
    fn decode_from<R: Read>(
        reader: &mut R,
        length: Option<usize>,
    ) -> DiameterResult<AvpData<Self::Output>>;
    fn len(&self) -> u32;
}
