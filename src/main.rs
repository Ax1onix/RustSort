mod bsort;
mod binsrch;

fn main()
{
    println!("Bin Search or Bubble Sort?\n  1 = BubbleSort; 2 = BinSearch");
    
    let x = bsort::read::function();
    match x 
    {
        1=>bsort::function(),
        2=>binsrch::function(),
        _=>println!("Not an option!") 
    }
}
