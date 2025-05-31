pub struct AVP {
    code: u32,
    flags: AvpFlags,
    length: u32, // 24 bits
    vendor_id: u32,
    data: Vec<u8>,
}

pub enum AvpFlags {
    MANDATORY = 0b01000000,
    OPTIONAL = 0b00000000,
}
