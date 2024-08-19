use::proj_euler_rust::get_all_divisors;
fn main() {
    let mut amicable_numbers:Vec<u64> = vec![];
    for i in 2..10000 {
        if amicable_numbers.contains(&i) {continue}
        if is_amicable(i) {
            amicable_numbers.push(i);
            amicable_numbers.push(sum_of_proper_divisors(i));
        }
    }
    let mut s = 0;
    for i in amicable_numbers {
        s += i
    }
    println!("{}", s)
}

fn sum_of_proper_divisors(n:u64) -> u64 {
    let divisors = get_all_divisors(n);
    let mut s = 0;
    for i in divisors {
        s += i
    }
    s - n
}
fn is_amicable(n:u64) -> bool {
    let m = sum_of_proper_divisors(n);
    if m == n {return false};
    if sum_of_proper_divisors(m) != n {return false};
    true
}