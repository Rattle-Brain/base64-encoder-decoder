/**
    Base64 encoder/decoder written in rust.
    Functionalities:
        Encode plain text string to Base64 and print it to the stdout
        Decode Base64 string to plain text and print it to the stdout

        Encode plain text file to Base64 and save it to another file
        Decode Base64 file and save it to a plain text file.
*/
use std::string::String;
use clap::{Command, Arg};
mod encode_string;
mod decode_string;
mod helper_fns;
mod encode_file;

fn main() {
    let flags = Command::new("Base64 Coder")
        .version("1.0")
        .author("Rattle Brain")
        .about("Translates to and from Base64")
        .arg(
            Arg::new("encode")
                .short('e')
                .long("encode")
                .value_name("STRING")
                .help("Encodes plain text string to Base 64")
        )
        .arg(
            Arg::new("decode")
                .short('d')
                .long("decode")
                .value_name("BASE64")
                .help("Decodes Base64 string to plain text")
        )
        .arg(
            Arg::new("encode-file")
                .short('E')
                .long("encode-file")
                .value_name("FILE.TXT")
                .help("Encodes a plain text file to Base64")
        )
        .get_matches();

    // Selects the code flow from flag received.
    if let Some(e) = flags.get_one::<String>("encode")
    {
        let encoded = encode_string::encode(e.to_string());
        println!("Result in Base64: {}", encoded);

    }
    else if let Some(d) = flags.get_one::<String>("decode")
    {
        let decoded = decode_string::decode(d.to_string());
        println!("Result in plain text: {}", decoded);

    }
    else if let Some(ef) = flags.get_one::<String>("encode-file")
    {
        let _ = encode_file::encode_file(ef.to_string(), "".to_string());
        println!("File encoded properly.");

    }
}
