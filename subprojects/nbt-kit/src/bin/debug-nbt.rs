use std::env::args;
use std::fs::read;
use std::io;
use std::io::{stdin, Read};
use nbt_kit::binary::BinaryParser;
use nbt_kit::traits::TagProducer;

fn main() -> io::Result<()> {
    let args: Vec<String> = args().collect();

    if args.len() == 1 {
        eprintln!("Usage: {} <nbt file>", args[0]);
        return Err(io::ErrorKind::InvalidInput.into());
    }

    let bytes = if args[1] == "-" {
        let mut bytes = Vec::new();
        let mut stdin = stdin();

        stdin.read_to_end(&mut bytes)?;
        bytes
    } else {
        read(&args[1])?
    };

    println!("{:?}", bytes);

    let mut parser = BinaryParser::from_ref(&bytes);
    let tag = match parser.take_tag(true) {
        Some(tag) => tag,
        None => {
            eprintln!("Failed to read NBT tag from {}", args[0]);
            return Err(io::ErrorKind::InvalidInput.into());
        },
    };

    println!("{:#?}", tag);
    Ok(())
}
