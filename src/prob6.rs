//Sum Square difference
//Find the difference between the sum of the squares of the first one hundred
//natural numbers and the square of the sum
fn main () {
    let mut sq = 0;
    let mut sum = 0;

    for i in 1..101 {
        sq += i*i;
        sum += i;
    }

    println!("The difference is {}.", (sum * sum) - sq);
}
