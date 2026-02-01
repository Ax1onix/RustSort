mod bsort;   //Importin
mod binsrch; //      All the neccessary libraries
use std::io;
use std::collections::HashMap;

pub fn function() -> String {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("FAIL READING");
    input
}

fn merge_sort(arr: &mut [i32]) {
    if arr.len() <= 1 { return; }
    
    let mid = arr.len() / 2;
    let mut left = arr[..mid].to_vec();
    let mut right = arr[mid..].to_vec();
    
    merge_sort(&mut left);
    merge_sort(&mut right);
    
    let mut i = 0;
    let mut j = 0;
    let mut k = 0;
    
    while i < left.len() && j < right.len() {
        if left[i] <= right[j] {
            arr[k] = left[i];
            i += 1;
        } else {
            arr[k] = right[j];
            j += 1;
        }
        k += 1;
    }
    
    while i < left.len() {
        arr[k] = left[i];
        i += 1; k += 1;
    }
    while j < right.len() {
        arr[k] = right[j];
        j += 1; k += 1;
    }
}


fn main()
{
    println!("Bin Search or Bubble Sort?\n  1 = BubbleSort; 2 = BinSearch; 3 = MergeSort");
    
    let x = bsort::read::function();  // Reading the input of your choice
    match x 
    {
        1=>bsort::function(),
        2=>binsrch::function(),
        3=>{
            println!("Write me a list of numbers, separated by whitespace");
            let mut nums: Vec<i32> = function()
                .split_whitespace()
                .map(|s| s.parse().unwrap())
                .collect();

            merge_sort(&mut nums);

            println!("Heres the sorted array with merge:\n{:#?}", nums);
        },
        _=>println!("Not an option!") 
    }
}
