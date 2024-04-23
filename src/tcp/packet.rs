use crate::tcp::command;

static PROTOCOL_VERSION: u8 = 1;

// TODO: totes rename this
// TODO: Restructure Protocl Spec

#[allow(dead_code)]
pub struct ProtocolPacket {
    pub version: u8,
    pub command: command::ProtocolCommand,
    pub status: u16,
    pub id: u16,
    pub data: Option<String>
}

#[allow(dead_code)]
impl ProtocolPacket {
    pub fn new() -> Self {
        Self {
            version: PROTOCOL_VERSION,
            command: command::ProtocolCommand::None,
            status: 0,
            id: 0,
            data: None
        }
    }

    pub fn command(command: command::ProtocolCommand) -> Self {
        Self {
            version: PROTOCOL_VERSION,
            command,
            status: 0,
            id: 0,
            data: None
        }
    }

    pub fn add_data(&mut self, data: String) {
        self.data = Some(data);
    }

    pub fn from(buf: &[u8]) -> Option<Self> {
        let mut packet: Self = Self::new();
        packet.version = buf[0];

        if packet.version != PROTOCOL_VERSION {
            return None;
        }

        packet.command = command::ProtocolCommand::from_byte(buf[1]);

        Some(packet)
    }

    pub fn into_bytes(&self) -> Vec<u8> {
        let mut buf: Vec<u8> = Vec::new();

        buf.push(self.version);
        buf.push(command::ProtocolCommand::to_byte(&self.command));
        buf
    }
}
