use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

fn main() -> std::io::Result<()> {
    const BUFFER_SIZE: usize = 10;

    let file = File::open("Cargo.toml")?;

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
