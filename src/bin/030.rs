fn main() {
    let mut s = 0;
    for n in 2..400000 {
        if n == digit_fifth_power(n) {
            s += n;
        }
    }
    println!("{}", s)
}
fn digit_fifth_power(n: u64) -> u64 {
    let digit_vec:Vec<u64> = n.to_string().chars().map(|x| x.to_digit(10).unwrap() as u64).collect();
    let mut s = 0;
    for digit in digit_vec {
        s += digit.pow(5);
    }
    s
}