use num_integer::sqrt;
use num::One;
use std::{mem, ops::Add};

pub struct Fibonacci<T> {
    curr: T,
    next: T,
}
impl<T> Default for Fibonacci<T>
where T: One, {
    fn default() -> Self {

        Self::new()
    }
}

impl<T: One> Fibonacci<T> {
    pub fn new() -> Fibonacci<T> {
        Fibonacci::with_init(One::one(), One::one())
    }
    pub fn with_init(a0: T, a1: T) -> Fibonacci<T> {
        Fibonacci {
            curr: a0,
            next: a1,
        }
    }
}

impl<T: Add<T, Output = T> + Clone> Iterator for Fibonacci<T> {
    type Item = T;
    fn next(&mut self) -> Option<T> {
        let new_next = self.curr.clone() + self.next.clone();
        let new_current = mem::replace(&mut self.next, new_next);
        let retval = mem::replace(&mut self.curr, new_current);
        Some(retval)
    }
}

pub struct Triangle {
    idx: u64,
    prev: u64,
}

impl Iterator for Triangle {
    type Item = u64;
    fn next(&mut self) -> Option<Self::Item> {
        self.idx += 1;
        let result = self.prev + self.idx;
        self.prev = result;
        Some(result)
    }
}

impl Default for Triangle {
    fn default() -> Self { Triangle {idx:0, prev: 0} }
}

pub struct Collatz {
    pub curr: u64,
}

impl Iterator for Collatz {
    type Item = u64;
    fn next(&mut self) -> Option<Self::Item> {
        let r;
        if self.curr % 2 == 0 {
            r = self.curr / 2;
        }
        else if self.curr == 1 {
            return None
        }
        else {
            r = self.curr * 3 + 1
        }
        self.curr = r;
        Some(r)
    }
}

pub fn simple_is_prime(n: u64) -> bool {
    for x in 2..(sqrt(n) + 1) {
        if n % x == 0 {return false}
    }
    true
}

pub fn largest_prime_factor(n: u64) -> u64 {
    let mut curr: u64 = n;
    let mut factors: Vec<u64> = vec![];
    loop {
        let result = find_lowest_factor(curr);
        factors.push(result[0]);
        if simple_is_prime(curr) == true {break}
        curr = result[1].clone();
    }
    factors.pop().expect("FUCK")
}

fn find_lowest_factor(n: u64) -> [u64; 2] {
    for x in 2..n {
        if n % x == 0 {return [x, n/x]}
    }
    return [n, 1]
}

pub fn prime_factorization(n: u64) -> Vec<u64> {
    let mut curr: u64 = n;
    let mut factors: Vec<u64> = vec![];
    loop {
        let result = find_lowest_factor(curr);
        factors.push(result[0]);
        if simple_is_prime(curr) == true {break}
        curr = result[1].clone();
    }
    factors
}

pub fn rev_string(str: &str) -> String {
    str.chars().rev().collect::<String>()
}

pub fn rev_int(arg: u64) -> u64 {
    let s = arg.to_string();
    rev_string(&s).parse::<u64>().unwrap()
}

pub fn nth_prime(n: u32) -> u64 {
    if n < 1 {
        return 0;
    }
    let x = if n <= 10 { 10.0 } else { n as f64 };
    let limit: usize = (x * (x * (x).ln()).ln()).ceil() as usize;
    
    let mut count = 0;
    let sieve = eratosthenes(limit);

    for prime in 2..limit {
        if !sieve[prime] {
            continue;
        }
        count += 1;
        if count == n {
            return prime as u64;
        }
    }
    0
}

pub fn eratosthenes(limit: usize) -> Vec<bool>{
    let mut sieve: Vec<bool> = vec![true; limit];
    sieve[0] = false; sieve[1] = false;
    for index in 2..limit {
        if !sieve[index] {
            continue;
        }
        for multiple in ((index.pow(2))..limit).step_by(index) {
            sieve[multiple] = false;
        }
    }
    sieve
}

pub fn get_all_divisors(n: u64) -> Vec<u64> {
    let mut divisors:Vec<u64> = vec![];
    if n == 1 {divisors.push(1)}
    else {
        let prime_factors = prime_factorization(n);
        divisors.push(1);
        let mut last_prime:u64 = 0;
        let mut factor:u64 = 0;
        let mut slice_len:u64 = 0;
        for prime in prime_factors {
            if last_prime != prime {
                slice_len = divisors.len() as u64;
                factor = prime
            } else {
                factor *= prime
            }
            for i in 0..slice_len {
                divisors.push(divisors[i as usize] * factor)
            }
            last_prime = prime
        }
    }
    divisors
}

pub fn factorial(num: u128) -> u128 {
    (1..=num).product()
}

pub fn as_digit_vec(n:u64) -> Vec<u64> {
    n.to_string().chars().map(|x| x.to_digit(10).unwrap() as u64).collect()
} 