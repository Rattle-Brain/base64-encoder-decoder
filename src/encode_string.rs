use std::string::String;
//use super::helper_fns as hf;
use super::helper_fns::Group;

pub fn encode(text: String) -> String
{
    let text_as_bytes: &[u8] = text.as_bytes();
    let bytes: Vec<Vec<u8>> = group_bytes(&text_as_bytes);
//    hf::print_2d_vector(bytes.clone());

    let mut result: Vec<u8> = Vec::new();
    let limit = 8;
    let mut current: u8;        // Stores the 6 current bits to be pushed to "result"
    let mut count: u8;          // Counts the number of positions to move the bits
    let mut reminder: u8;       // stores the bits not taken from the byte
    let mut mask: u8;           // Mask to extract the bits not taken during previous operations

    for byte_group in bytes {
        // Initialize variables for each byte group
        let byte_group_len = byte_group.len(); // Store the length of byte_group
        reminder = 0;
        count = 2;
        current = 0;
    
        // Iterate over each byte in the byte group plus one extra iteration
        for i in 0..=byte_group_len {
            // Handle the case when i exceeds the length of byte_group
            if i >= byte_group_len {
                let mask = 0x3F; // Take the 6 less significant bits (0b00111111)
                current = byte_group[i - 1] & mask;
                result.push(current); // Push the current value to the result
                break; // Exit the loop
            }
    
            let byte = byte_group[i]; // Get the current byte
            current |= byte >> count; // Shift the byte right by count and OR with current
            current += reminder; // Add the reminder to current
    
            mask = (1 << count) - 1; // Calculate the mask based on count
    
            reminder = byte & mask; // Extract the not-taken bits using the mask
            reminder <<= (limit - count) - 2; // Shift the reminder bits
    
            count = (count + 2) % limit; // Update the count and wrap around using limit
    
            result.push(current); // Push the current value to the result
            current = 0; // Reset current to 0
        }
    }
//    hf::print_vector(&text_as_bytes);
    return stringify(&result)
}

/**
    Makes groups of 3 bytes of the text to encode.
    Padding is ascii '=' or 0x3D
 */
fn group_bytes(text_bytes: &[u8]) -> Vec<Vec<u8>>
{
    // Matrix of bytes needed for the algorithm
    let mut bytes: Vec<Vec<u8>> = Vec::new();
    let maxi: usize;
    if text_bytes.len() % 3 == 0
    {
        maxi = text_bytes.len()/3;
    }
    else
    {
        maxi = text_bytes.len() / 3 + 1;
    }

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
                bytes[index].push(0x00);    // Padding is 0x00 (later transformed)
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

/**
    Transforms the value of the byte to each corresponding
    value in the Base64 translation table.

    Returns the resulting string.
 */
pub fn stringify(bytes: &[u8]) -> String
{
    let mut stringified = String::new();
    for i in 0..bytes.len()
    {
        stringified += &*transform(bytes[i]).to_string();
    }
    return stringified;
}

/**
    Transforms a signle byte to its corresponding value
    using the Base64 translation table
*/
fn transform (value: u8) -> char
{
    if value == 0x00
    {
        return 0x3D as char;
    }
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
    return if value <= 25
    {
        Group::Upper
    } else if value >= 26 && value <= 51
    {
        Group::Lower
    } else if value >= 51 && value <= 61
    {
        Group::Num
    } else if value == 62
    {
        Group::Plus
    } else {
        Group::Slash
    }
}

/*------------TRANSLATION FUNCTIONS------------*/
fn translate_upper(value: u8) -> char
{
    return (value + 0x41) as char
}

fn translate_lower (value: u8) -> char
{
    return (value -26 + 0x61) as char
}

fn translate_number (value: u8) -> char
{
    return (value - 52  + 0x30) as char
}

fn translate_plus () -> char
{
    return 0x2B as char;        // returns '+'
}

fn translate_slash () -> char
{
    return 0x2F as char;        // returns '/'
}