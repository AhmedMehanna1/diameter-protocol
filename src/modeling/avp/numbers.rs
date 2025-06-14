use crate::modeling::avp::data::AvpData;

pub type Integer32 = AvpData<i32>;
pub type Integer64 = AvpData<i64>;
pub type Unsigned32 = AvpData<u32>;
pub type Unsigned64 = AvpData<u64>;
pub type Float32 = AvpData<f32>;
pub type Float64 = AvpData<f64>;
pub type Enumerated = Integer32;
