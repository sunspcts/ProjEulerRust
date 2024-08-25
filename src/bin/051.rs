use::proj_euler_rust::eratosthenes;
use num::traits::cast::ToPrimitive;

fn main() {
    let primes: Vec<u64> = get_prime_array(10000);
    let mut primes_with_recurring_digits:Vec<u64> = vec![];
    for i in primes {
        let num = i;
        let mut digits = to_digits(num);
        digits.sort();
        let lendigs = digits.len();
        digits.dedup();
        if lendigs > digits.len() {
            primes_with_recurring_digits.push(i)
        }
    }
    
}

fn get_prime_array(limit: usize) -> Vec<u64> {
    let sieve = eratosthenes(limit);
    let mut primes = vec![];
    for (i, val) in sieve.into_iter().enumerate() {
        if val {
            primes.push(i as u64);
        }
    }
    primes
}       
fn to_digits(n:u64) -> Vec<u64> {
    let mut digits:Vec<u64> = vec![];
    for i in n.to_string().chars() {
        digits.push(i as u64)
    }
    digits
}

fn to_digit_histogram(n: u64) -> [u64; 10] {
    let mut hist = [0; 10];
    for d in to_digits(n) {
        hist[d.to_usize().unwrap()] += 1;
    }
    hist
}
