use num::BigUint;

fn main() {
    let mut factorials:Vec<BigUint> = vec![BigUint::from(1 as u64)];
    for i in 1..=100 {
        factorials.push(factorial(i))
    }
    let mut s: u64 = 0;
    for n in 1..=100 {
        let n_fac = &factorials[n];
        for r in 1..n {
            let r_fac = &factorials[r];
            let combinations = n_fac / (r_fac * factorial((n - r) as u64));
            if combinations > BigUint::from(1000000 as u64) {
                s += 1;
            }
        }
    }
    println!("{}", s)
}

pub fn factorial(num: u64) -> BigUint {
    (1..=num).product()
}