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
    println!("Please type the mode of sorting:\n 1. Sort min - max\n 2. Sort max-min");
    let mut choic:u16 = read_input() as u16;
    if choic == 1 
    {
        'outer:for _i in 0..numbers.len()
        {
            let mut boxy = 0; let mut boly = 0;
            for x in 0..numbers.len()-1
            {
               if numbers[x]>numbers[x+1]
                {
                    boxy = numbers[x];
                    numbers[x] = numbers[x+1];
                    numbers[x+1] = boxy;
                    boly = 1;
               }
            }
            if boly == 0 {
                break 'outer;
            }
        }
    }
    else if choic == 2
    {
        'outer: for _i in 0..numbers.len()
        {
            let mut boxy = 0; let mut boly = 0;
            for x in 0..numbers.len()-1
            {
                if numbers[x]<numbers[x+1]
                {
                    boxy = numbers[x+1];
                    numbers[x+1] = numbers[x];
                    numbers[x] = boxy;
                }
            }
            if boly == 0 {
                break  'outer;
            }
        }
    }
    else
    {
        println!("THAT WASN`T A CHOICE, SHWEINHUND!")
    }
    for i in 0..numbers.len()
    {
        print!(" {}",numbers[i]);
    }
}
