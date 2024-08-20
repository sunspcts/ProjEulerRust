use::proj_euler_rust::get_all_divisors;

fn sum_of_proper_divisors(n:u64) -> u64 {
    let divisors = get_all_divisors(n);
    let mut s = 0;
    for i in divisors {
        s += i
    }
    s - n
}

fn is_abundant(n:u64) -> bool {
    let s = sum_of_proper_divisors(n);
    s > n
}

fn main() {
    let mut abundant:Vec<u64> = vec![];
    for i in 1..28124 {
        if is_abundant(i) {
            abundant.push(i)
        }
    }
    let mut is_abundant_sum: Vec<bool> = vec![false; 28124];
    for i in &abundant {
        for j in &abundant {
            let s = i + j;
            if s >= is_abundant_sum.len() as u64 {
                break
            }
            is_abundant_sum[s as usize] = true
        }
    }
    let mut s: u64 = 0;
    for n in 1..is_abundant_sum.len() {
        if is_abundant_sum[n] {continue}
        s += n as u64
    }
    println!("{}", s)
}