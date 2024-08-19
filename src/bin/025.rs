use num::BigInt;
use proj_euler_rust::Fibonacci;
fn main() {
    println!("{0}", compute(1000))
}
fn compute(num_digits: usize) -> usize {
    for (i, item) in Fibonacci::<BigInt>::new().enumerate() {
        if item.to_string().chars().count() >= num_digits {
            println!("{}", item);
            return i+1
        }
    }
    0
}