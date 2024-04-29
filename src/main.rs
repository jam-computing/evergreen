use config::serverconf::ServerConfig;

use crate::config::config::Config;

mod config;
mod db;
mod log;
mod player;
mod server;
mod tcp;
mod tree;

fn main() {
    let config: ServerConfig = ServerConfig::read_from_file().expect("Could not read config file.");
    server::server::start(config);
}
