mod file_handler;
mod chunks;
mod handler;
mod gui;

use std::fmt::format;
use std::net::{TcpListener, TcpStream};
use std::io::{Error, Read};

pub fn listen(port: u32) {
    let address = format!("0.0.0.0:{}", port);

    println!("Listening to {}...", address);
    let listener= TcpListener::bind(address.as_str())
        .expect(format!("ServerError: Couldn't bind to {}", port).as_str());

    for incoming in listener.incoming() {
        receive(incoming);
    }
}

fn receive(incoming: Result<TcpStream, Error>) {
    let mut stream = incoming
        .expect("ServerError: Couldn't accept incoming connection!");

    let mut buffer: Vec<u8> = Vec::new();
    stream.read_to_end(&mut buffer)
        .expect("ServerError: Couldn't read incoming data!");

    let handled = handler::handle_data(&buffer, stream.peer_addr().unwrap().to_string().as_str());

    match handled {
        Some((filename, data)) => {
            file_handler::write_file(data, filename.as_str());
            gui::message("rcpd", "File transfer has been successful!");
        },
        None => return
    }

}