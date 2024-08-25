use proj_euler_rust::simple_is_prime;

fn main() {
    let mut s = 0;
    let mut branches: Vec<u64> = vec![2, 3, 5, 7];
    branches = add_right(&branches);
    while branches.len() != 0 {
        for b in &branches {
            if is_left_truncatable(*b) {
                s += *b;
            }
        }
    branches = add_right(&branches);    
    }
    println!("{}", s)
}


fn add_right(v: &Vec<u64>) -> Vec<u64> {
    let digits: [u64; 4] = [1, 3, 7, 9];
    let mut w = Vec::new();
    for branch in v {
        for d in digits.iter() {
            let candidate = branch * 10 + d;
            if simple_is_prime(candidate) {
                w.push(candidate)
            }
        }
    }
    w
}

fn trunc_left(n: u64) -> u64 {
    let zeroes = (n as f32).log(10f32) as u32;
    n % 10u64.pow(zeroes)
}

fn is_left_truncatable(n: u64) -> bool {
    let mut nc = trunc_left(n);
    while nc > 0 {
        if !simple_is_prime(nc) {return false}
        nc = trunc_left(nc);
    }
    true
}