use::proj_euler_rust::{eratosthenes, as_digit_vec};

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
    let primes = get_primes(1000000);
    let mut circ_primes:Vec<u64> = vec![];
    for i in &primes {
        if circ_primes.contains(&i) {continue}
        circ_primes.append(&mut circ(*i, &primes))
    }
    println!("{:?}", circ_primes);
    println!("{}", circ_primes.len())
}
fn circ(v:u64, primes: &Vec<u64>) -> Vec<u64> {
    let mut digit_vec = as_digit_vec(v);
    let mut current:Vec<u64> = vec![];
    loop {
        digit_vec.rotate_right(1);
        let x = concat(&digit_vec);
        if !primes.contains(&x) {break}
        current.push(x);
        if x == v {
            return current
        }
    }
    vec![]
}

fn concat(vec: &[u64]) -> u64 {
    vec.iter().fold(0, |acc, elem| acc * 10 + elem)
}