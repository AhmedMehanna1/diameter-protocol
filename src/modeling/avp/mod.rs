pub mod avp;
pub mod enumerated;
pub mod float32;
pub mod float64;
pub mod group;
pub mod integer32;
pub mod integer64;
pub mod ipv4;
pub mod ipv6;
pub mod octet_string;
pub mod time;
pub mod unsigned32;
pub mod unsigned64;
pub mod utf8_string;

#[macro_use]
pub mod macros;

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
