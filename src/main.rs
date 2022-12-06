use clap::{Arg, Command};
use std::{fs::File, io::Write, process::exit};

fn main() {
    let matches = Command::new(env!("CARGO_BIN_NAME"))
        .about("A CLI utility to read and write hexadecimal value to a file")
        .arg_required_else_help(true)
        .arg(
            Arg::new("bytes")
                .help("Quoted bytes in hexadecimal format without 0x (e.g.: \"1f 8b 08\")")
                .required(true),
        )
        .arg(Arg::new("file").help("file to read/write").required(true))
        .get_matches();

    let bytes = matches
        .get_one::<String>("bytes")
        .unwrap()
        .trim()
        .to_lowercase();

    let mut data = Vec::new();
    bytes.split_whitespace().for_each(|byte| {
        if let Ok(byte) = u8::from_str_radix(byte, 16) {
            data.extend_from_slice(&[byte]);
        } else {
            eprintln!("{byte} isn't a hexadecimal byte.");
            exit(-1);
        }
    });

    let path = matches.get_one::<String>("file").unwrap();
    let mut file = match File::options().create(true).write(true).open(path) {
        Ok(file) => file,
        Err(err) => {
            eprintln!("Failed to open file {path}: {err}");
            return;
        }
    };

    if let Err(err) = file.write_all(&data) {
        eprintln!("Failed to write file {path}: {err}");
    }
}
