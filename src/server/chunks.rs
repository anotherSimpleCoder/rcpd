#[derive(Debug)]
pub struct Chunk {
    pub name: String,
    pub position: usize,
    pub chunk_size: usize,
    pub data: Vec<u8>,
}

pub fn get_chunk(buf: &Vec<u8>, tag: &str) -> Result<Chunk, String> {
    let position = get_chunk_pos(&buf, tag)?;
    let chunk_size = get_chunk_size(&buf[position..(position+4)])?;
    let data = buf[(position+8)..(position+8 + chunk_size)].to_vec();

    Ok(Chunk {
        name: String::from(tag),
        position,
        chunk_size,
        data
    })
}

fn get_chunk_pos(buf: &Vec<u8>, tag: &str) -> Result<usize, String> {
    for i in 0..buf.len() {
        let slice = &buf[i..(i+4)];

        if slice == tag.as_bytes() {
            return Ok(i-4);
        }
    }

    Err(format!("ChunkError: Chunk {} not found!", tag))
}

fn get_chunk_size(buf_slice: &[u8]) -> Result<usize, String> {
    let slice_size = buf_slice.iter()
        .map(|x| *x as usize)
        .reduce(|mut acc: usize, byte: usize| {
            acc <<= 8;
            acc += byte;
            return acc;
        });

    match slice_size {
        Some(size) => Ok(size),
        None => Err(String::from("Chunk size not found!"))
    }
}