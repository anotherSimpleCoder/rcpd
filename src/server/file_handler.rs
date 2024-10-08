use std::fs::File;
use std::io::Write;

pub fn write_file(buf: Vec<u8>, name: &str) {
    let mut file = File::create(name)
        .expect("Could not create audio file");

    file.write_all(buf.as_slice())
        .expect("Could not write audio file");

    drop(file);
}