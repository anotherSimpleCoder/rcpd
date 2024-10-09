use std::fs::File;
use std::io::Write;

pub fn write_file(buf: Vec<u8>, name: &str) -> Result<(), String> {
    let file = File::create(name);

    match file {
        Ok(mut file) => {
            let write_result = file.write_all(buf.as_slice());

            if write_result.is_err() {
                return Err(String::from("FileError: Error writing file!"));
            }

            drop(file);
            Ok(())
        },
        Err(e) => Err(format!("FileError: Could not create file: {}", e))
    }
}