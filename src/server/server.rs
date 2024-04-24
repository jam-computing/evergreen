use std::{io::{BufRead, BufReader, Write}, net::{TcpListener, TcpStream}};

use crate::{config::serverconf::ServerConfig, log::logger::{log, warn}, tcp::{command::ProtocolCommand, packet::ProtocolPacket}};

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

    log("Connection Received!!!!");

    let protocol_packet: Option<ProtocolPacket> = ProtocolPacket::from(buf);

    if let None = protocol_packet {
        warn("A valid packet was not sent. Please ensure that you are using the correct version");
        return;
    }

    let packet = protocol_packet.unwrap();

    log("Packet was received");

    match packet.command {
        ProtocolCommand::Init => handle_init(&mut stream, packet),
        ProtocolCommand::Play => handle_play(&mut stream, packet),
        _ => {},
    }
}

fn handle_init(stream: &mut TcpStream, _: ProtocolPacket) {
    log("received health request");
    let packet = ProtocolPacket::command(ProtocolCommand::None);

    let binding = packet.into_bytes();
    let buf = &binding.as_slice();

    let result = stream.write_all(buf);

    if let Err(_) = result {
        warn("data not sent successfully")
    }
}

fn handle_play(stream: &mut TcpStream, packet: ProtocolPacket) {
    log("received play!");

    if let Some(s) = packet.data {
        log(format!("Received data: {}", s).as_str());
    }

    let packet = ProtocolPacket::command(ProtocolCommand::None);

    let binding = packet.into_bytes();
    let buf = &binding.as_slice();

    let result = stream.write_all(buf);

    if let Err(_) = result {
        warn("data not sent successfully")
    }
}
