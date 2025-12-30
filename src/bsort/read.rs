use std::io;

pub fn function() -> u32
{
    let mut input = String::new();
    io::stdin().read_line(&mut input)
        .expect("Failed to read line");

    let number: u32 = input.trim().parse()
        .expect("Input not an integer");
    number
}

