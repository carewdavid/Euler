//Largest Palindrome Product
//Find the largest palindrome made from the product of two three digit numbers
fn palindrome(n : u32) -> bool {
    let s : String = n.to_string();
    let r : String = s.chars().rev().collect();
    s == r
}

#[test]
fn test_palindrome() {
    assert!(palindrome(1));
    assert!(palindrome(121));
    assert!(palindrome(5445));
    assert!(!palindrome(1234));
    assert!(!palindrome(12));
}

fn main() {
    let mut palindromes : Vec<u32> = Vec::new();
    for i in 100..1000 {
        for j in 100..1000 {
            let prod = i * j;
            if palindrome(prod) {
                palindromes.push(prod);
            }
        }
    }
    println!("Largest palindrome product is {}.", palindromes.iter().max().unwrap());
}

