use std::{io::{BufRead, BufReader, Write}, net::{TcpListener, TcpStream}};

use crate::{config::config::Config, log::logger::{log, warn}, tcp::packet::{ProtocolCommand, ProtocolSpec}};

pub fn start(config: &Config) {
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

    let protocol_packet: Option<ProtocolSpec> = ProtocolSpec::from(buf);

    if let None = protocol_packet {
        warn("A valid packet was not sent. Please ensure that you are using the correct version");
        return;
    }

    let packet = protocol_packet.unwrap();

    match packet.command {
        ProtocolCommand::None => {},
        ProtocolCommand::Init => handle_init(&mut stream, packet)
    }
}

fn handle_init(stream: &mut TcpStream, _: ProtocolSpec) {

    log("received health request");
    let packet = ProtocolSpec::command(ProtocolCommand::None);

    let binding = packet.into_bytes();
    let buf = &binding.as_slice();

    let result = stream.write_all(buf);

    if let Err(_) = result {
        warn("data not sent successfully")
    }
}
