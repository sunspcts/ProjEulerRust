use proj_euler_rust::Fibonacci;
use num_integer::Integer;
fn main() {
    println!("{0}", compute(4000000))
}
fn compute(bound: u32) -> u32 {
    Fibonacci::<u32>::new()
        .take_while(|&f| f < bound)
        .filter(|&f| f.is_even())
        .sum()
}