const PRIMES:[u64; 8] = [2,3,5,7,11,13,17,19];

fn main() {
    let mut n: u64 = 1;
    use std::time::Instant;
    let now = Instant::now();
    for i in PRIMES {
        if n > 100000 {break}
        n *= i
    }

    let elapsed = now.elapsed();
    println!("{}", n);
    println!("Elapsed: {:.2?}", elapsed);
}