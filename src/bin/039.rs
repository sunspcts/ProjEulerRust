use num_integer::gcd;

fn main() {
    let p: u64 = 1000;
    let mut count:Vec<u64> = vec![0; (p+1) as usize];
    let mut m = 0;
    while 2*m*m < p {
        m += 1;
        let mut n: u64 = 0;
        while n < m {
            n += 1;
            if m % 2 == 1 && n % 2 == 1 {continue};
            if gcd(m, n) > 1 {continue};
            let mut k = 1;
            loop {
                let a = k * (m*m - n*n);
                let b = k * 2 * m * n;
                let c = k * (m*m + n*n);
                let perimeter = a+b+c;
                if perimeter > p {break};
                count[perimeter as usize] += 1;
                k += 1
            }

        }

    }
    let mut max_p = 0;
    let mut max_sols = 0;
    for (i,j) in count.into_iter().enumerate() {
        if j > max_sols {
            max_sols = j; max_p = i;
        }
    }
    println!("{}", max_p)
}