fn main() {
    for m in 1..100 {
        for n in 1..100{
            if m * m + m * n == 500 && m*m > n*n {
                let a: u64 = m*m - n*n;
                let b: u64 = 2*m*n;
                let c: u64 = m*m+n*n;
                println!("{}", a*b*c);
            }
        }
    }
}