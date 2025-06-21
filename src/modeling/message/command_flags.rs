#[derive(Debug)]
pub enum CommandFlag {
    Request,
    Proxyable,
    Error,
    Retransmit,
}

impl CommandFlag {
    pub fn value(&self) -> u8 {
        match self {
            CommandFlag::Request => 0x80,
            CommandFlag::Proxyable => 0x40,
            CommandFlag::Error => 0x20,
            CommandFlag::Retransmit => 0x10,
        }
    }
}
