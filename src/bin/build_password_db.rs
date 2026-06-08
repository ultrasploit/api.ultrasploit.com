use std::{
    fs::File,
    io::{BufRead, BufReader, BufWriter, Write},
};

use xxhash_rust::xxh3::xxh3_64;

fn main() -> std::io::Result<()> {
    let input = File::open("/home/ultra/Documents/databases/Pwdb_top-10000000.txt")?;
    let reader = BufReader::new(input);

    let mut hashes = Vec::with_capacity(10_000_000);
    for line in reader.lines() {
        let pw = line?;
        hashes.push(xxh3_64(pw.trim().as_bytes()))
    }

    println!("Loaded {} passwords", hashes.len());

    hashes.sort_unstable();
    hashes.dedup();

    println!("After dedup: {}", hashes.len());

    let output = File::create("passwords.bin")?;
    let mut writer = BufWriter::new(output);

    for hash in hashes {
        writer.write_all(&hash.to_le_bytes())?;
    }

    writer.flush()?;

    println!("Done!");
    Ok(())
}
