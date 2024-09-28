use std::string::String;
use crate::helper_fns::Group;
// use super::helper_fns as hf;

pub fn decode(encoded: String) -> String
{
    let encoded_bytes = encoded.as_bytes();
    let mut bytes = group_bytes(encoded_bytes);
    for i in 0..bytes.len()
    {
         bytes[i] = translate(&bytes[i]);
    }

    let mut result: Vec<u8> = Vec::new();
//    hf::print_2d_vector(bytes.clone());

    let mut current: u8;
    let mut reminder: u8;
    let mut count: u8;
    let mut mask: u8;
    let limit:u8 = 8;

    for byte_group in bytes
    {
        current = 0;
        count = 2;

        let byte_group_len = byte_group.len(); // Store the length of byte_group

        for i in 0..byte_group_len {
            if i == byte_group_len - 1 {
                // Handle the last byte in the group
                current = byte_group[i] << count;
            } else {
                let byte = byte_group[i]; // Get the current byte
                let next_byte = byte_group[i + 1]; // Get the next byte
        
                current |= byte << count; // Shift the byte left by count and OR with current
        
                // Calculate the mask based on count and shift it
                mask = (2_u8.pow(count.into()) - 1) << (limit - count - 2);
        
                // Extract and shift the not-taken bits using the mask
                reminder = (next_byte & mask) >> (limit - count - 2);
        
                current += reminder; // Add the reminder to current
                count = (count + 2) % limit; // Update the count and wrap around using limit
        
                result.push(current); // Push the current value to the result
                current = 0; // Reset current to 0
            }
        }
    }

//   hf::print_vector(&result);

    return stringify(&result);
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

fn stringify(bytes: &[u8]) -> String
{
    let mut stringified: String = String::new();
    for byte in bytes
    {
        stringified.push(*byte as char)
    }
    return stringified;
}

pub fn translate(bytes: &[u8]) -> Vec<u8>
{
    let mut bytes_return: Vec<u8> = Vec::new();
    for _i in 0..bytes.len()
    {
        bytes_return.push(0);
    }

    for i in 0..bytes.len()
    {
        bytes_return[i] = transform(bytes[i]);
    }
    return bytes_return;
}

fn transform (value: u8) -> u8
{
    return match select_group(value)
    {
        Group::Upper => translate_upper(value),
        Group::Lower => translate_lower(value),
        Group::Num => translate_number(value),
        Group::Plus => translate_plus(),
        Group::Slash => translate_slash(),
    };
}


fn select_group(value: u8) -> Group
{
    return if value >= 0x41 && value <= 0x5A
    {
        Group::Upper
    } else if value >= 0x61 && value <= 0x7A
    {
        Group::Lower
    } else if value >= 0x30 && value <= 0x39
    {
        Group::Num
    } else if value == 0x2B
    {
        Group::Plus
    } else {
        Group::Slash
    }
}

/*------------TRANSLATION FUNCTIONS------------*/
fn translate_upper(value: u8) -> u8
{
    return value - 0x41
}

fn translate_lower (value: u8) -> u8
{
    return value + 26 - 0x61;
}

fn translate_number (value: u8) -> u8
{
    return value + 52  - 0x30;
}

fn translate_plus () -> u8
{
    return 62;        // returns '+'
}

fn translate_slash () -> u8
{
    return 63;       // returns '/'
}