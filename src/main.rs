use clap::{value_parser, Arg, Command};
use std::{
    fs::File,
    io::{Read, Write},
    process::exit,
    thread::sleep,
    time::Duration,
};

fn hexdump(data: &[u8], line_width: usize) {
    for line_offset in (0..data.len()).step_by(line_width) {
        // header
        print!("{line_offset:08x}");

        // hexadecimal bytes
        for i in (0..line_width).step_by(1) {
            if line_width != 1 && i % (line_width / 2) == 0 {
                print!(" ");
            }
            if line_offset + i < data.len() {
                print!(" {:02x}", data[line_offset + i]);
            } else {
                // print spaces as place holder
                print!("   ");
            }
        }

        // chracters
        print!("  |");
        for i in (0..line_width).step_by(1) {
            if line_offset + i < data.len() {
                let byte = data[line_offset + i];
                if byte.is_ascii() && !byte.is_ascii_control() {
                    print!("{}", byte as char);
                } else {
                    print!(".");
                }
            } else {
                print!(" ");
            }
        }
        println!("|");
    }
}

fn main() {
    let matches = Command::new(env!("CARGO_BIN_NAME"))
        .about("A CLI utility to read and write hexadecimal value to a file")
        .arg_required_else_help(true)
        .arg(
            Arg::new("write")
                .short('w')
                .long("write")
                .help("Quoted bytes to write in hexadecimal format without 0x (e.g.: \"1f 8b 08\")")
                .required_unless_present("read"),
        )
        .arg(
            Arg::new("delay")
                .short('d')
                .long("delay")
                .value_parser(value_parser!(u8))
                .help("Delay bwtween write and read operation in seconds")
                .default_value("0"),
        )
        .arg(
            Arg::new("read")
                .short('r')
                .long("read")
                .value_parser(value_parser!(usize))
                .help("How many bytes to read after write operation")
                .required_unless_present("write"),
        )
        .arg(Arg::new("file").help("file to read/write").required(true))
        .get_matches();

    let path = matches.get_one::<String>("file").unwrap();
    let mut file = match File::options()
        .create(true)
        .write(true)
        .read(true)
        .open(path)
    {
        Ok(file) => file,
        Err(err) => {
            eprintln!("Failed to open file {path}: {err}");
            return;
        }
    };

    if let Some(write) = matches.get_one::<String>("write") {
        let bytes = write.trim().to_lowercase();

        let mut data = Vec::new();
        bytes.split_whitespace().for_each(|byte| {
            if let Ok(byte) = u8::from_str_radix(byte, 16) {
                data.extend_from_slice(&[byte]);
            } else {
                eprintln!("{byte} isn't a hexadecimal byte.");
                exit(-1);
            }
        });

        if let Err(err) = file.write_all(&data) {
            eprintln!("Failed to write file {path}: {err}");
        }
    }

    let delay = matches.get_one::<u8>("delay").unwrap();
    sleep(Duration::from_secs(*delay as u64));

    if let Some(len) = matches.get_one::<usize>("read") {
        let len = *len;
        let mut data = vec![0; len];
        match file.read(&mut data) {
            Ok(read) => {
                if read != len {
                    eprintln!(
                        "Error in reading file {path}: expecting {len} bytes, read back {read} bytes"
                    );
                } else {
                    hexdump(&data, 16);
                }
            }
            Err(err) => {
                eprintln!("Failed to read file {path}: {err}");
            }
        }
    }
}
