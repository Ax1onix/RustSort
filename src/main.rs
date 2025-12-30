mod bsort;
fn main()
{
    println!("Bin Search or Bubble Sort?\n  1 = BinSearch; 2 = BubbleSort");
    
    let x = bsort::read::function();
    match x 
    {
        1=>bsort::function(),
        2=>println!("Not available rn!"),
        _=>println!("Not an option!") 
    }
}
