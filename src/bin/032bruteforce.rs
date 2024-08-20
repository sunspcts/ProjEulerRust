fn as_digit_vec(n:u64) -> Vec<u64> {
    n.to_string().chars().map(|x| x.to_digit(10).unwrap() as u64).collect()
} 
fn main() {
    let mut pandigital_products:Vec<u64> = vec![];
    for a in 1..5000 {
        for b in 1..5000 {
            let mut digits = as_digit_vec(a * b);
            digits.extend(as_digit_vec(a));
            digits.extend(as_digit_vec(b));
            if digits.len() == 9 && !digits.contains(&0) {
                digits.sort(); digits.dedup();
                if digits.len() == 9 {
                    pandigital_products.push(a * b);
                }
            } 
        }
    }
    pandigital_products.sort(); pandigital_products.dedup();
    println!("{:?}", pandigital_products.into_iter().sum::<u64>())
}
