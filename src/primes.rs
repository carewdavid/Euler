//True if n is prime, false otherwise
pub fn is_prime(n : usize) -> bool {
    if n == 1 {return false;}//Special case since everything is divisible by 1
    if n < 10000 {
        let primes = primes(n as u64 + 1);
        match primes.iter().position(|&x| x == (n as u64)) {
            Some(_) => return true,
            None => return false,
        }
    }

    let limit = f64::ceil(f64::sqrt(n as f64)) as usize;
        
    for i in 2..limit + 1{
        if n % i == 0 {
            return false
        }
    }
    true
}

#[test]
fn test_is_prime(){
    assert_eq!(is_prime(1), false);
    assert_eq!(is_prime(2), true);
    assert_eq!(is_prime(19), true);
    assert_eq!(is_prime(20), false);
    assert_eq!(is_prime(25), false);
    assert_eq!(is_prime(769), true);
}

//Generate a list of prime numbers up to a limit n using the sieve of eratosthenes
fn primes(n : u64) -> Vec<u64> {
    let mut sieve : Vec<bool>  = Vec::with_capacity(n as usize);

    //Initialize sieve to all true.
    //There's probably a better way to do this.
    for _ in 0..n { sieve.push(true); }

    //Small values mess up the loop below, handle them separately
    if n == 2 {
        return vec![2];
    }

    let limit = n / 2;
    let mut step = 2;
    while step < limit {
        if sieve[step as usize] {
            let mut i = step * 2;
            while i < n {
                sieve[i as usize] = false;
                i += step;
            }
        }
        step += 1;
    }

    let mut result : Vec<u64> = Vec::with_capacity(limit as usize);
    for i in 2..n {
        if sieve[i as usize] {
            result.push(i);
        }
    }
    result
}

#[test]
fn test_primes(){
    assert_eq!(primes(10).len(), 4);
    assert_eq!(primes(10)[0], 2);
    assert_eq!(primes(10)[1], 3);
    assert_eq!(primes(10)[2], 5);
    assert_eq!(primes(10)[3], 7);
    assert_eq!(primes(20).len(), 8);
    assert_eq!(primes(20)[4], 11);
    assert_eq!(primes(20)[7], 19);
}


pub fn prime_range(lo : u64, hi : u64) -> Vec<u64> {
    primes(hi).into_iter().filter(|x| x >= &lo && x <= &hi).collect()
}

