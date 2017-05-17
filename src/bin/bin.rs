extern crate ahagon;
extern crate env_logger;
#[macro_use]
extern crate log;

use ahagon::{Client, Server};

fn main() {
    env_logger::init().unwrap();

    let (client, config) = Client::new("cfg.toml").unwrap();
    debug!("config: {:?}", config);
    debug!("client: {:?}", client);

    info!("Starting server at {:?}...", config.url());
    Server::run(config);
}
