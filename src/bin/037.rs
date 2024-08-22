use::proj_euler_rust::eratosthenes;

fn main() {
    let mut primes:Vec<u64> = vec![];
    for (i, j) in eratosthenes(1000000).into_iter().enumerate() {
        if j {primes.push(i as u64)}
    }
    let mut truncatable_primes:Vec<u64> = vec![];
    for i in &primes {
        if *i < 10 {continue}
        if is_trunc_prime(*i, &primes) {
            truncatable_primes.push(*i);
        }
        if truncatable_primes.len() == 10 {
            break
        }
    }
    println!("{:?}", truncatable_primes)

}

fn is_trunc_prime(n: u64, primes:&Vec<u64>) -> bool {
    let mut right = n;
    let mut left = n;
    while right != 0 {
        if !primes.contains(&right) {return false}
        right = trunc_right(right);
    }
    while left != 0 {
        if !primes.contains(&left) {return false}
        left = trunc_left(left);
    }
    true
}
fn trunc_right(n: u64) -> u64 {
    n / 10
}

fn trunc_left(n: u64) -> u64 {
    let mut factor = 1; 
    while factor * 10 < n {
        factor *= 10 
    }
    n % factor
}