use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

use structopt::StructOpt;

#[derive(StructOpt, Debug)]
struct CliOptions {
    #[structopt(short = "c", long = "chunk-size")]
    chunk_size: Option<usize>,
    #[structopt(parse(from_os_str))]
    path: std::path::PathBuf,
    // pattern: String,
    // #[structopt(parse(from_os_str))]
    // path: std::path::PathBuf,
    // play with optional value
    // value: Option<String>,
}

fn main() -> std::io::Result<()> {
    let args = CliOptions::from_args();

    // const BUFFER_SIZE: usize = 10;
    let BUFFER_SIZE: usize = match args.chunk_size {
        Some(size) => size,
        None => 1024,
    };

    let file = File::open(args.path)?;

    let mut reader = BufReader::with_capacity(BUFFER_SIZE, file);

    loop {
        let mut buffer = reader.fill_buf()?;
        if buffer.len() == 0 {
            break;
        }
        // let xored: [u8; BUFFER_SIZE] = buffer.into_iter().map(|x| x ^ 13).collect();
        let xored: Vec<u8> = buffer.into_iter().map(|x| x ^ 13).collect();

        println!("Read: {:?}", buffer);
        println!("Xored: {:?}", xored);
        let n = buffer.len();
        reader.consume(n);
    }
    Ok(())
}

#[cfg(test)]
mod tests {}
