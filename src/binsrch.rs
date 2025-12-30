use crate::bsort::read;  //Making a shorter access to a read input function

pub fn function()
{
    println!("Hey, this is a simple binary search program!");
    println!("In order for it to work, you need to give me a list of sorted numbers");
    println!("Is it sorted? 1=Y   2=N");
    //Reading the answer of the user
    let choice = read::function();
    match choice 
    {
        1=>println!("Then we can continue"),
        2=>println!("Then sort it using our BubbleSort, before going here!"),
        _=> println!("Not an option!")
    }
    if choice == 2
    {
        std::process::exit(0);
    }
    // reading the numbers in the list and putting them in the vector
    println!("First, tell me the size of the list:");

    let n: u32 = read::function(); 
    let num: usize = n as usize;

    println!("Perfect, now, give me those numbers separated by Enter");

    let mut numbers:Vec<u32> = vec![0u32;num]; // <-- the vector

    for i in 0..numbers.len()
    {
        numbers[i] = read::function();
    }

    println!("Now finally, tell me the number, that you want to find the position of in the list:");
    let target = read::function();

    let listlen: u32 = numbers.len() as u32;  //Putting the min and max positions in the list as mn and mx
    let mut mn:u32 = 0; let mut mx:u32 = listlen-1 as u32; 
    loop  //A loop for the bin search
    {   
        let avg:usize = {
            if n%2 == 0{
                let avg = (mn+mx)/2;
                let avg:usize = avg as usize;
                avg
    
            }else{
                let avg = (mn+mx)/2;
                let avg:usize = avg as usize;
                avg
            }
        };
        println!("avg = {} and a number of avg is {}\n mx={};mn={}",avg, numbers[avg],mx,mn);
        if numbers[avg] == target  // A god forsaken if chain which took me a whole bloody hour >:((
        {
            println!("The results are in, gentelmen! The position of {} is {}", target, avg+1);
            break;
        }
        else if numbers[avg] < target
        {
            mn = avg as u32+1;
        }
        else if numbers[avg] > target
        {
            mx = avg as u32;
        }
    }
}
