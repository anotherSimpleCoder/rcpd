mod file_handler;
mod chunks;
mod handler;
mod gui;
mod config;

use std::net::{TcpListener, TcpStream};
use std::io::{Error, Read};
use std::path::Path;
use crate::server::config::Config;

pub fn listen() {
    let config = config::load_config();
    let address = format!("0.0.0.0:{}", config.port);

    println!("Listening to {}...", address);
    let listener= TcpListener::bind(address.as_str())
        .expect(format!("ServerError: Couldn't bind to {}", config.port).as_str());

    for incoming in listener.incoming() {
        receive(incoming, &config);
    }
}

fn receive(incoming: Result<TcpStream, Error>, config: &Config) {
    let mut stream = incoming
        .expect("ServerError: Couldn't accept incoming connection!");

    let mut buffer: Vec<u8> = Vec::new();
    let _ = stream.read_to_end(&mut buffer);

    let handled = handler::handle_data(&buffer, stream.peer_addr().unwrap().to_string().as_str());

    match handled {
        Ok((filename, data)) => {
            let path = Path::new(config.out_path.as_str());
            let full_path = path.join(filename)
                .display()
                .to_string();

            let write_result = file_handler::write_file(data, full_path.as_str());
            if write_result.is_err() {
                gui::error("rcpd", write_result.unwrap_err().as_str());
                return;
            }

            gui::message("rcpd", "File transfer has been successful!");
        },
        Err(_) => {
            gui::error("rcpd", "File transfer failed!");
        }
    }

}