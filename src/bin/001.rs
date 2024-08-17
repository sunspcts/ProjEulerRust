fn main() {
    println!("{}", compute(1000).to_string())
}

fn compute(bound: u32) -> u32 {
    (1..bound).filter(|n: &u32| n % 3 == 0 || n % 5 == 0).sum()
}