use::proj_euler_rust::get_all_divisors;
use proj_euler_rust::Triangle;

fn main() {
    let iter = Triangle::default()
        .map(|n| NumberWithDivisors{ number: n, divisors: get_all_divisors(n) })
        .skip_while(|x| x.divisors.len() <= 500 );

    for n in iter {
        println!("{} -> {}", n.number, n.divisors.len());
        break
    }
}
struct NumberWithDivisors {
    number: u64,
    divisors: Vec<u64>,
}

