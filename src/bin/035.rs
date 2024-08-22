use::proj_euler_rust::{eratosthenes, as_digit_vec};

use std::collections::VecDeque;

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

fn main() {
    let primes: Vec<u64> = get_primes(1000000);
    let circular_primes: Vec<u64> = vec![];
    for i in primes {
        let mut current = vec![i];
        let mut digit_vec = as_digit_vec(i).into_iter().collect::<VecDeque<_>>();
        digit_vec.rotate_right(1);
    }
}