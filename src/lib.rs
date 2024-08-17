use num_integer::sqrt;
use num_traits::One;
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