//Largest Product in a Series
//Find the thirteen adjacent digits in the 1000-digit number that have the
//greatest product.
use std::env;
use std::fs::File;
use std::io::prelude::*;


fn lookahead(window : &[u64]) -> Option<usize> {
    window.iter().position(|&x| x == 0)
}
    

fn main () {
    //read file into vec of u64s
    let digits : Vec<u64>;
    let mut f = File::open(env::args().last().unwrap()).expect("File not found");
    let mut chars = Vec::new();
    f.read_to_end(&mut chars).unwrap();
    digits = chars.iter().map(|&x| (x as char).to_digit(10).unwrap_or(0) as u64).collect();

    //look for product
    let mut max = 0;
    let mut i = 0;
    let window_len = 13;
    while i < digits.len() {
        let range = if i + window_len < digits.len() { i + window_len } else { digits.len() };
        match lookahead(&digits[i..range]) {
            Some(p) => i = i + p + 1,
            None    => { let product = (&digits[i..range]).iter().product();
                         if product > max { max = product}
                         i += 1
            }
        }
    }
    println!("The largest product is {}.", max);
}






