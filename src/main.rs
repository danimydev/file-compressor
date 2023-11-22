extern crate flate2;

use flate2::write::GzEncoder;
use flate2::Compression;

use std::env::args;
use std::fs::File;
use std::io;

fn main() -> Result<(), io::Error>{
    let arguments: Vec<String> = args().collect();
    let parsed_args = parse_args(&arguments);

    if arguments.len() != 3 {
        eprintln!("Usage: `source` `target`");
        panic!("must pass source and target!");
    }

    return run(parsed_args.file_path, parsed_args.output_file_name);
}

struct ParsedArgs {
    file_path: String,
    output_file_name: String,
}

fn parse_args(args: &[String]) -> ParsedArgs {
    let file_path = args[1].clone();
    let output_file_name = args[2].clone();

    ParsedArgs {
        file_path,
        output_file_name,
    }
}

fn run(file_path: String, output_file_name: String) -> Result<(), io::Error> {
    let file = File::open(file_path)?;
    let mut reader = io::BufReader::new(file);

    let output = File::create(output_file_name).unwrap();
    let mut encoder = GzEncoder::new(output, Compression::default());
    io::copy(&mut reader, &mut encoder).unwrap();
    let output = encoder.finish().unwrap();

    println!(
        "Source len: {:?}",
        reader.get_ref().metadata().unwrap().len()
    );
    println!("Target len: {:?}", output.metadata().unwrap().len());

    Ok(())
}
