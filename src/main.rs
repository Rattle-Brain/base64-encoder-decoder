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


fn main() {
    let string: String = "String to encode".to_string();
    let encoded = encode_string::encode(string);

    println!("{}", encoded);
}
