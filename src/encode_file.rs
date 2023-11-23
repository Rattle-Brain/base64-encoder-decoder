use super::encode_string as es;
use std::string::String;
use std::fs::File;
use std::io::{Read, Write};

pub fn encode_file(filename: String, new_filename: String) -> std::io::Result<()>
{
    let mut f = File::open(filename.clone())?;

    let mut buff: Vec<u8> = Vec::new();
    let _ = f.read_to_end(&mut buff).unwrap();

    let plain_text: String = String::from_utf8(buff).unwrap();
    let encoded = es::encode(plain_text.clone());

    let encoded_filename;
    if new_filename.len() == 0
    {
        let splitted: Vec<&str> = filename.split(".txt").collect();
        encoded_filename = splitted[0].to_string() + "-encoded.txt";
    }
    else
    {
        encoded_filename = new_filename.clone();
    }

    let new_file = File::create(encoded_filename);
    let _var = new_file.unwrap().write_all(encoded.as_bytes()).expect("Something went wrong writing the file");
    Ok(())
}