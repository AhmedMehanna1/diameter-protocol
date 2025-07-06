use std::fmt::Debug;

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
