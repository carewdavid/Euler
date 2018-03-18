//10001st Prime
//What is the 10001st prime number?
mod primes;

fn main() {
    let mut found : u64 = 0;
    let mut prime = 0;
    let mut i = 1;

    while found < 10002 {
        if primes::is_prime(i) {
            prime = i;
            found += 1;
        }
        i += 1;
    }

    println!("The 10001st prime number is {}.", prime);
}
