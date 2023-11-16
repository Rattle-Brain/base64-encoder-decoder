/**
    Base64 encoder/decoder written in rust.
    Functionalities:
        Encode plain text string to Base64 and print it to the stdout
        Decode Base64 string to plain text and print it to the stdout

        Encode plain text file to Base64 and save it to another file
        Decode Base64 file and save it to a plain text file.
*/
use std::string::String;
mod encode_string;
mod decode_string;
mod helper_fns;

fn main() {
    let string: String = "Test to see if this encodes correctly. It does.".to_string();
    let encoded = encode_string::encode(string);
    println!("{}", encoded);
    let decoded = decode_string::decode(encoded.clone());
    println!("{}", decoded);
}
