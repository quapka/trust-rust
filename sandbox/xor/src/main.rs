use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

use structopt::StructOpt;

#[derive(StructOpt, Debug)]
struct CliOptions {
    #[structopt(short = "c", long = "chunk-size")]
    chunk_size: Option<usize>,
    #[structopt(parse(from_os_str))]
    input: std::path::PathBuf,
    #[structopt(short = "k", long = "key")]
    // FIXME what about non-ascii characters?
    // FIXME what about raw bytes: 0x80 0x10 or 0x801230
    key: String,
}

fn main() -> std::io::Result<()> {
    let args = CliOptions::from_args();

    let chunk_size: usize = match args.chunk_size {
        Some(size) => size,
        None => 1024,
    };

    let file = File::open(args.input)?;

    let mut reader = BufReader::with_capacity(chunk_size, file);

    loop {
        let buffer = reader.fill_buf()?;
        if buffer.len() == 0 {
            break;
        }
        // let xored: [u8; chunk_size] = buffer.into_iter().map(|x| x ^ 13).collect();
        // let xored: Vec<u8> = buffer.into_iter().map(|x| x ^ 13).collect();
        let xored: Vec<u8> = buffer
            .iter()
            .zip(args.key.bytes().cycle().take(chunk_size))
            .map(|(p, k)| p ^ k)
            .collect();

        println!("Read: {:?}", buffer);
        println!("Xored: {:?}", xored);
        let n = buffer.len();
        reader.consume(n);
    }
    Ok(())
}

#[cfg(test)]
mod tests {}
