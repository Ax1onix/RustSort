pub mod read;

pub fn function()
{
    println!("This is a simple sorting program!");
    println!("Please print rn the size of the list of numbers:  ");
    let n: u32 = read::function();
    let num: usize = n as usize;
    println!("Perfect, now, give me those numbers separated by Enter");
    let mut numbers:Vec<u32> = vec![0u32;num];

    for i in 0..numbers.len()
    {
        numbers[i] = read::function();
    }
    println!("Please type the mode of sorting:\n 1. Sort min - max\n 2. Sort max-min");
    let choic:u16 = read::function() as u16;
    if choic == 1 
    {
        for i in 0..numbers.len()
        {
            let mut boxy;
            for x in 0..numbers.len()-i-1
            {
               if numbers[x]>numbers[x+1]
                {
                    boxy = numbers[x];
                    numbers[x] = numbers[x+1];
                    numbers[x+1] = boxy;
               }
            }
        }
    }
    else if choic == 2
    {
        for i in 0..numbers.len()
        {
            let mut boxy;
            for x in 0..numbers.len()-i-1
            {
                if numbers[x]<numbers[x+1]
                {
                    boxy = numbers[x+1];
                    numbers[x+1] = numbers[x];
                    numbers[x] = boxy;
                }
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
