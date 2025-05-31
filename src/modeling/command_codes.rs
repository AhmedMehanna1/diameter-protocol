#[derive(Debug)]
pub struct CommandCode {
    name: &'static str,
    code: u32, // 24 bits
}

impl CommandCode {
    const fn new(name: &'static str, code: u32) -> Self {
        Self { name, code }
    }

    pub fn get_name(&self) -> &'static str {
        self.name
    }

    pub fn get_code(&self) -> u32 {
        self.code
    }
}

pub static ASR: CommandCode = CommandCode::new("Abort-Session-Request", 274);
pub static ASA: CommandCode = CommandCode::new("Abort-Session-Answer", 274);
pub static ACR: CommandCode = CommandCode::new("Accounting-Request", 271);
pub static ACA: CommandCode = CommandCode::new("Accounting-Response", 271);
pub static CER: CommandCode = CommandCode::new("Capabilities-Exchange-Request", 257);
pub static CEA: CommandCode = CommandCode::new("Capabilities-Exchange-Answer", 257);
pub static DWR: CommandCode = CommandCode::new("Device-Watchdog-Request", 280);
pub static DWA: CommandCode = CommandCode::new("Device-Watchdog-Answer", 280);
pub static DPR: CommandCode = CommandCode::new("Disconnect-Peer-Request", 282);
pub static DPA: CommandCode = CommandCode::new("Disconnect-Peer-Answer", 282);
pub static RAR: CommandCode = CommandCode::new("Re-Auth-Request", 258);
pub static RAA: CommandCode = CommandCode::new("Re-Auth-Answer", 258);
pub static STR: CommandCode = CommandCode::new("Session-Termination-Request", 275);
pub static STA: CommandCode = CommandCode::new("Session-Termination-Answer", 275);
