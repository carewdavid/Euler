//Largest Prime Factor:
//What is the larges prime factor of the number 600851475143?
mod primes;

fn main() {
    let num = 600851475143;
    let mut i = 2;
    let step = 1;

    while i < (f64::sqrt(num as f64) as usize) {
        if num % i == 0 {
            if primes::is_prime(i) {
                println!("Prime factor {}.", i);
            }
                //num = num / i;
            if primes::is_prime(num / i) {
                println!("Prime factor {}.", num / i);
            }
        }
        i += step;
    }
}
