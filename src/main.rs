use argh::FromArgs;
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

#[derive(FromArgs)]
#[argh(description = "A CLI utility to read and write hexadecimal value to a file")]
struct Args {
    #[argh(
        option,
        short = 'w',
        description = "quoted bytes to write in hexadecimal format without 0x (e.g.: \"1f 8b 08\")"
    )]
    write: Option<String>,

    #[argh(
        option,
        short = 'r',
        description = "how many bytes to read after write operation"
    )]
    read: Option<usize>,

    #[argh(
        option,
        short = 'd',
        description = "delay bwtween write and read operation in seconds"
    )]
    delay: Option<u8>,

    #[argh(positional, description = "file to read/write")]
    file: String,
}

fn main() {
    let args: Args = argh::from_env();

    if args.write.is_none() && args.read.is_none() {
        eprintln!("At least one of --write and --read must be present");
        return;
    }

    let mut file = match File::options()
        .create(true)
        .write(true)
        .read(true)
        .open(&args.file)
    {
        Ok(file) => file,
        Err(err) => {
            eprintln!("Failed to open file {}: {err}", &args.file);
            return;
        }
    };

    if let Some(write) = args.write {
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
            eprintln!("Failed to write file {}: {err}", &args.file);
        }
    }

    if let Some(delay) = args.delay {
        sleep(Duration::from_secs(delay as u64));
    }

    if let Some(len) = args.read {
        let mut data = vec![0; len];
        match file.read(&mut data) {
            Ok(read) => {
                if read != len {
                    eprintln!(
                        "Error in reading file {}: expecting {len} bytes, read back {read} bytes",
                        &args.file
                    );
                } else {
                    hexdump(&data, 16);
                }
            }
            Err(err) => {
                eprintln!("Failed to read file {}: {err}", &args.file);
            }
        }
    }
}
