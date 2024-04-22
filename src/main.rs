use crate::config::config::Config;

mod config;
mod player;
mod server;
mod tcp;
mod log;

fn main() {
    let config: Config = Config::get_from_file().expect("Could not read configuration from file");
    server::server::start(&config);
}
