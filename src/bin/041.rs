use itertools::Itertools;
use proj_euler_rust::simple_is_prime;

fn main() {
    let perms: itertools::Permutations<std::ops::Range<u64>> = (1..8).permutations(7);
    let mut pandigitals: Vec<u64> = vec![];
    for i in perms {
        let mut n = 0;
        let mut k: u32 = 0;
        for j in i {
            n += 10_u64.pow(k) * j;
            k += 1
        }
        pandigitals.push(n)
    }
    pandigitals.sort();
    pandigitals.reverse();
    for i in pandigitals {
        if simple_is_prime(i) {
            println!("{}", i); break
        }
    }
}