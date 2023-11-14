use std::string::String;

pub fn decode(encoded: String) -> String
{
    let encoded_bytes = encoded.as_bytes();
    let bytes = group_bytes(encoded_bytes);

    let mut result: Vec<u8> = Vec::new();
    print_2d_vector(bytes.clone());

    let mut current: u8;
    let mut reminder: u8;
    let mut count: u8;
    let mut mask: u8 = 0;
    let limit:u8 = 8;

    for byte_group in bytes
    {
        current = 0;
        reminder = 0;
        count = 2;

        for i in 0..byte_group.len()
        {
            if i == byte_group.len() - 1
            {
                current = byte_group[i] << count;
            }
            else
            {
                let byte = byte_group[i];
                let next_byte: u8 = byte_group[i+1];
                current |= byte << count;

                mask = 2_u8.pow(count.into()) - 1;
                mask = mask << (limit - count) - 2;

                reminder = next_byte & mask;                     // Extracts the not-taken bits
                reminder = reminder >> (limit - count) ; // 0b00xxxxxx

                current += reminder;
                count = (count + 2) % limit;                // updates the count

                result.push(current);
                current = current ^ current;
            }
        }
    }

    print_vector(&result);

    return String::new();
}

/**
    Makes groups of 4 bytes of the text to encode and removes padding
    Padding is ascii '=' or 0x3D
 */
fn group_bytes(encoded_bytes: &[u8]) -> Vec<Vec<u8>>
{
    // Matrix of bytes needed for the algorithm
    let mut bytes: Vec<Vec<u8>> = Vec::new();
    let maxi: usize = encoded_bytes.len()/4 + 1;

    // Initialize "bytes" vector as not null
    for _i in 0..maxi{
        bytes.push(Vec::new());
    }

    let mut index = 0;
    for i in (0..encoded_bytes.len()).step_by(4)
    {
        for j in i..i+4
        {
            if encoded_bytes[j] == 0x3D
            {
                continue;
            }
            bytes[index].push(encoded_bytes[j])
        }
        index += 1;
    }
    return bytes;
}


pub fn print_vector(bytes: &[u8])
{
    for i in 0..bytes.len()
    {
        print!("{:#04X?} ", bytes[i]);
    }
    println!();
}

pub fn print_2d_vector(bytes: Vec<Vec<u8>>)
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