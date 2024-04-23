#[allow(dead_code)]
pub enum ProtocolCommand {
    None,
    Init,
    Play,
    Pause,
    Get,
    New,
    On,
    Off,
    OnRange,
    OffRange,
    TreeData
}

#[allow(dead_code)]
impl ProtocolCommand {
    pub fn to_byte(&self) -> u8 {
        match self {
            ProtocolCommand::None => 0,
            ProtocolCommand::Init => 1,
            ProtocolCommand::Play => 2,
            ProtocolCommand::Pause => 3,
            ProtocolCommand::Get => 4,
            ProtocolCommand::New => 5,
            ProtocolCommand::On => 6,
            ProtocolCommand::Off => 7,
            ProtocolCommand::OnRange => 8,
            ProtocolCommand::OffRange => 9,
            ProtocolCommand::TreeData => 10,
        }
    }

    pub fn from_byte(b: u8) -> Self {
        match b {
            1 => ProtocolCommand::Init,
            2 => ProtocolCommand::Play,
            3 => ProtocolCommand::Pause,
            4 => ProtocolCommand::Get,
            5 => ProtocolCommand::New,
            6 => ProtocolCommand::On,
            7 => ProtocolCommand::Off,
            8 => ProtocolCommand::OnRange,
            9 => ProtocolCommand::OffRange,
            10 => ProtocolCommand::TreeData,
            _ => ProtocolCommand::None,
        }
    }
}
