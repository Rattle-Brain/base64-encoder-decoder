use std::string::String;

pub fn encode(text: String) -> String
{
    let bytes: &[u8] = text.as_bytes();
    let string_return: String = String::new();

    let mut result: Vec<u8> = Vec::new();
    let limit = 8;

    let mut current: u8 = 0;
    let mut count: u8 = 2;
    let mut reminder: u8 = 0;
    let mut mask: u8;

    for byte in bytes
    {
        current |= (byte + reminder) >> count;
        mask = 2_u8.pow(count.into()) - 1;
        reminder = byte & mask;
        reminder = reminder << limit - count;
        count = (count + 2) % limit;
        result.push(current);
        current = current ^ current;
    }
    print_vector(&result);
    return string_return;
}

fn print_vector(bytes: &[u8])
{
    for i in 0..bytes.len()
    {
        print!("{:#04X?} ", bytes[i]);
    }
}

fn stringify(bytes: &[u8]) -> String
{
    let mut stringified = String::new();
    for i in 0..bytes.len()
    {
        stringified += &bytes[i].to_string();
    }
    return stringified;
}