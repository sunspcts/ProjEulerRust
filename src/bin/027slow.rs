use::proj_euler_rust::simple_is_prime;
use::proj_euler_rust::eratosthenes;
fn main() {
    let primes_below_1k = get_primes(1000);
    let mut max_c = 0;
    let mut max_ab = 0;
    for a in (-999..1000).step_by(2) {
        for b in &primes_below_1k {
            let c = count(a, *b as i64);
            if c > max_c {
                max_c = c; 
                max_ab = a * *b as i64;
            }
        }
    }
    println!("{}", max_ab)
}

fn count(a: i64, b: i64) -> u64 {
    let mut n = 0;
    loop {
        let s = n * n + a * n + b;
        if s < 0 {
            return n as u64
        }
        if !simple_is_prime(s as u64) {
            return n as u64
        }
        n += 1
    }
}
fn get_primes(limit: usize) -> Vec<u64> {
    let primes_below_limit_bool = eratosthenes(limit);
    let mut primes_below_limit:Vec<u64> = vec![];
    for (i,t) in primes_below_limit_bool.into_iter().enumerate() {
        if t {
            primes_below_limit.push(i as u64)
        }
    }
    primes_below_limit
}