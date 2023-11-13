use std::string::String;

pub fn encode(text: String) -> String
{
    let text_as_bytes: &[u8] = text.as_bytes();
    let string_return: String = String::new();
    let bytes: Vec<Vec<u8>> = group_bytes(&text_as_bytes);
    print_2d_vector(bytes.clone());

    let mut result: Vec<u8> = Vec::new();
    let limit = 8;
    let mut current: u8 = 0;
    let mut count: u8 = 2;
    let mut reminder: u8 = 0;
    let mut mask: u8;

    for byte_group in bytes
    {
        for byte in byte_group
        {
            current |= (byte + reminder) >> count;
            mask = 2_u8.pow(count.into()) - 1;
            reminder = byte & mask;
            reminder = reminder << (limit - count);
            count = (count + 2) % limit;
            result.push(current);
            current = current ^ current;
        }
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

fn print_2d_vector(bytes: Vec<Vec<u8>>)
{
    for i in 0..bytes.len()
    {
        for j in 0..bytes[i].len()
        {
            print!("{:#04X?} ", bytes[i][j]);
        }
        println!();
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

/**
    Makes groups of 3 bytes of the text to encode.
    Padding is ascii '=' or 0x3D
 */
fn group_bytes(text_bytes: &[u8]) -> Vec<Vec<u8>>
{
    // Matrix of bytes needed for the algorithm
    let mut bytes: Vec<Vec<u8>> = Vec::new();
    let maxi: usize = text_bytes.len()/3 + 1;

    // Initialize "bytes" vector as not null
    for _i in 0..maxi{
        bytes.push(Vec::new());
    }

    let mut index = 0;
    for i in (0..text_bytes.len()).step_by(3)
    {
        for j in i..i+3
        {
            if j >= text_bytes.len()
            {
                bytes[index].push(0x3D);
            }
            else
            {
                bytes[index].push(text_bytes[j])
            }
        }
        index += 1;

    }
    return bytes;
}