use std::fs::OpenOptions;
use std::io::prelude::*;
use std::io::ErrorKind;
use std::path::Path;

// No shadowing:
// let x1 = 1;
// let x2 = 2;
// let x3 = 3;
//
// Against shadowing:
// let x = 1;
// let x = 2;
// let x = 3;
//

fn main() -> std::io::Result<()> {
    let shadow_path = Path::new("shadow_test");

    // stop on any error that is not AlreadyExists kind
    match std::fs::create_dir(shadow_path) {
        Err(e) => match e.kind() {
            ErrorKind::AlreadyExists => (),
            _ => return Err(e),
        },
        Ok(res) => res,
    }

    let filename = Path::new("shadow_test/shadow.rs");
    let mut output = OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open(filename)?;

    // add header
    output.write_all(b"#[allow(unused_variables)]\nfn main() {")?;
    // add variables definition
    for i in 0..10_000 {
        output.write_all(format!("\tlet x = {};\n", i).as_bytes())?;
    }
    // add footer
    output.write_all(b"\n}")?;

    Ok(())
}
