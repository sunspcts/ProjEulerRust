use::proj_euler_rust::{factorial, as_digit_vec};

fn main() {
    let mut s: u64 = 0;
    for i in 3..100000 {
        let digit_vec = as_digit_vec(i);
        let digit_fac:u128 = digit_vec.into_iter().map(|x| factorial(x as u128)).sum();
        let digit_fac:u64 = digit_fac as u64;
        if digit_fac == i {
            s += i
        }
    }
    println!("{}", s)
}