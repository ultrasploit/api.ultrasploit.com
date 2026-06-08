use std::{
    fs::File,
    io::{BufReader, Read},
};

use xxhash_rust::xxh3::xxh3_64;

pub fn load_db(path: &str) -> std::io::Result<Vec<u64>> {
    let file = File::open(path)?;
    let mut reader = BufReader::new(file);

    let mut bytes = Vec::new();
    reader.read_to_end(&mut bytes)?;

    Ok(bytes
        .chunks_exact(8)
        .map(|chunk| {
            let mut arr = [0u8; 8];
            arr.copy_from_slice(chunk);
            u64::from_le_bytes(arr)
        })
        .collect())
}

pub fn is_common(password: &str, passwords: &[u64]) -> bool {
    let hash = xxh3_64(password.as_bytes());
    passwords.binary_search(&hash).is_ok()
}
