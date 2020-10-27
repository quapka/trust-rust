// use std::fs::File;
// use std::io::{prelude::*, BufReader};
// use anyhow::{Context, Result};
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
struct CliOptions {
    pattern: String,
    #[structopt(parse(from_os_str))]
    path: std::path::PathBuf,
    // play with optional value
    value: Option<String>,
}

// fn main() -> std::io::Result<()> {
fn main() {
    let args = CliOptions::from_args();

    println!(
        "pattern: {pattern} path: {path} value: {value}",
        pattern = args.pattern,
        // path = args.path.to_str(),
        path = match args.path.to_str() {
            Some(path) => path,
            None => "Invalid path",
        },
        value = match args.value {
            Some(value) => value,
            None => String::from("No value set"),
        }
    );

    // let file = File::open(args.path)?;
    // let mut reader = BufReader::new(file);
    // let mut line = String::new();

    // while let read = reader.read_line(&mut line) {
    //     println!("{}", line);
    // }

    let content = std::fs::read_to_string(args.path).unwrap();
    // {
    //     Ok(content) => content,
    //     Err(e) => panic!("Hey you, error: {}", e),
    // };

    for line in content.lines() {
        if line.contains(&args.pattern) {
            println!("{}", line);
        }
    }

    // Ok(())
}
