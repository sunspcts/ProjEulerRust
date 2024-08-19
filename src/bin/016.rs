use num::BigUint;

fn main() {
    let a: BigUint = BigUint::from(2 as u64);
    let b = a.pow(1000);
    let binding = b.to_string();
    let c = binding.chars().map(|x| x.to_digit(10).unwrap());
    let mut s:u64 = 0;
    for x in c {
        s += x as u64
    }
    println!("{}", s)
}