//Special Pythagorean Triplet
//There exists exactly one pythagorean triplet for which a + b + c = 1000
//Find the product abc
fn main() {
    for a in 1..1000 {
        for b in a..1000 {
            let c = 1000 - (a+b);
            if (a + b + c) == 1000 && (a*a) + (b*b) == (c*c){
                println!("Found triple. Product is {}", a*b*c);
                break;
            }
        }
    }
}
