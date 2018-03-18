//Summation of Primes
//find the sum of all the primes below two million
mod primes;

fn main() {
    let primes = primes::prime_range(1, 2000000);
    let sum : u64 = primes.iter().sum();
    println!("Sum of primes is {}.", sum);
}
