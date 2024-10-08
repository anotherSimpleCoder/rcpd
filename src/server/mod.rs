mod file_handler;
mod chunks;

use std::net::{TcpListener, TcpStream};
use std::io::{Error, Read};
use miniz_oxide::inflate::decompress_to_vec;

pub fn listen(port: u32) {
    let address = format!("127.0.0.1:{}", port);

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

    let mut compressed_buffer: Vec<u8> = Vec::new();
    stream.read_to_end(&mut compressed_buffer)
        .expect("ServerError: Couldn't read incoming data!");

    /*
    let buffer = decompress_to_vec(compressed_buffer.as_slice())
        .expect("ServerError: Couldn't decompress data!");

     */
    file_handler::write_file(compressed_buffer, "tmp.tmp");
}