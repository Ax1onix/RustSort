use std::io;
fn read_input() -> u32
{
    let mut input = String::new();
    io::stdin().read_line(&mut input)
        .expect("Failed to read line");

    let number: u32 = input.trim().parse()
        .expect("Input not an integer");
    number
}
fn main()
{
    let mut n: u32 = read_input();
    let num: usize = n as usize;

    let mut numbers:Vec<u32> = vec![0u32;num];

    for i in 0..numbers.len()
    {
        numbers[i] = read_input();
    }

    for _i in 0..numbers.len()
    {
        let mut boxy = 0;
        for x in 0..numbers.len()-1
        {
            if numbers[x]>numbers[x+1]
            {
                boxy = numbers[x];
                numbers[x] = numbers[x+1];
                numbers[x+1] = boxy;
            }
        }
    }

    for i in 0..numbers.len()
    {
        print!(" {}",numbers[i]);
    }
}
