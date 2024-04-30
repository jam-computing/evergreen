use std::{
    io::{BufRead, BufReader, Write},
    net::{TcpListener, TcpStream},
};

use crate::{
    config::serverconf::ServerConfig,
    db::db::{get_all_animations, make_animation_request},
    log::logger::{log, warn},
    player::playable::Playable,
    tcp::{command::ProtocolCommand, packet::ProtocolPacket},
};

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
        ProtocolCommand::Get => handle_get(&mut stream, packet),
        _ => {}
    }
}

fn handle_get(stream: &mut TcpStream, _: ProtocolPacket) {
    let mut response = ProtocolPacket::command(ProtocolCommand::Get);

    let animations_req = get_all_animations();

    if let Err(err) = animations_req {
        response.status = 500;
        response.data = Some("Error parsing json".into());
        println!("{}", err);
        let binding = response.into_bytes();
        let buf = &binding.as_slice();
        let _ = stream.write_all(buf);
        return;
    }

    if animations_req.as_ref().unwrap().is_empty() {
        response.status = 500;
        response.data = Some("Error Querying DB".into());
        println!("Error Querying DB");
        let binding = response.into_bytes();
        let buf = &binding.as_slice();
        let _ = stream.write_all(buf);
        return;
    }

    let animations = animations_req.unwrap();

    let json = serde_json::to_string(&animations);

    response.data = match json {
        Ok(v) => Some(v),
        Err(_) => None
    };

    let binding = response.into_bytes();
    let buf = &binding.as_slice();
    let result = stream.write_all(buf);

    if let Err(_) = result {
        warn("data not sent successfully")
    }
}

fn handle_led_count(stream: &mut TcpStream, _: ProtocolPacket) {
    let mut packet = ProtocolPacket::command(ProtocolCommand::LedCount);

    // TODO:
    // Make DB call, Marsall to json

    packet.add_data("2".into());

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
    let mut packet = ProtocolPacket::command(ProtocolCommand::None);

    let title: String = match recv_packet.data {
        Some(v) => v,
        None => "".into(),
    };

    println!("title is: {}", title);

    let ani_req = make_animation_request(title);

    if let Err(err) = ani_req {
        packet.status = 500;
        packet.data = Some("Error parsing json".into());
        println!("{}", err);
        let binding = packet.into_bytes();
        let buf = &binding.as_slice();
        let _ = stream.write_all(buf);
        return;
    }

    if let None = ani_req.as_ref().unwrap() {
        packet.status = 500;
        packet.data = Some("Error Querying DB".into());
        println!("Error Querying DB");
        let binding = packet.into_bytes();
        let buf = &binding.as_slice();
        let _ = stream.write_all(buf);
        return;
    }

    let animation = ani_req.unwrap().unwrap();
    animation.play();

    packet.data = Some("Success!".into());
    let binding = packet.into_bytes();
    let buf = &binding.as_slice();
    let result = stream.write_all(buf);

    if let Err(_) = result {
        warn("data not sent successfully")
    }
}
