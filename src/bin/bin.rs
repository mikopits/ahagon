extern crate ahagon;

use ahagon::{Client, Server};

fn main() {
    let (client, config) = Client::new("cfg.toml").unwrap();
    println!("config: {:?}", config);
    println!("client: {:?}", client);
    Server::run(config);
}
