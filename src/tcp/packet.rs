use std::str::from_utf8;

use crate::{log::logger::warn, tcp::command};

static PROTOCOL_VERSION: u8 = 2;

// TODO: totes rename this
// TODO: Restructure Protocl Spec
trait Packet {}

#[allow(dead_code)]
pub struct ProtocolPacketMetadata {
    pub version: u8,
    pub id: u16,
    pub command: command::ProtocolCommand,
    pub status: u16,
    pub len: u16,
}

pub struct ProtocolPacketData {
    pub version: u8,
    pub id: u16,
    pub data: String,
}

#[allow(dead_code)]
impl ProtocolPacketMetadata {
    pub fn new() -> Self {
        Self {
            version: PROTOCOL_VERSION,
            id: 0,
            command: command::ProtocolCommand::None,
            status: 0,
            len: 0,
        }
    }

    pub fn command(command: command::ProtocolCommand) -> Self {
        Self {
            version: PROTOCOL_VERSION,
            id: 0,
            command,
            status: 200,
            len: 0,
        }
    }

    pub fn attach_data(&self, data: &str) -> ProtocolPacketData {
        ProtocolPacketData {
            version: PROTOCOL_VERSION,
            id: self.id,
            data: data.to_string(),
        }
    }

    pub fn from(buf: &[u8]) -> Option<Self> {
        let mut packet: Self = Self::new();
        packet.version = buf[0];

        if packet.version != PROTOCOL_VERSION {
            return None;
        }

        packet.id = u16::from_le_bytes([buf[1], buf[2]]);
        packet.command = command::ProtocolCommand::from_byte(buf[3]);
        packet.status = u16::from_le_bytes([buf[4], buf[5]]);
        packet.len = u16::from_le_bytes([buf[6], buf[7]]);

        Some(packet)
    }
}

impl ProtocolPacketData {
    pub fn from(buf: &[u8]) -> Option<Self> {
        let mut packet: Self = Self {
            version: PROTOCOL_VERSION,
            id: 0,
            data: String::new(),
        };

        packet.version = buf[0];

        if packet.version != PROTOCOL_VERSION {
            warn("Incorrect packet version");
            return None;
        }

        packet.id = u16::from_le_bytes([buf[1], buf[2]]);

        let string = from_utf8(&buf[3..]);

        match string {
            Ok(v) => {
                packet.data = v.into();
            }
            Err(_) => {
                warn("Error occured when parsing string data");
                return None;
            }
        }

        Some(packet)
    }
}

impl Packet for ProtocolPacketMetadata {}
impl Packet for ProtocolPacketData {}

pub fn packet_to_bytes(
    packet: &ProtocolPacketMetadata,
    data: Option<&ProtocolPacketData>,
) -> (Vec<u8>, Option<Vec<u8>>) {
    let mut meta_buf: Vec<u8> = Vec::new();
    let mut data_buf: Vec<u8> = Vec::new();

    meta_buf.push(packet.version);
    meta_buf.extend_from_slice(&packet.id.to_le_bytes());
    meta_buf.push(command::ProtocolCommand::to_byte(&packet.command));
    meta_buf.extend_from_slice(&packet.status.to_le_bytes());

    if let Some(d) = data {
        if d.id != packet.id {
            let len: u16 = 0;
            meta_buf.extend_from_slice(&len.to_le_bytes());
            return (meta_buf, None);
        }
        let len: u16 = d.data.len() as u16;
        meta_buf.extend_from_slice(&len.to_le_bytes());

        let id: u16 = packet.id;
        data_buf.push(d.version);
        data_buf.extend_from_slice(&id.to_le_bytes());
        let s = d.data.as_bytes();
        data_buf.extend_from_slice(&s);
    } else {
        let len: u16 = 0;
        meta_buf.extend_from_slice(&len.to_le_bytes());
        return (meta_buf, None);
    }

    (meta_buf, Some(data_buf))
}
