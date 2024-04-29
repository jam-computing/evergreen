use std::{io::{BufRead, BufReader, Write}, net::{TcpListener, TcpStream}};

use crate::{config::serverconf::ServerConfig, db::db::make_animation_request, log::logger::{log, warn}, player::animation::Animation, tcp::{command::ProtocolCommand, packet::ProtocolPacket}};

pub fn start(config: ServerConfig) {
    let listener: TcpListener;
    let bind_addr = format!("{}:{}", config.ip, config.port);

    match TcpListener::bind(&bind_addr) {
        Ok(v) => listener = v,
        Err(_) => {
            warn("Could not bind to address. There might be another process using this port?");
            warn("Stopping Server");
            return;
        }
    };

    log(format!("Server started on: {}", bind_addr).as_str());

    for stream in listener.incoming() {
        let stream = stream.unwrap();
        handle_conn(stream);
    }
}

fn handle_conn(mut stream: TcpStream) {
    let mut buf_reader = BufReader::new(&mut stream);
    let buf: &[u8];

    match buf_reader.fill_buf() {
        Ok(v) => buf = v,
        Err(e) => {
            warn(format!("Could not establish connection\n{}", e).as_str());
            return;
        }
    }

    let protocol_packet: Option<ProtocolPacket> = ProtocolPacket::from(buf);

    if let None = protocol_packet {
        warn("A valid packet was not sent. Please ensure that you are using the correct version");
        return;
    }

    let packet = protocol_packet.unwrap();
    log(format!("Received command: {}", packet.command.to_str()).as_str());

    match &packet.command {
        ProtocolCommand::Init => handle_init(&mut stream, packet),
        ProtocolCommand::Play => handle_play(&mut stream, packet),
        ProtocolCommand::LedCount => handle_led_count(&mut stream, packet),
        _ => {},
    }

}

fn handle_led_count(stream: &mut TcpStream, _: ProtocolPacket) {
    let mut packet = ProtocolPacket::command(ProtocolCommand::LedCount);

    // Make DB call, Marsall to json

    let a: Animation = Animation::from(Vec::new());
    let json = serde_json::to_string(&a);

    if let Ok(s) = json {
        packet.add_data(s);
    }

    let binding = packet.into_bytes();
    let buf = &binding.as_slice();
    let result = stream.write_all(buf);

    if let Err(_) = result {
        warn("data not sent successfully")
    }
}

fn handle_init(stream: &mut TcpStream, _: ProtocolPacket) {
    let packet = ProtocolPacket::command(ProtocolCommand::None);

    let binding = packet.into_bytes();
    let buf = &binding.as_slice();

    let result = stream.write_all(buf);

    if let Err(_) = result {
        warn("data not sent successfully")
    }
}


fn handle_play(stream: &mut TcpStream, recv_packet: ProtocolPacket) {
    let packet = ProtocolPacket::command(ProtocolCommand::None);

    let title: String = match recv_packet.data {
        Some(v) => v,
        None => "".into()
    };

    let _ = make_animation_request("One".into());

    println!("Received Play Animation");

    // let url = format!("https://localhost:8090/api/collections/Animations/records?Title={}", title);

    let binding = packet.into_bytes();
    let buf = &binding.as_slice();

    let result = stream.write_all(buf);

    if let Err(_) = result {
        warn("data not sent successfully")
    }
}
