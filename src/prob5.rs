//Smallest Multiple
//What is the smallest positive number evenly divisible by all the numbers from 1 to 20?
fn gcd(mut n: u64, mut m: u64) -> u64 {
    assert!(n != 0 && m != 0);
    while m != 0 {
        if m < n {
            let t = m;
            m = n;
            n = t;
        }
        m = m % n;
    }
    n
}

fn lcm(n : u64, m : u64) -> u64 {
    (n * m) / gcd(n, m)
}

fn main() {
    let mut n = 1;
    for i in 1..20 {
        n = lcm(n, i);
    }
    println!("The smallest multiple is {}.", n);
}
