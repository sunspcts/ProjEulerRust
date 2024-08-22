use::proj_euler_rust::prime_factorization;

fn totient(n: u64) -> u64 {
    let mut prime_facs = prime_factorization(n);
    prime_facs.dedup(); 
    let prime_facs_f64:Vec<f64> = prime_facs.into_iter().map(|x| x as f64).collect();
    let mut phi = n as f64;
    for i in prime_facs_f64 {
        phi *= 1.0 - i.recip();
    }
    phi as u64
}

fn main() {
    let mut max:f64 = 0.0;
    let mut max_n:u64 = 0;
    let mut n = 2;
    use std::time::Instant;
    let now = Instant::now();

    loop {
        if n > 1000000 {break}
        let totient = totient(n);
        let n_over_phi = n as f64 / totient as f64;
        if n_over_phi > max {max = n_over_phi; max_n = n; println!("{}", n)}
        n += max_n
    }
    
    let elapsed = now.elapsed();
    println!("Elapsed: {:.2?}", elapsed);
}