use num::{BigUint, ToPrimitive, Integer, Unsigned, One, NumCast, FromPrimitive};

fn main() {
    let a = 100 as u8;
    let f: BigUint = factorial(a);
    println!("{}", f);
    let binding = f.to_string();
    let c = binding.chars().map(|x| x.to_digit(10).unwrap());
    let mut s:u64 = 0;
    for x in c {
        s += x as u64
    }
    println!("{}", s)
}

fn factorial<N: Unsigned + Integer + ToPrimitive>(n: N) -> BigUint {
    let mut f:BigUint = One::one();
    let end: usize = NumCast::from(n).unwrap();
    for i in 1..(end + 1) {
        let bu: BigUint = FromPrimitive::from_usize(i).unwrap();
        f *= bu
    }
    f
}