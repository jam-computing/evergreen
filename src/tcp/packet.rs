static PROTOCOL_VERSION: u8 = 1;

// TODO: totes rename this
// TODO: Restructure Protocl Spec

#[allow(dead_code)]
pub struct ProtocolSpec {
    pub version: u8,
    pub command: ProtocolCommand,
}

#[allow(dead_code)]
impl ProtocolSpec {
    pub fn new() -> Self {
        Self {
            version: PROTOCOL_VERSION,
            command: ProtocolCommand::None,
        }
    }

    pub fn command(command: ProtocolCommand) -> Self {
        Self {
            version: PROTOCOL_VERSION,
            command
        }
    }

    pub fn from(buf: &[u8]) -> Option<Self> {
        let mut packet: Self = Self::new();
        packet.version = buf[0];

        if packet.version != PROTOCOL_VERSION {
            return None;
        }

        packet.command = ProtocolCommand::from_byte(buf[1]);

        Some(packet)
    }

    pub fn into_bytes(&self) -> Vec<u8> {
        let mut buf: Vec<u8> = Vec::new();

        buf.push(self.version);
        buf.push(ProtocolCommand::to_byte(&self.command));
        buf
    }
}

#[allow(dead_code)]
pub enum ProtocolCommand {
    None,
    Init,
}

#[allow(dead_code)]
impl ProtocolCommand {
    pub fn to_byte(&self) -> u8 {
        match self {
            ProtocolCommand::None => 0,
            ProtocolCommand::Init => 1,
        }
    }

    pub fn from_byte(b: u8) -> Self {
        match b {
            1 => ProtocolCommand::Init,
            _ => ProtocolCommand::None,
        }
    }
}
