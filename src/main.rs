use std::io;
fn read_input(n: &mut String) -> u32
{
    io::stdin().read_line(n).expect("Wrong input, Dummkopf!");   //Readin
    let num: u32 = n.trim().parse().expect("NOT AN INTEGRER, SCHWEIN!");
    num
}
fn main() {
    let mut n = String::new(); 
    println!("{}", read_input(&mut n)*2);
}
