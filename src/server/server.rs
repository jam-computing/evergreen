use std::{
    borrow::Borrow,
    collections::HashMap,
    io::{BufRead, BufReader, Write},
    net::{TcpListener, TcpStream},
    sync::Mutex,
    usize,
};

use lazy_static::lazy_static;

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

lazy_static! {
    static ref IDS: Mutex<HashMap<u16, ProtocolPacketMetadata>> = Mutex::new(HashMap::new());
}

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
    // TODO:
    // Grab the second u16 - Check if alr exists
    let mut buf_reader = BufReader::new(&mut stream);
    let buf: &[u8];

    match buf_reader.fill_buf() {
        Ok(v) => buf = v,
        Err(e) => {
            warn(format!("Could not establish connection\n{}", e).as_str());
            return;
        }
    }

    let version: u8 = buf[0];

    const PROTOCOL_VERSION: u8 = 2;

    if version != PROTOCOL_VERSION {
        return;
    }

    let id: u16 = u16::from_le_bytes([buf[1].clone(), buf[2].clone()]);

    let hashmap = IDS.lock().unwrap();
    let find = hashmap.get(&id);

    if let Some(meta) = find {
        // META EXISTS

        let data = ProtocolPacketData::from(buf);

        IDS.lock().unwrap().remove(&meta.id);
    }

    // META DOESNT EXIST

    let metadata = ProtocolPacketMetadata::from(buf);

    if let None = metadata {
        warn("Could not marshall bytes into metadata");
        return;
    }

    let meta = metadata.unwrap();

    // add meta key
    IDS.lock().unwrap().insert(meta.id, meta);

    match meta.command {
        _ => {}
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
            println!("{}", v);
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
        let data_res = stream.write_all(data_buf);
    }

    if let Err(_) = meta_res {
        warn("data not sent successfully")
    }
}

fn handle_led_count(stream: &mut TcpStream, _: ProtocolPacketMetadata) {
    let mut packet = ProtocolPacketMetadata::command(ProtocolCommand::LedCount);

    // TODO:
    // Make DB call, Marsall to json

    let data = packet.attach_data("2".into());

    let binding = packet_to_bytes(&packet, Some(&data));
    let meta_buf = &binding.0.as_slice();
    let meta_res = stream.write_all(meta_buf);

    let data_res: Result<usize, std::io::Error> = Ok(0);

    if let Some(v) = &binding.1 {
        let data_buf = &v.as_slice();
        let data_res: Result<usize, std::io::Error> = stream.write(data_buf);
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

    println!("title is: {}", title);
    let ani_req = make_animation_request(title);

    if let Err(err) = ani_req {
        meta_response.status = 500;
        let data_response = meta_response.attach_data("Error parsing json");
        println!("{}", err);
        let binding = packet_to_bytes(&meta_response, Some(&data_response));
        let meta_buf = &binding.0.as_slice();
        let data_buf = &binding.1.unwrap().as_slice();
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
        let data_buf = &binding.1.unwrap().as_slice();
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
        let meta_res = stream.write_all(data_buf);
    }

    if let Err(_) = meta_res {
        warn("data not sent successfully")
    }
}
