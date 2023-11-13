use std::string::String;

pub fn encode(text: String) -> String
{
    let text_as_bytes: &[u8] = text.as_bytes();
    let bytes: Vec<Vec<u8>> = group_bytes(&text_as_bytes);
    print_2d_vector(bytes.clone());

    let mut result: Vec<u8> = Vec::new();
    let limit = 8;
    let mut current: u8;
    let mut count: u8;
    let mut reminder: u8;
    let mut mask: u8;

    for byte_group in bytes
    {
        reminder = 0;
        count = 2;
        current = 0;
        for i in 0.. byte_group.len() + 1
        {
            if i >= byte_group.len()
            {
                mask = 0x3F;
                current = byte_group[i-1] & mask;
                result.push(current);
                break;
            }

            let byte = byte_group[i];
            current |= byte >> count;
            current += reminder;
            mask = 2_u8.pow(count.into()) - 1;
            reminder = byte & mask;
            reminder = reminder << (limit - count);
            count = (count + 2) % limit;
            result.push(current);
            current = current ^ current;
        }
    }
    print_vector(&result);
    println!("{}", result.len());
    return stringify(&result)
}

fn print_vector(bytes: &[u8])
{
    for i in 0..bytes.len()
    {
        print!("{:#04X?} ", bytes[i]);
    }
    println!();
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