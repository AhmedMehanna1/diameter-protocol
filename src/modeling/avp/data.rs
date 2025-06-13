use std::fmt::Debug;
use std::rc::Rc;

#[derive(Debug)]
pub struct AvpData<T> {
    pub(super) raw_value: T,
    pub(super) encoded_value: Option<Rc<Vec<u8>>>,
}

impl<T> AvpData<T> {
    pub fn new(data: T) -> Self {
        Self {
            raw_value: data,
            encoded_value: None,
        }
    }
}

pub trait AvpDataFormater {
    fn encode(&mut self) -> Rc<Vec<u8>>;
}