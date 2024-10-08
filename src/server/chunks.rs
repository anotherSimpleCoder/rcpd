pub fn get_chunk(buf: Vec<u8>, tag: &str) {
    let position = get_chunk_pos(&buf, tag);
}

fn get_chunk_pos(buf: &Vec<u8>, tag: &str) -> usize {
    for i in 0..buf.len() {
        let slice = &buf[i..(i+4)];

        if(slice == tag.as_bytes()) {
            return i-4;
        }
    }

    panic!("ChunkError: Chunk {} not found!", tag);
}

fn get_chunk_size(buf: &Vec<u8>, position: usize) -> usize {
    buf[position..position+4]
}