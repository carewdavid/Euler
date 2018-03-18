//Largest Product in a Series
//Find the thirteen adjacent digits in the 1000-digit number that have the
//greatest product.
use std::env;
use std::fs::File;
use std::io::prelude::*;

//Won't be needed
/*
fn pare(mut v : Vec<u64>, window : u64) -> Vec<u64> {
    if window > v.len() { panic!(); }

    let mut i = 0;

    while i < v.len() {
        if v[i] == 0 {
            let mut j  = 0;
            if i - window > 0 { j = i - window}
            for _ in 0..j {
                v.remove(j);
            }

            let mut k = i + window;
            if i + window > v.len() { k = v.len()}
            for _ in i..k {
                v.remove(i);
            }

            i = 0;
        }
        i += 1;
    }

    v
}
*/

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
    /*
    for byte in f.bytes() {
        if byte.is_ok() {
            let c = byte.unwrap() as char;//Read an ascii digit from the file
            println!("{}", c);
            digits.push(c.to_digit(10).unwrap());
            
        }
    }
    */

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






