use std::string::String;

pub fn encode(text: String) -> String
{
    let text_as_bytes: &[u8] = text.as_bytes();
    let bytes: Vec<Vec<u8>> = group_bytes(&text_as_bytes);

    let mut result: Vec<u8> = Vec::new();
    let limit = 8;
    let mut current: u8;        // Stores the 6 current bits to be pushed to "result"
    let mut count: u8;          // Counts the number of positions to move the bits
    let mut reminder: u8;       // stores the bits not taken from the byte
    let mut mask: u8;           // Mask to extract the bits not taken during previous operations

    for byte_group in bytes
    {
        reminder = 0;
        count = 2;
        current = 0;
        for i in 0.. byte_group.len() + 1
        {
            if i >= byte_group.len()
            {
                mask = 0x3F;        // Take the 6 less significant bits (0b00111111)
                current = byte_group[i-1] & mask;
                result.push(current);
                break;
            }

            let byte = byte_group[i];
            current |= byte >> count;
            current += reminder;

            mask = 2_u8.pow(count.into()) - 1;

            reminder = byte & mask;                     // Extracts the not-taken bits
            reminder = reminder << (limit - count) - 2; // 0b00xxxxxx

            count = (count + 2) % limit;                // updates the count

            result.push(current);
            current = current ^ current;
        }
    }
    print_vector(&text_as_bytes);
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
        1 => translate_upper(value),
        2 => translate_lower(value),
        3 => translate_number(value),
        4 => translate_plus(),
        5 => translate_slash(),
        _ => 0x0 as char
    };
}

fn select_group(value: u8) -> i32
{
    return if value <= 25
    {
        1
    } else if value >= 26 && value <= 51
    {
        2
    } else if value >= 51 && value <= 61
    {
        3
    } else if value == 62
    {
        4
    } else {
        5
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