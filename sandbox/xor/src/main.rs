use std::fs::File;
use std::io::prelude::*;
use std::io::{BufReader, BufWriter};

use structopt::StructOpt;

#[derive(StructOpt, Debug)]
struct CliOptions {
    #[structopt(short = "c", long = "chunk-size")]
    chunk_size: Option<usize>,
    #[structopt(parse(from_os_str))]
    input: std::path::PathBuf,
    #[structopt(parse(from_os_str), short = "o", long = "output")]
    output: Option<std::path::PathBuf>,
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

    let file = File::open(&args.input)?;

    let mut reader = BufReader::with_capacity(chunk_size, file);
    let out_file: std::path::PathBuf = match args.output {
        Some(path) => path,
        None => {
            // TODO is there a better way to join path and string with 'dash'
            std::path::PathBuf::from([args.input.to_str().unwrap(), args.key.as_str()].join("-"))
        }
    };
    let mut writer = BufWriter::new(File::create(out_file)?);

    let mut keystream = args.key.bytes().cycle();

    loop {
        let buffer = reader.fill_buf()?;
        let buf_size = buffer.len();
        if buf_size == 0 {
            break;
        }
        let chunk_key = keystream.by_ref().take(buf_size);
        let xored: Vec<u8> = buffer.iter().zip(chunk_key).map(|(p, k)| p ^ k).collect();

        writer.write_all(&xored)?;
        writer.flush()?;

        reader.consume(buf_size);
    }

    Ok(())
}

#[cfg(test)]
mod tests {}
