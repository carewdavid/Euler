//Even Fibonacci numbers:
//By considering the terms in the Fibonacci sequence whose
//values do not exceed four million, find the sum of the
//even numbered terms.
fn fibonacci(n : u64) -> u64 {
    let mut fib1 : u64 = 1;
    let mut fib2 : u64 = 2;
    let mut temp;

    if n <= 0 {
        panic!()
    }else if n == 1 {
        1
    }else {
        for _ in 2..n {
                temp = fib1;
                fib1 = fib2;
                fib2 = temp + fib1;
        }
        fib2
    }
}

#[test]
fn test_fib() {
    assert_eq!(fibonacci(1), 1);
    assert_eq!(fibonacci(2), 2);
    assert_eq!(fibonacci(3), 3);
    assert_eq!(fibonacci(10), 89);
}

fn main() {
    let mut sum = 0;
    let mut fib = 0;
    let mut counter = 1;
    while fib < 4000000 {
        fib = fibonacci(counter);
        if fib % 2 == 0 {
            sum += fib;
        }
        counter += 1;
    }
    println!("The sum is {}.", sum)
}

