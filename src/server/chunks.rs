#[derive(Debug)]
pub struct Chunk {
    pub name: String,
    pub position: usize,
    pub chunk_size: usize,
    pub data: Vec<u8>,
}

pub fn get_chunk(buf: &Vec<u8>, tag: &str) -> Chunk {
    let position = get_chunk_pos(&buf, tag);
    let chunk_size = get_chunk_size(&buf[position..(position+4)]);
    let data = buf[(position+8)..(position+8 + chunk_size)].to_vec();

    Chunk {
        name: String::from(tag),
        position,
        chunk_size,
        data
    }
}

fn get_chunk_pos(buf: &Vec<u8>, tag: &str) -> usize {
    for i in 0..buf.len() {
        let slice = &buf[i..(i+4)];

        if slice == tag.as_bytes() {
            return i-4;
        }
    }

    panic!("ChunkError: Chunk {} not found!", tag);
}

fn get_chunk_size(buf_slice: &[u8]) -> usize {
    buf_slice.iter()
        .map(|x| *x as usize)
        .reduce(|mut acc: usize, byte: usize| {
            acc <<= 8;
            acc += byte;
            return acc;
        })
        .expect("ChunkError: Unable to calculate chunk size.")
}