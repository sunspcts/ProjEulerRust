use proj_euler_rust::rev_int;

fn main() {
    let mut highest_palindrome = 0;
    for a in 100..1000 {
        for b in 100..1000 {
            let prod: u64 = a * b;
            if prod > highest_palindrome {
                if prod == rev_int(prod) {
                    highest_palindrome = prod
                }
            }
        }
    }
    println!("{0}", highest_palindrome)
}