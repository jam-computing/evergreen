use std::{
    io::{BufRead, BufReader, Write},
    net::{TcpListener, TcpStream},
};

use crate::{
    config::serverconf::ServerConfig,
    db::db::{get_all_animations, make_animation_request},
    log::logger::{log, warn},
    player::playable::Playable,
    tcp::{
        command::ProtocolCommand,
        packet::{packet_to_bytes, ProtocolPacketData, ProtocolPacketMetadata},
    },
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
    const PROTOCOL_VERSION: u8 = 2;

    let meta: ProtocolPacketMetadata;

    {
        let buf = buf_reader.fill_buf().unwrap();
        let version: u8 = buf[0];
        if version != PROTOCOL_VERSION {
            println!("Imcompatible version number");
            return;
        }
        let metadata = ProtocolPacketMetadata::from(buf);
        if let None = metadata {
            warn("Could not marshall bytes into metadata");
            return;
        }
        meta = metadata.unwrap();
    }

    buf_reader.consume(8);

    match meta.command {
        ProtocolCommand::Init => handle_init(&mut stream, meta),
        ProtocolCommand::Get => handle_get(&mut stream, meta),
        ProtocolCommand::LedCount => handle_led_count(&mut stream, meta),
        _ => {
            let second_buf: &[u8];
            match buf_reader.fill_buf() {
                Ok(v) => second_buf = v,
                Err(e) => {
                    warn(format!("Could not establish connection\n{}", e).as_str());
                    return;
                }
            }

            let binding = ProtocolPacketData::from(second_buf);

            if let None = binding {
                warn("Could not marshall bytes into metadata");
                return;
            }
            let data = binding.unwrap();

            match meta.command {
                ProtocolCommand::Play => handle_play(&mut stream, data),
                _ => {}
            }
        }
    }
}

fn handle_get(stream: &mut TcpStream, _: ProtocolPacketMetadata) {
    let mut meta_response = ProtocolPacketMetadata::command(ProtocolCommand::Get);

    let animations_req = get_all_animations();

    if let Err(err) = animations_req {
        meta_response.status = 500;
        let data_response = meta_response.attach_data("Error Parsing Json");
        println!("{}", err);
        let binding = packet_to_bytes(&meta_response, Some(&data_response));
        let meta_buf = &binding.0.as_slice();
        let data_binding = binding.1.unwrap();
        let data_buf = data_binding.as_slice();
        let _ = stream.write_all(meta_buf);
        let _ = stream.write_all(data_buf);
        return;
    }

    if animations_req.as_ref().unwrap().is_empty() {
        meta_response.status = 500;
        let data_response = meta_response.attach_data("Error querying DB");
        println!("Error Querying DB");
        let binding = packet_to_bytes(&meta_response, Some(&data_response));
        let meta_buf = &binding.0.as_slice();
        let data_binding = binding.1.unwrap();
        let data_buf = data_binding.as_slice();
        let _ = stream.write_all(meta_buf);
        let _ = stream.write_all(data_buf);
        return;
    }

    let animations = animations_req.unwrap();
    let json = serde_json::to_string(&animations);

    let data = match json {
        Ok(v) => {
            // println!("{}", v);
            v
        }
        Err(_) => String::new(),
    };

    let data_response = meta_response.attach_data(&data);

    let binding = packet_to_bytes(&meta_response, Some(&data_response));
    let meta_buf = &binding.0.as_slice();
    let meta_res = stream.write_all(meta_buf);

    if let Some(v) = &binding.1 {
        let data_buf = v.as_slice();
        let _ = stream.write_all(data_buf);
    }

    if let Err(_) = meta_res {
        warn("data not sent successfully")
    }
}

fn handle_led_count(stream: &mut TcpStream, _: ProtocolPacketMetadata) {
    let packet = ProtocolPacketMetadata::command(ProtocolCommand::LedCount);

    // TODO:
    // Make DB call, Marsall to json

    let data = packet.attach_data("2".into());

    let binding = packet_to_bytes(&packet, Some(&data));
    let meta_buf = &binding.0.as_slice();
    let meta_res = stream.write_all(meta_buf);

    let data_res: Result<usize, std::io::Error> = Ok(0);

    if let Some(v) = &binding.1 {
        let data_buf = &v.as_slice();
        let _: Result<usize, std::io::Error> = stream.write(data_buf);
    }

    if let Err(_) = meta_res {
        warn("metadata not sent successfully")
    }
    if let Err(_) = data_res {
        warn("data not sent successfully")
    }
}

fn handle_init(stream: &mut TcpStream, _: ProtocolPacketMetadata) {
    let packet = ProtocolPacketMetadata::command(ProtocolCommand::None);

    let binding = packet_to_bytes(&packet, None).0;
    let buf = &binding.as_slice();

    let result = stream.write_all(buf);

    if let Err(_) = result {
        warn("data not sent successfully")
    }
}

fn handle_play(stream: &mut TcpStream, recv_packet: ProtocolPacketData) {
    let mut meta_response = ProtocolPacketMetadata::command(ProtocolCommand::None);

    let title: String = recv_packet.data;

    let ani_req = make_animation_request(title);

    if let Err(err) = ani_req {
        meta_response.status = 500;
        let data_response = meta_response.attach_data("Error parsing json");
        println!("{}", err);
        let binding = packet_to_bytes(&meta_response, Some(&data_response));
        let meta_buf = &binding.0.as_slice();
        let binding_one = &binding.1.clone().unwrap();
        let data_buf = binding_one.as_slice();
        let _ = stream.write_all(meta_buf);
        let _ = stream.write_all(data_buf);
        return;
    }

    if let None = ani_req.as_ref().unwrap() {
        meta_response.status = 500;
        let data_response = meta_response.attach_data("Error querying DB");
        println!("Error Querying DB");
        let binding = packet_to_bytes(&meta_response, Some(&data_response));
        let meta_buf = &binding.0.as_slice();
        let binding_one = &binding.1.clone().unwrap();
        let data_buf = binding_one.as_slice();
        let _ = stream.write_all(meta_buf);
        let _ = stream.write_all(data_buf);
        return;
    }

    let animation = ani_req.unwrap().unwrap();
    animation.play();

    let data_response = meta_response.attach_data("Success!");
    let binding = packet_to_bytes(&meta_response, Some(&data_response));
    let meta_buf = &binding.0.as_slice();
    let meta_res = stream.write_all(meta_buf);

    if let Some(v) = &binding.1 {
        let data_buf = v.as_slice();
        let _ = stream.write_all(data_buf);
    }

    if let Err(_) = meta_res {
        warn("data not sent successfully")
    }
}
