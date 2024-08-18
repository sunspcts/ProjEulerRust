use proj_euler_rust::eratosthenes;

fn main() {
    let mut s = 0;
    let limit = 2000000;
    let sieve = eratosthenes(limit);
    for prime in 2..limit {
        if !sieve[prime] {
            continue;
        }
        s += prime
    }
    println!("{}", s)
}