pub enum Group
{
    Upper,
    Lower,
    Num,
    Plus,
    Slash
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
