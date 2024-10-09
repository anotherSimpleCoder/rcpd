use crate::server::chunks;
use crate::server::gui;
use miniz_oxide::inflate::decompress_to_vec;
use crate::server::chunks::Chunk;

pub fn handle_data(buffer: &Vec<u8>, sender: &str) -> Result<(String, Vec<u8>), String> {
    let header_chunk = chunks::get_chunk(buffer, "RHDR")?;
    let filename = handle_header(header_chunk)?;

    let response = gui::dialog("Incoming file transfer", format!("Do you accept the file {} from {}?", filename, sender).as_str());
    if !response {
        return Err(String::from("File transfer declined."));
    }

    let data_chunk = chunks::get_chunk(buffer, "RDAT")?;
    let decompressed_data = decompress_to_vec(data_chunk.data.as_slice())
        .expect("DataError: Failed decompressing data");

    Ok((filename, decompressed_data))
}

fn handle_header(buffer: Chunk) -> Result<String, String> {
    assert_eq!(buffer.name, "RHDR");

    let filename_chunk = chunks::get_chunk(&buffer.data, "FNME")?;
    match String::from_utf8(filename_chunk.data) {
        Ok(filename) => Ok(filename),
        Err(_) => Err(String::from("FilenameError: Failed decoding filename!"))
    }
}

